use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use sha2::{Digest, Sha256};
// use ethereum_types::{H160, H256};
// use tiny_keccak::{Keccak};


pub fn get_cketh_address(user_name: String) -> String{
    let mut hasher = Sha256::new();
    hasher.update(user_name);
    let result = hasher.finalize();
    let sub_id: [u8; 32]= result.into();
    let new_subaccount = Account{ owner: ic_cdk::api::id(), subaccount: Some(sub_id)};
    new_subaccount.to_string()
}

pub fn get_eth_address(user_name: String) -> String {
    // let mut hasher = Keccak::v256();
    // let mut result = [0u8; 32];
    // hasher.update(user_name.as_str().as_bytes());
    // result = hasher.finalize();
    // H160::from(H256::from_slice(&result[12..])).to_string()
    "".to_string()
}