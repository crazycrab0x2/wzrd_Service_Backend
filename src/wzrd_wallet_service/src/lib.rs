mod btc_types;
mod btc_utils;
mod icp_utils;
mod eth_utils;

use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use ic_cdk_macros::update;
use std::cell::{Cell, RefCell};

thread_local! {
    static NETWORK: Cell<BitcoinNetwork> = Cell::new(BitcoinNetwork::Mainnet);
    static KEY_NAME: RefCell<String> = RefCell::new(String::from("key_1"));
}

#[update (name = "Get_BTC_Address")]
pub async fn get_btc_address(user_name: String) -> String {
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let network = NETWORK.with(|n| n.get());
    btc_utils::get_btc_address(network, key_name, user_name).await
}

#[update (name = "Get_BTC_Balance")]
pub async fn get_btc_balance(address: String) -> u64 {
    let network = NETWORK.with(|n| n.get());
    btc_utils::get_balance(network, address).await
}

#[update (name = "Send_BTC")]
pub async fn send_btc(request: btc_types::SendRequest) -> String {
    let derivation_path = vec![request.sender.as_bytes().to_vec()];
    let network = NETWORK.with(|n| n.get());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let tx_id = btc_utils::send(
        network,
        derivation_path,
        key_name,
        request.destination_address,
        request.amount_in_satoshi,
    )
    .await;

    tx_id.to_string()
}


#[update (name = "Get_ICP_Address")]
pub async fn get_icp_address(user_name: String) -> String {
    icp_utils::get_icp_address(user_name)
}

#[update (name = "Get_ICP_Balance")]
pub async fn get_icp_balance(user_name: String) -> String {
    icp_utils::get_icp_balance(user_name).await
}

#[update (name = "Send_ICP")]
pub async fn send_icp(request: icp_utils::SendRequest) -> String {
    icp_utils::send(request).await
}

#[update (name = "Get_ckETH_Address")]
pub async fn get_cketh_address(user_name: String) -> String {
    eth_utils::get_cketh_address(user_name)
}

// #[update (name = "Get_ETH_Address")]
// pub async fn get_eth_address(user_name: String) -> String {
//     eth_utils::get_eth_address(user_name)
// }
