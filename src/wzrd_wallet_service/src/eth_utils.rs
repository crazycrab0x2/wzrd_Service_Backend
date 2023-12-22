use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use sha2::{Digest, Sha256};
use candid::Principal;
use candid::CandidType;
// use ethereum_types::{H160, H256};
// use tiny_keccak::{Keccak};
#[derive(Clone, Debug, CandidType)]
pub enum AuthType {
    Rpc,
    RegisterProvider,
    FreeRpc,
    Admin,
}


pub fn get_cketh_address(user_name: String) -> String{
    let mut hasher = Sha256::new();
    hasher.update(user_name);
    let result = hasher.finalize();
    let sub_id: [u8; 32]= result.into();
    let new_subaccount = Account{ owner: ic_cdk::api::id(), subaccount: Some(sub_id)};
    new_subaccount.to_string()
}

pub async fn get_eth_address() -> String {
    let id = ic_cdk::api::id();
    let user_validation = ic_cdk::call::<(Principal, AuthType,), ()>(Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap(), "authorize", (id.clone(), AuthType::RegisterProvider,)).await;
    match user_validation {
        Err(_err) => {
            return "Can't access store".to_string();
        }
        Ok(result) => {
            "okay".to_owned()
        }
    }
}