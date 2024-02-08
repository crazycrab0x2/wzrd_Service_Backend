mod wm_utils;
mod btc_types;
mod btc_utils;
mod icp_utils;

use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use ic_cdk_macros::update;
use std::cell::{Cell, RefCell};

thread_local! {
    static NETWORK: Cell<BitcoinNetwork> = Cell::new(BitcoinNetwork::Testnet);
    static KEY_NAME: RefCell<String> = RefCell::new(String::from("test_key_1"));
}

#[update (name = "Create_Wallet")]
pub async fn create_wallet(params: wm_utils::CreateWalletParams) -> wm_utils::CreateWalletResponse {
    let network = NETWORK.with(|n| n.get());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    wm_utils::create_wallet(network, key_name, params).await
}

#[update (name = "Import_Wallet")]
pub async fn import_wallet(params: wm_utils::ImportWalletParams) -> wm_utils::CreateWalletResponse {
    let network = NETWORK.with(|n| n.get());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    wm_utils::import_wallet(network, key_name, params).await
}

#[update (name = "Destroy_Wallet")]
pub async fn destroy_wallet(params: wm_utils::CreateWalletParams) -> wm_utils::DestoryWalletResponse {
    wm_utils::destroy_wallet(params).await
}

#[update (name = "Get_Wallet_Address")]
pub async fn get_wallet_address(params: wm_utils::CreateWalletParams) -> wm_utils::CreateWalletResponse {
    wm_utils::get_wallet_address(params).await
}

#[update (name = "Get_BTC_Balance")]
pub async fn get_btc_balance(request: wm_utils::BalanceRequest) -> wm_utils::BalanceResult {
    let network = NETWORK.with(|n| n.get());
    wm_utils::get_btc_balance(network, request).await
}

#[update (name = "Send_BTC")]
pub async fn send_btc(request: wm_utils::SendRequest) -> wm_utils::SendResult {
    let network = NETWORK.with(|n| n.get());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    wm_utils::send_btc(network, key_name, request).await
}

#[update (name = "Get_ICP_Balance")]
pub async fn get_icp_balance(request: wm_utils::BalanceRequest) -> wm_utils::BalanceResult {
    wm_utils::get_icp_balance(request).await
}

#[update (name = "Send_ICP")]
pub async fn send_icp(request: wm_utils::SendRequest) -> wm_utils::SendResult {
    wm_utils::send_icp(request).await
}
