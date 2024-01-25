mod wm_utils;
mod btc_types;
mod btc_utils;
mod icp_utils;

use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use ic_cdk_macros::update;
use std::cell::{Cell, RefCell};


thread_local! {
    static NETWORK: Cell<BitcoinNetwork> = Cell::new(BitcoinNetwork::Mainnet);
    static KEY_NAME: RefCell<String> = RefCell::new(String::from("key_1"));
}

#[update (name = "Create_Wallet")]
pub async fn create_wallet(params: wm_utils::CreateWalletParams) -> wm_utils::CreateWalletResponse {
    let network = NETWORK.with(|n| n.get());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    wm_utils::create_wallet(network, key_name, params).await
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
pub async fn get_btc_balance(request: btc_utils::BalanceRequest) -> btc_utils::BalanceResult {
    let network = NETWORK.with(|n| n.get());
    btc_utils::get_balance(network, request).await
}

#[update (name = "Send_BTC")]
pub async fn send_btc(request: btc_utils::SendRequest) -> String {
    let network = NETWORK.with(|n| n.get());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let derivation_path = vec![request.token.as_bytes().to_vec()];
    let tx_id = btc_utils::send(
        network,
        key_name,
        derivation_path,
        request.destination_address,
        request.amount_in_satoshi,
    )
    .await;

    tx_id.to_string()
}

#[update (name = "Get_ICP_Balance")]
pub async fn get_icp_balance(request: icp_utils::BalanceRequest) -> icp_utils::BalanceResult {
    icp_utils::get_icp_balance(request).await
}

#[update (name = "Send_ICP")]
pub async fn send_icp(request: icp_utils::SendRequest) -> icp_utils::SendResult {
    icp_utils::send(request).await
}
