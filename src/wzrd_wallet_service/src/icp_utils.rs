use sha2::{Digest, Sha256};
use ic_ledger_types::{AccountIdentifier, Subaccount, transfer, Tokens, Memo, TransferArgs, DEFAULT_FEE, AccountBalanceArgs, MAINNET_LEDGER_CANISTER_ID, account_balance};

pub fn get_icp_address(user_name: String) -> String{
    let mut hasher = Sha256::new();
    hasher.update(user_name);
    let result = hasher.finalize();
    let sub_id= Subaccount(result.into());
    let new_subaccount = AccountIdentifier::new( &ic_cdk::api::id(), &sub_id);
    new_subaccount.to_string()
}

pub async fn get_icp_balance(address: String) -> String {
  let balance = account_balance(
      MAINNET_LEDGER_CANISTER_ID,
      AccountBalanceArgs {
        account: AccountIdentifier::from_hex(address.as_str()).unwrap()
      }
    ).await;
  match balance {
    Ok(tokens) => {
      tokens.e8s().to_string()
    }
    Err((_, error)) => {
      error
    }
  }
}

pub async fn send_icp(phrase: String, des_address: String, amount: u64) -> String {
  let mut hasher = Sha256::new();
  hasher.update(phrase);
  let result = hasher.finalize();
  let sender_id: Option<Subaccount> = Some(Subaccount(result.into()));

  let block_index = transfer(
      MAINNET_LEDGER_CANISTER_ID,
      TransferArgs {
        memo: Memo(0),
        amount: Tokens::from_e8s(amount),
        fee: DEFAULT_FEE,
        from_subaccount: sender_id,
        to: AccountIdentifier::from_hex(des_address.as_str()).unwrap(),
        created_at_time: None,
      }
    ).await;
  match block_index {
    Ok(result,) => {
      match result {
        Ok(block_id,) => block_id.to_string(),
        Err(err,) => err.to_string()
      }
    }
    Err((_, error)) => {
      error
    }
  }
}