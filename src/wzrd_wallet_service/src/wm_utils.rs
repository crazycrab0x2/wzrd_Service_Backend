use bitcoin::blockdata::block;
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use std::{clone, collections::BTreeMap};
use std::cell::RefCell;
use bip39::{Mnemonic, Language, Seed, MnemonicType}; 
use sha2::{Digest, Sha256};
use ic_ledger_types::{AccountIdentifier, Subaccount, transfer, Tokens, Memo, TransferArgs, DEFAULT_FEE, AccountBalanceArgs, MAINNET_LEDGER_CANISTER_ID, account_balance};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use crate::{btc_utils, icp_utils};

#[derive(CandidType,Clone, Deserialize, Debug)]
pub struct WalletInfo {
    pub phrase: String,
    pub btc_address: String,
    pub icp_address: String,
    pub eth_address: String,
    pub public_key: String,
    pub private_key: String,
}

type WalletStore = BTreeMap<String, WalletInfo>; //(user_name => wallet info)

thread_local! {
    pub static WALLET_STORE: RefCell<WalletStore> = RefCell::default();
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct CreateWalletParams {
    pub token: String
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct DestoryWalletResponse {
    pub token: String,
    pub result: String
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct CreateWalletResponse {
    pub token: String,
    pub phrase: String,
    pub btc_address: String,
    pub icp_address: String,
    pub eth_address: String,
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct BalanceRequest {
    pub token: String
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct BalanceResult {
    pub token: String,
    pub balance: String
}

#[derive(CandidType, Deserialize)]
pub struct SendRequest {
    pub token: String,
    pub destination_address: String,
    pub amount_in_e8s: u64,
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct SendResult {
    pub token: String,
    pub result: String
}

pub async fn create_wallet(network: BitcoinNetwork, key_name: String, params: CreateWalletParams) -> CreateWalletResponse {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("urpxs-4aaaa-aaaap-qb6mq-cai").unwrap(), "CheckToken", (params.token,)).await;
    match user_validation {
        Err(_err) => {
            CreateWalletResponse {
                token: "".to_string(),
                phrase: "Can't access ID service".to_string(),
                icp_address: "".to_string(),
                eth_address: "".to_string(),
                btc_address: "".to_string()
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                CreateWalletResponse {
                    token: "".to_string(),
                    phrase: "Invalid token".to_string(),
                    icp_address: "".to_string(),
                    eth_address: "".to_string(),
                    btc_address: "".to_string()
                }
            }
            else{
                let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
                let phrase = mnemonic.phrase().to_string();
                // let word_list: Vec<String> = phrase.split_whitespace().map(|word| word.to_string()).collect();
                let icp_address = icp_utils::get_icp_address(phrase.clone());
                let btc_address = btc_utils::get_btc_address(network, key_name, phrase.to_string()).await;
                let user_name = get_user_name(token.clone());
                WALLET_STORE.with(|wallet_store| {
                    if wallet_store.borrow().get(&user_name).is_some(){
                        let wallet_info = wallet_store.borrow().get(&user_name).unwrap().clone();
                        CreateWalletResponse {
                            token,
                            phrase: wallet_info.phrase,
                            icp_address: wallet_info.icp_address,
                            eth_address: "".to_string(),
                            btc_address: wallet_info.btc_address
                        }
                    }
                    else{
                        let new_wallet_info = WalletInfo {
                            phrase: phrase.clone(),
                            btc_address: btc_address.clone(),
                            icp_address: icp_address.clone(),
                            eth_address: "".to_string(),
                            public_key: "".to_string(),
                            private_key: "".to_string()
                        };
                        wallet_store.borrow_mut().insert(get_user_name(token), new_wallet_info);
                        CreateWalletResponse {
                            token: "".to_string(),
                            phrase,
                            icp_address,
                            eth_address: "".to_string(),
                            btc_address
                        }
                    }
                })
            }
        }
    }
}

pub async fn destroy_wallet(params: CreateWalletParams) -> DestoryWalletResponse {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("urpxs-4aaaa-aaaap-qb6mq-cai").unwrap(), "CheckToken", (params.token,)).await;
    match user_validation {
        Err(_err) => {
            DestoryWalletResponse {
                token: "".to_string(),
                result: "Can't access ID service".to_string()
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                DestoryWalletResponse {
                    token,
                    result: "Invalid token".to_string()
                }
            }
            else{
                let user_name = get_user_name(token.clone());
                WALLET_STORE.with(|wallet_store| {
                    let res = wallet_store.borrow_mut().remove(&user_name);
                    if res.is_some() {
                        DestoryWalletResponse {
                            token,
                            result: "Success".to_string()
                        }
                    }
                    else{
                        DestoryWalletResponse {
                            token,
                            result: "No exist wallet".to_string()
                        }
                    }
                })
            }
        }
    }
}

pub async fn get_wallet_address(params: CreateWalletParams) -> CreateWalletResponse {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("urpxs-4aaaa-aaaap-qb6mq-cai").unwrap(), "CheckToken", (params.token,)).await;
    match user_validation {
        Err(_err) => {
            CreateWalletResponse {
                token: "".to_string(),
                phrase: "".to_string(),
                icp_address: "".to_string(),
                eth_address: "".to_string(),
                btc_address: "".to_string()
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                CreateWalletResponse {
                    token: "".to_string(),
                    phrase: "".to_string(),
                    icp_address: "".to_string(),
                    eth_address: "".to_string(),
                    btc_address: "".to_string()
                }
            }
            else{
                let user_name = get_user_name(token.clone());
                WALLET_STORE.with(|wallet_store| {
                    if wallet_store.borrow().get(&user_name).is_some(){
                        let wallet_info = wallet_store.borrow().get(&user_name).unwrap().clone();
                        CreateWalletResponse {
                            token,
                            phrase: wallet_info.phrase,
                            icp_address: wallet_info.icp_address,
                            eth_address: wallet_info.eth_address,
                            btc_address: wallet_info.btc_address
                        }
                    }
                    else {
                        CreateWalletResponse {
                            token,
                            phrase: "".to_string(),
                            icp_address: "".to_string(),
                            eth_address: "".to_string(),
                            btc_address: "".to_string()
                        }
                    }
                })
            }
        }
    }
}

pub async fn get_icp_balance(params: BalanceRequest) -> BalanceResult {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("urpxs-4aaaa-aaaap-qb6mq-cai").unwrap(), "CheckToken", (params.token,)).await;
    match user_validation {
        Err(_err) => {
            BalanceResult {
                token: "".to_string(),
                balance: "Can't access ID service".to_string()
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                BalanceResult {
                    token,
                    balance: "Invalid token".to_string()
                }
            }
            else{
                let user_name = get_user_name(token.clone());
                let mut address = "".to_string();
                WALLET_STORE.with(|wallet_store|{
                    if wallet_store.borrow().get(&user_name).is_some() {
                        address = wallet_store.borrow().get(&user_name).unwrap().clone().icp_address;
                    }
                });
                if address == "".to_string() {
                    BalanceResult {
                        token,
                        balance: "No exist wallet".to_string()
                    }
                }
                else {
                    let balance = icp_utils::get_icp_balance(address).await;
                    BalanceResult {
                        token,
                        balance
                    }
                }
            }
        }
    }
} 

pub async fn send_icp(params: SendRequest) -> SendResult {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("urpxs-4aaaa-aaaap-qb6mq-cai").unwrap(), "CheckToken", (params.token,)).await;
    match user_validation {
        Err(_err) => {
            SendResult {
                token: "".to_string(),
                result: "Can't access ID service".to_string()
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                SendResult {
                    token,
                    result: "Invalid token".to_string()
                }
            }
            else{
                let user_name = get_user_name(token.clone());
                let mut phrase = "".to_string();
                WALLET_STORE.with(|wallet_store|{
                    if wallet_store.borrow().get(&user_name).is_some() {
                        phrase = wallet_store.borrow().get(&user_name).unwrap().clone().phrase;
                    }
                });
                if phrase == "".to_string() {
                    SendResult {
                        token,
                        result: "No exist wallet".to_string()
                    }
                }
                else {
                    let result = icp_utils::send_icp(phrase, params.destination_address, params.amount_in_e8s).await;
                    SendResult {
                        token,
                        result
                    }
                }
            }
        }
    }
}

pub async fn get_btc_balance(network: BitcoinNetwork, params: BalanceRequest) -> BalanceResult {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("urpxs-4aaaa-aaaap-qb6mq-cai").unwrap(), "CheckToken", (params.token,)).await;
    match user_validation {
        Err(_err) => {
            BalanceResult {
                token: "".to_string(),
                balance: "Can't access ID service".to_string()
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                BalanceResult {
                    token,
                    balance: "Invalid token".to_string()
                }
            }
            else{
                let user_name = get_user_name(token.clone());
                let mut address = "".to_string();
                WALLET_STORE.with(|wallet_store|{
                    if wallet_store.borrow().get(&user_name).is_some() {
                        address = wallet_store.borrow().get(&user_name).unwrap().clone().btc_address;
                    }
                });
                if address == "".to_string() {
                    BalanceResult {
                        token,
                        balance: "No exist wallet".to_string()
                    }
                }
                else {
                    let balance = btc_utils::get_btc_balance(network, address).await;
                    BalanceResult {
                        token,
                        balance
                    }
                }
            }
        }
    }
}

