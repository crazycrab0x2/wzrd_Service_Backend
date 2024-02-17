use ic_web3::{
    ethabi::ethereum_types::U256,
    ic::{
        get_eth_addr,
        KeyInfo,
    },
    transports::ICHttp,
    types::{
        Address, 
        TransactionParameters
    }, 
    Web3
};
use std::str::FromStr;

pub async fn get_evm_address(phrase: String, key_name: String) -> String {
    let derivation: Vec<Vec<u8>> = phrase.split_whitespace().map(|word| word.as_bytes().to_vec()).collect();
    let res = get_eth_addr(None, Some(derivation), key_name.to_string()).await;
    format!("0x{}", hex::encode(res.unwrap()))
}

pub async fn get_evm_balance(network: String, address: String) -> (u64, String) {
    let (rpc_end_point, _, _) = get_network_info(network.as_str());
    let w3 = match ICHttp::new(&rpc_end_point, None) {
        Ok(v) => { Web3::new(v) },
        Err(e) => { return (0, e.to_string()) },
    };
    let evm_address= &address[2..];
    let balance = w3.eth().balance(Address::from_str(evm_address).unwrap(), None,).await;
    match balance {
        Ok(bal) => (bal.as_u64(), "".to_string()),
        Err(err) => (0, err.to_string())
    }
}
  
pub async fn send_evm(network: String, phrase: String, to_add: String, amount: u64, key_name: String) -> (String, String) {
    // ecdsa key info
    let derivation_path: Vec<Vec<u8>> = phrase.split_whitespace().map(|word| word.as_bytes().to_vec()).collect();
    // let derivation_path = vec![params.user_name.as_bytes().to_vec()];
    let key_info = KeyInfo{ derivation_path: derivation_path.clone(), key_name: key_name.clone(), ecdsa_sign_cycles: None};

    // get canister eth address
    let from_addr = get_eth_addr(None, Some(derivation_path), key_name)
        .await
        .unwrap();
    // get canister the address tx count
    let (rpc_end_point, chain_id, gas_price) = get_network_info(network.as_str());

    let w3 = match ICHttp::new(&rpc_end_point, None) {
        Ok(v) => { Web3::new(v) },
        Err(e) => { return (e.to_string(), "".to_string()) },
    };

    let tx_catch = w3.eth()
        .transaction_count(from_addr, None)
        .await;
    match tx_catch {
        Ok(tx_count) => {
            // construct a transaction
            let address= &to_add[2..];
            let to_addr = Address::from_str(address).unwrap();
            let tx = TransactionParameters {
                to: Some(to_addr),
                nonce: Some(tx_count), 
                value: U256::from(amount),
                gas_price: Some(U256::from(gas_price)),
                gas: U256::from(21000),
                ..Default::default()
            };
            // sign the transaction and get serialized transaction + signature
            let signed_tx_res = w3.accounts()
                .sign_transaction(tx, hex::encode(from_addr), key_info, chain_id)
                .await;
            match signed_tx_res {
                Ok(signed_tx) => {
                    let tx_hash_res = w3.eth().send_raw_transaction(signed_tx.raw_transaction).await;
                    match tx_hash_res {
                        Ok(tx_hash) => ("".to_string(), hex::encode(tx_hash)),
                        Err(error) => (error.to_string(), "".to_string())
                    }
                },
                Err(error) => (error.to_string(), "".to_string())
            }
        },
        Err(error) => (error.to_string(), "".to_string())
    }
}
    
fn get_network_info(network: &str) -> (String, u64, u64) {
    match network {
      "ethereum" => ("https://ethereum-goerli.publicnode.com".to_string(), 5, 16000000000),
      "binance" => ("https://bsc-testnet.publicnode.com".to_string(), 97, 3000000000),
      "polygon" => ("https://polygon-mumbai-pokt.nodies.app".to_string(), 80001, 38000000000),
      &_ => ("None".to_string(), 0, 0)
    }
}