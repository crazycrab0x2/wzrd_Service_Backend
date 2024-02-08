// use bitcoin::blockdata::block;
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use std::collections::BTreeMap;
use std::cell::RefCell;
use bip39::{Mnemonic, Language, MnemonicType}; 
use sha2::Sha256;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
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
    pub error: String,
    pub token: String,
    pub result: bool
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct CreateWalletResponse {
    pub error: String,
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
    pub error: String,
    pub token: String,
    pub balance: u64
}

#[derive(CandidType, Deserialize)]
pub struct SendRequest {
    pub token: String,
    pub destination_address: String,
    pub amount_in_e8s: u64,
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct SendResult {
    pub error: String,
    pub token: String,
    pub result: String
}

pub async fn create_wallet(network: BitcoinNetwork, key_name: String, params: CreateWalletParams) -> CreateWalletResponse {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckToken", (params.token,)).await;
    match user_validation {
        Err(_err) => {
            CreateWalletResponse {
                error: "Can't access ID service".to_string(),
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
                    error: "Invalid token".to_string(),
                    token: "".to_string(),
                    phrase: "".to_string(),
                    icp_address: "".to_string(),
                    eth_address: "".to_string(),
                    btc_address: "".to_string()
                }
            }
            else{
                let res = ic_cdk::call::<(), (Vec<u8>,)>(Principal::management_canister(), "raw_rand", ()).await;
                match res {
                    Ok((entropy,)) => {
                        let mnemonic = Mnemonic::from_entropy(&entropy[0..16], Language::English).unwrap();
                        let phrase = mnemonic.phrase().to_string();
                        let icp_address = icp_utils::get_icp_address(phrase.clone());
                        let btc_address = btc_utils::get_btc_address(network, key_name, phrase.to_string()).await;
                        let user_name = get_user_name(token.clone());
                        WALLET_STORE.with(|wallet_store| {
                            if wallet_store.borrow().get(&user_name).is_some(){
                                let wallet_info = wallet_store.borrow().get(&user_name).unwrap().clone();
                                CreateWalletResponse {
                                    error: "".to_string(),
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
                                wallet_store.borrow_mut().insert(get_user_name(token.clone()), new_wallet_info);
                                CreateWalletResponse {
                                    error: "".to_string(),
                                    token,
                                    phrase,
                                    icp_address,
                                    eth_address: "".to_string(),
                                    btc_address
                                }
                            }
                        })
                    }
                    Err((_, error)) => {
                        CreateWalletResponse {
                            error,
                            token,
                            phrase: "".to_string(),
                            icp_address: "".to_string(),
                            eth_address: "".to_string(),
                            btc_address: "".to_string()
                        }
                    }
                }
            }
        }
    }
}

pub async fn destroy_wallet(params: CreateWalletParams) -> DestoryWalletResponse {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckToken", (params.token,)).await;
    match user_validation {
        Err(_err) => {
            DestoryWalletResponse {
                error: "Can't access ID service".to_string(),
                token: "".to_string(),
                result: false
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
               DestoryWalletResponse {
                    error: "Invalid token".to_string(),
                    token,
                    result: false
                }
            }
            else{
                let user_name = get_user_name(token.clone());
                WALLET_STORE.with(|wallet_store| {
                    let res = wallet_store.borrow_mut().remove(&user_name);
                    if res.is_some() {
                        DestoryWalletResponse {
                            error: "".to_string(),
                            token,
                            result: true
                        }
                    }
                    else{
                        DestoryWalletResponse {
                            error: "No wallet exist".to_string(),
                            token,
                            result: false
                        }
                    }
                })
            }
        }
    }
}

pub async fn get_wallet_address(params: CreateWalletParams) -> CreateWalletResponse {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckToken", (params.token,)).await;
    match user_validation {
        Err(_err) => {
            CreateWalletResponse {
                error: "Can't access ID service".to_string(),
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
                    error: "Invalid token".to_string(),
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
                            error: "".to_string(),
                            token,
                            phrase: wallet_info.phrase,
                            icp_address: wallet_info.icp_address,
                            eth_address: wallet_info.eth_address,
                            btc_address: wallet_info.btc_address
                        }
                    }
                    else {
                        CreateWalletResponse {
                            error: "No wallet exist".to_string(),
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
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckToken", (params.token,)).await;
    match user_validation {
        Err(_err) => {
            BalanceResult {
                error: "Can't access ID service".to_string(),
                token: "".to_string(),
                balance: 0
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                BalanceResult {
                    error: "Invalid token".to_string(),
                    token,
                    balance: 0
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
                        error: "No wallet exist".to_string(),
                        token,
                        balance: 0
                    }
                }
                else {
                    let (error, balance) = icp_utils::get_icp_balance(address).await;
                    BalanceResult {
                        error,
                        token,
                        balance
                    }
                }
            }
        }
    }
} 

pub async fn send_icp(params: SendRequest) -> SendResult {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckToken", (params.token,)).await;
    match user_validation {
        Err(_err) => {
            SendResult {
                error: "Can't access ID service".to_string(),
                token: "".to_string(),
                result: "".to_string()
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                SendResult {
                    error: "Invalid token".to_string(),
                    token,
                    result: "".to_string()
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
                        error: "No wallet exist".to_string(),
                        token,
                        result: "".to_string()
                    }
                }
                else {
                    let (error, result) = icp_utils::send_icp(phrase, params.destination_address, params.amount_in_e8s).await;
                    SendResult {
                        error,
                        token,
                        result
                    }
                }
            }
        }
    }
}

pub async fn get_btc_balance(network: BitcoinNetwork, params: BalanceRequest) -> BalanceResult {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckToken", (params.token,)).await;
    match user_validation {
        Err(_err) => {
            BalanceResult {
                error: "Can't access ID service".to_string(),
                token: "".to_string(),
                balance: 0
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                BalanceResult {
                    error: "Invalid token".to_string(),
                    token,
                    balance: 0
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
                        error: "No wallet exist".to_string(),
                        token,
                        balance: 0
                    }
                }
                else {
                    let (error, balance) = btc_utils::get_btc_balance(network, address).await;
                    BalanceResult {
                        error,
                        token,
                        balance
                    }
                }
            }
        }
    }
}

pub async fn send_btc(network: BitcoinNetwork, key_name: String, params: SendRequest) -> SendResult {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckToken", (params.token,)).await;
    match user_validation {
        Err(_err) => {
            SendResult {
                error: "Can't access ID service".to_string(),
                token: "".to_string(),
                result: "".to_string()
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                SendResult {
                    error: "Invalid token".to_string(),
                    token,
                    result: "".to_string()
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
                        error: "No wallet exist".to_string(),
                        token,
                        result: "".to_string()
                    }
                }
                else {
                    let (error, result) = btc_utils::send_btc(network, key_name, phrase, params.destination_address, params.amount_in_e8s).await;
                    SendResult {
                        error,
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