use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use sha2::{Digest, Sha256};
// use ethers::{core::utils::Ganache, signers::LocalWallet};


pub fn get_cketh_address(user_name: String) -> String{
    let mut hasher = Sha256::new();
    hasher.update(user_name);
    let result = hasher.finalize();
    let sub_id: [u8; 32]= result.into();
    let new_subaccount = Account{ owner: ic_cdk::api::id(), subaccount: Some(sub_id)};
    new_subaccount.to_string()
}

// pub fn get_eth_address(user_name: String) -> String {
//     let mnemonic = "gas monster ski craft below illegal discover limit dog bundle bus artefact";
//     let ganache = Ganache::new().mnemonic(mnemonic).spawn();
//     let wallet: LocalWallet = ganache.keys()[0].clone().into();
//     let wallet_address: String = wallet.address().encode_hex();
//     wallet_address
// }