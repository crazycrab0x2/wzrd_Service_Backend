mod bitcoin_api;
mod bitcoin_wallet;
mod ecdsa_api;
mod types;
mod btc_utils;

use ic_cdk::api::management_canister::bitcoin::{
    BitcoinNetwork, GetUtxosResponse, MillisatoshiPerByte,
};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, update};
use std::cell::{Cell, RefCell};

thread_local! {
    static NETWORK: Cell<BitcoinNetwork> = Cell::new(BitcoinNetwork::Regtest);
    static DERIVATION_PATH: Vec<Vec<u8>> = vec![vec![0;5];5];
    static KEY_NAME: RefCell<String> = RefCell::new(String::from("dfx_test_key"));
}

#[update (name = "Get_BTC_Address")]
pub async fn get_btc_address(user_name: String) -> String {
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let network = NETWORK.with(|n| n.get());
    btc_utils::get_btc_address(network, key_name, user_name).await
}

#[update (name = "Get_BTC_Balance")]
pub async fn get_balance(address: String) -> u64 {
    let network = NETWORK.with(|n| n.get());
    btc_utils::get_balance(network, address).await
}

#[update]
pub async fn send(request: types::SendRequest) -> String {
    let derivation_path = vec![request.user_name.as_bytes().to_vec()];
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

