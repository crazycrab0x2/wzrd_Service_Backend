use ic_cdk::api::management_canister::bitcoin::{
    BitcoinNetwork, 
    GetUtxosResponse, 
    MillisatoshiPerByte, 
    Utxo, 
    GetBalanceRequest, 
    GetCurrentFeePercentilesRequest, 
    GetUtxosRequest, 
    Satoshi, 
    SendTransactionRequest
};
use bitcoin::{
    blockdata::{
        script::Builder, 
        witness::Witness
    },
    hashes::Hash,
    util::psbt::serialize::Serialize,
    Address, 
    AddressType, 
    EcdsaSighashType, 
    OutPoint, 
    Script, 
    Transaction, 
    TxIn, 
    TxOut, 
    Txid,
};
use sha2::Digest;
use std::str::FromStr;
use crate::btc_types::*;
use ic_cdk::{
    api::call::call_with_payment, 
    call
};
use candid::{CandidType, Deserialize, Principal};

const SIGN_WITH_ECDSA_COST_CYCLES: u64 = 10_000_000_000;
const SIG_HASH_TYPE: EcdsaSighashType = EcdsaSighashType::All;
const GET_BALANCE_COST_CYCLES: u64 = 100_000_000;
const GET_CURRENT_FEE_PERCENTILES_CYCLES: u64 = 100_000_000;
const GET_UTXOS_COST_CYCLES: u64 = 10_000_000_000;
const SEND_TRANSACTION_BASE_CYCLES: u64 = 5_000_000_000;
const SEND_TRANSACTION_PER_BYTE_CYCLES: u64 = 20_000_000;

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct BalanceRequest {
    pub token: String,
    pub address: String
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct BalanceResult {
    pub token: String,
    pub balance: String
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct SendRequest {
    pub token: String,
    pub destination_address: String,
    pub amount_in_satoshi: u64,
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct SendResult {
    pub token: String,
    pub tx_id: String
}

pub async fn get_btc_address (network: BitcoinNetwork, key_name: String, phrase: String) -> String {
    let derivation: Vec<Vec<u8>> = phrase.split_whitespace().map(|word| word.as_bytes().to_vec()).collect();
    let public_key = ecdsa_public_key(key_name, derivation).await;
    public_key_to_btc_address(network, &public_key)
}

pub async fn get_btc_balance(network: BitcoinNetwork, address: String) -> (String, u64) {
    let balance_res: Result<(Satoshi,), _> = call_with_payment(
        Principal::management_canister(),
        "bitcoin_get_balance",
        (GetBalanceRequest {
            address,
            network: network.into(),
            min_confirmations: None,
        },),
        GET_BALANCE_COST_CYCLES,
    )
    .await;
    match balance_res {
        Ok((balance,)) => ("".to_string(), balance),
        Err((_, error)) => (error, 0)
    }
}

pub async fn send_btc(
    network: BitcoinNetwork,
    key_name: String,
    phrase: String,
    dst_address: String,
    amount: Satoshi,
) -> (String, String) {
    let derivation_path: Vec<Vec<u8>> = phrase.split_whitespace().map(|word| word.as_bytes().to_vec()).collect();

    let fee_percentiles = get_current_fee_percentiles(network).await;

    let fee_per_byte = if fee_percentiles.is_empty() {
        2000
    } else {
        fee_percentiles[50]
    };

    let own_public_key = ecdsa_public_key(key_name.clone(), derivation_path.clone()).await;

    let own_address = public_key_to_btc_address(network, &own_public_key);

    let own_utxos = get_utxos(network, own_address.clone())
        .await
        .utxos;

    let own_address = Address::from_str(&own_address).unwrap();
    let dst_address = Address::from_str(&dst_address).unwrap();

    // Build the transaction that sends `amount` to the destination address.
    let transaction = build_transaction(
        &own_public_key,
        &own_address,
        &own_utxos,
        &dst_address,
        amount,
        fee_per_byte,
    )
    .await;

    // Sign the transaction.
    let signed_transaction = sign_transaction(
        &own_public_key,
        &own_address,
        transaction,
        key_name,
        derivation_path,
        sign_with_ecdsa,
    )
    .await;

    let signed_transaction_bytes = signed_transaction.serialize();

    send_transaction(network, signed_transaction_bytes).await;

    ("".to_string(), signed_transaction.txid().to_string())
}

pub async fn ecdsa_public_key(key_name: String, derivation_path: Vec<Vec<u8>>) -> Vec<u8> {
    let res: Result<(ECDSAPublicKeyReply,), _> = call(
        Principal::management_canister(),
        "ecdsa_public_key",
        (ECDSAPublicKey {
            canister_id: None,
            derivation_path,
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: key_name,
            },
        },),
    ).await;
    res.unwrap().0.public_key
}

fn public_key_to_btc_address(network: BitcoinNetwork, public_key: &[u8]) -> String {
    let result = ripemd160(&sha256(public_key));

    let prefix = match network {
        BitcoinNetwork::Testnet | BitcoinNetwork::Regtest => 0x6f,
        BitcoinNetwork::Mainnet => 0x00,
    };
    let mut data_with_prefix = vec![prefix];
    data_with_prefix.extend(result);

    let checksum = &sha256(&sha256(&data_with_prefix.clone()))[..4];

    let mut full_address = data_with_prefix;
    full_address.extend(checksum);

    bs58::encode(full_address).into_string()
}

fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = sha2::Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn ripemd160(data: &[u8]) -> Vec<u8> {
    let mut hasher = ripemd::Ripemd160::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

async fn build_transaction(
    own_public_key: &[u8],
    own_address: &Address,
    own_utxos: &[Utxo],
    dst_address: &Address,
    amount: Satoshi,
    fee_per_byte: MillisatoshiPerByte,
) -> Transaction {
    let mut total_fee = 0;
    loop {
        let transaction =
            build_transaction_with_fee(own_utxos, own_address, dst_address, amount, total_fee)
                .expect("Error building transaction.");

        // Sign the transaction. In this case, we only care about the size
        // of the signed transaction, so we use a mock signer here for efficiency.
        let signed_transaction = sign_transaction(
            own_public_key,
            own_address,
            transaction.clone(),
            String::from(""), // mock key name
            vec![],           // mock derivation path
            mock_signer,
        )
        .await;

        let signed_tx_bytes_len = signed_transaction.serialize().len() as u64;

        if (signed_tx_bytes_len * fee_per_byte) / 1000 == total_fee {
            return transaction;
        } else {
            total_fee = (signed_tx_bytes_len * fee_per_byte) / 1000;
        }
    }
}

fn build_transaction_with_fee(
    own_utxos: &[Utxo],
    own_address: &Address,
    dst_address: &Address,
    amount: u64,
    fee: u64,
) -> Result<Transaction, String> {
    const DUST_THRESHOLD: u64 = 1_000;

    let mut utxos_to_spend = vec![];
    let mut total_spent = 0;
    for utxo in own_utxos.iter().rev() {
        total_spent += utxo.value;
        utxos_to_spend.push(utxo);
        if total_spent >= amount + fee {
            break;
        }
    }

    if total_spent < amount + fee {
        return Err(format!(
            "Insufficient balance: {}, trying to transfer {} satoshi with fee {}",
            total_spent, amount, fee
        ));
    }

    let inputs: Vec<TxIn> = utxos_to_spend
        .into_iter()
        .map(|utxo| TxIn {
            previous_output: OutPoint {
                txid: Txid::from_hash(Hash::from_slice(&utxo.outpoint.txid).unwrap()),
                vout: utxo.outpoint.vout,
            },
            sequence: 0xffffffff,
            witness: Witness::new(),
            script_sig: Script::new(),
        })
        .collect();

    let mut outputs = vec![TxOut {
        script_pubkey: dst_address.script_pubkey(),
        value: amount,
    }];

    let remaining_amount = total_spent - amount - fee;

    if remaining_amount >= DUST_THRESHOLD {
        outputs.push(TxOut {
            script_pubkey: own_address.script_pubkey(),
            value: remaining_amount,
        });
    }

    Ok(Transaction {
        input: inputs,
        output: outputs,
        lock_time: 0,
        version: 1,
    })
}

async fn sign_transaction<SignFun, Fut>(
    own_public_key: &[u8],
    own_address: &Address,
    mut transaction: Transaction,
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
    signer: SignFun,
) -> Transaction
where
    SignFun: Fn(String, Vec<Vec<u8>>, Vec<u8>) -> Fut,
    Fut: std::future::Future<Output = Vec<u8>>,
{
    assert_eq!(
        own_address.address_type(),
        Some(AddressType::P2pkh),
        "This example supports signing p2pkh addresses only."
    );

    let txclone = transaction.clone();
    for (index, input) in transaction.input.iter_mut().enumerate() {
        let sighash =
            txclone.signature_hash(index, &own_address.script_pubkey(), SIG_HASH_TYPE.to_u32());

        let signature = signer(key_name.clone(), derivation_path.clone(), sighash.to_vec()).await;

        let der_signature = sec1_to_der(signature);

        let mut sig_with_hashtype = der_signature;
        sig_with_hashtype.push(SIG_HASH_TYPE.to_u32() as u8);
        input.script_sig = Builder::new()
            .push_slice(sig_with_hashtype.as_slice())
            .push_slice(own_public_key)
            .into_script();
        input.witness.clear();
    }

    transaction
}

// Converts a SEC1 ECDSA signature to the DER format.
fn sec1_to_der(sec1_signature: Vec<u8>) -> Vec<u8> {
    let r: Vec<u8> = if sec1_signature[0] & 0x80 != 0 {
        // r is negative. Prepend a zero byte.
        let mut tmp = vec![0x00];
        tmp.extend(sec1_signature[..32].to_vec());
        tmp
    } else {
        // r is positive.
        sec1_signature[..32].to_vec()
    };

    let s: Vec<u8> = if sec1_signature[32] & 0x80 != 0 {
        // s is negative. Prepend a zero byte.
        let mut tmp = vec![0x00];
        tmp.extend(sec1_signature[32..].to_vec());
        tmp
    } else {
        // s is positive.
        sec1_signature[32..].to_vec()
    };

    // Convert signature to DER.
    vec![
        vec![0x30, 4 + r.len() as u8 + s.len() as u8, 0x02, r.len() as u8],
        r,
        vec![0x02, s.len() as u8],
        s,
    ]
    .into_iter()
    .flatten()
    .collect()
}

async fn mock_signer(
    _key_name: String,
    _derivation_path: Vec<Vec<u8>>,
    _message_hash: Vec<u8>,
) -> Vec<u8> {
    //
    vec![255; 64]
}

pub async fn get_current_fee_percentiles(network: BitcoinNetwork) -> Vec<MillisatoshiPerByte> {
    let res: Result<(Vec<MillisatoshiPerByte>,), _> = call_with_payment(
        Principal::management_canister(),
        "bitcoin_get_current_fee_percentiles",
        (GetCurrentFeePercentilesRequest {
            network: network.into(),
        },),
        GET_CURRENT_FEE_PERCENTILES_CYCLES,
    )
    .await;
    res.unwrap().0
}

pub async fn get_utxos(network: BitcoinNetwork, address: String) -> GetUtxosResponse {
    let utxos_res: Result<(GetUtxosResponse,), _> = call_with_payment(
        Principal::management_canister(),
        "bitcoin_get_utxos",
        (GetUtxosRequest {
            address,
            network: network.into(),
            filter: None,
        },),
        GET_UTXOS_COST_CYCLES,
    )
    .await;
    utxos_res.unwrap().0
}

pub async fn sign_with_ecdsa(
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
    message_hash: Vec<u8>,
) -> Vec<u8> {
    let res: Result<(SignWithECDSAReply,), _> = call_with_payment(
        Principal::management_canister(),
        "sign_with_ecdsa",
        (SignWithECDSA {
            message_hash,
            derivation_path,
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: key_name,
            },
        },),
        SIGN_WITH_ECDSA_COST_CYCLES,
    )
    .await;

    res.unwrap().0.signature
}

pub async fn send_transaction(network: BitcoinNetwork, transaction: Vec<u8>) {
    let transaction_fee = SEND_TRANSACTION_BASE_CYCLES
        + (transaction.len() as u64) * SEND_TRANSACTION_PER_BYTE_CYCLES;

    let res: Result<(), _> = call_with_payment(
        Principal::management_canister(),
        "bitcoin_send_transaction",
        (SendTransactionRequest {
            network: network.into(),
            transaction,
        },),
        transaction_fee,
    )
    .await;

    res.unwrap();
}