use sha2::{Digest, Sha256};
use ic_cdk::update;
use ic_cdk::api::caller;
use ic_ledger_types::{AccountIdentifier, Subaccount, transfer, Tokens, Memo, TransferArgs, DEFAULT_FEE, DEFAULT_SUBACCOUNT, AccountBalanceArgs, MAINNET_LEDGER_CANISTER_ID, account_balance};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize)]
pub struct SendRequest {
    pub sender: String,
    pub destination_address: String,
    pub amount_in_e8s: u64,
}

pub fn get_icp_address(user_name: String) -> String{
    let mut hasher = Sha256::new();
    hasher.update(user_name);
    let result = hasher.finalize();
    let sub_id= Subaccount(result.into());
    let new_subaccount = AccountIdentifier::new( &ic_cdk::api::id(), &sub_id);
    new_subaccount.to_string()
}

pub async fn send(request: SendRequest) -> String {
    let mut hasher = Sha256::new();
    hasher.update(request.sender);
    let result = hasher.finalize();
    let sender_id: Option<Subaccount> = Some(Subaccount(result.into()));

    let block_index = transfer(
        MAINNET_LEDGER_CANISTER_ID,
        TransferArgs {
          memo: Memo(0),
          amount: Tokens::from_e8s(request.amount_in_e8s),
          fee: DEFAULT_FEE,
          from_subaccount: sender_id,
          to: AccountIdentifier::from_hex(&request.destination_address.as_str()).unwrap(),
          created_at_time: None,
        }
      ).await;
    block_index.unwrap().unwrap().to_string()
}

pub async fn get_icp_balance(address: String) -> String {
    let balance = account_balance(
        MAINNET_LEDGER_CANISTER_ID,
        AccountBalanceArgs {
          account: AccountIdentifier::from_hex(address.as_str()).unwrap()
        }
      ).await;
    balance.unwrap().e8s().to_string()
}