pub async fn send_btc(network: BitcoinNetwork, key_name: String, params: SendRequest) -> SendResult {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("urpxs-4aaaa-aaaap-qb6mq-cai").unwrap(), "CheckToken", (params.token,)).await;
    match user_validation {
        Err(_err) => {
            SendResult {
                token: "".to_string(),
                result: "Can't access ID service".to_string()
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                SendResult {
                    token,
                    result: "Invalid token".to_string()
                }
            }
            else{
                let user_name = get_user_name(token.clone());
                let mut phrase = "".to_string();
                WALLET_STORE.with(|wallet_store|{
                    if wallet_store.borrow().get(&user_name).is_some() {
                        phrase = wallet_store.borrow().get(&user_name).unwrap().clone().phrase;
                    }
                });
                if phrase == "".to_string() {
                    SendResult {
                        token,
                        result: "No exist wallet".to_string()
                    }
                }
                else {
                    let result = btc_utils::send_btc(network, key_name, phrase, params.destination_address, params.amount_in_e8s).await;
                    SendResult {
                        token,
                        result
                    }
                }
            }
        }
    }
}

pub fn get_user_name(token: String) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"wzrd-secret-key").unwrap();
    let veri_claims;
    let result: Result<BTreeMap<String, String>, jwt::Error> = token.as_str().verify_with_key(&key);
    match result{
        Ok(okay_result) => veri_claims = okay_result,
        Err(_) => return "".to_string()
    };
    let user_name = &veri_claims["username"];
    user_name.clone()
 }