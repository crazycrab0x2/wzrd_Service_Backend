use sha2::{Digest, Sha256};
use ic_cdk::{update, print};
use candid::types::principal::Principal;
use ic_cdk::api::{caller, call::call};
use ic_ledger_types::{AccountIdentifier, Subaccount, DEFAULT_SUBACCOUNT, AccountBalanceArgs, Tokens, MAINNET_LEDGER_CANISTER_ID, account_balance};

#[update(name="create_ledger_account")]
pub fn create_ledger_account(user_name: String) -> String{
    let mut hasher = Sha256::new();
    hasher.update(user_name);
    let result = hasher.finalize();
    let sub_id= Subaccount(result.into());
    let new_subaccount = AccountIdentifier::new( &caller(), &sub_id);
    new_subaccount.to_string()
}

pub fn get_icp_address(user_name: String) -> String {
    "".to_string()
}

pub async fn get_icp_balance(user_name: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(user_name);
    let result = hasher.finalize();
    let sub_id= Subaccount(result.into());
    let balance = account_balance(
        MAINNET_LEDGER_CANISTER_ID,
        // caller(),
        AccountBalanceArgs {
          account: AccountIdentifier::new(&caller(), &DEFAULT_SUBACCOUNT)
        }
      ).await;
    balance.unwrap().e8s().to_string()
}