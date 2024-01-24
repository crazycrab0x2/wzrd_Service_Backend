use sha2::{Digest, Sha256};
use ic_ledger_types::{AccountIdentifier, Subaccount, transfer, Tokens, Memo, TransferArgs, DEFAULT_FEE, AccountBalanceArgs, MAINNET_LEDGER_CANISTER_ID, account_balance};
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct BalanceRequest {
    pub token: String,
    pub address: String
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct BalanceResult {
    pub token: String,
    pub balance: String
}

#[derive(CandidType, Deserialize)]
pub struct SendRequest {
    pub token: String,
    pub destination_address: String,
    pub amount_in_e8s: u64,
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct SendResult {
    pub token: String,
    pub block_id: String
}

pub fn get_icp_address(user_name: String) -> String{
    let mut hasher = Sha256::new();
    hasher.update(user_name);
    let result = hasher.finalize();
    let sub_id= Subaccount(result.into());
    let new_subaccount = AccountIdentifier::new( &ic_cdk::api::id(), &sub_id);
    new_subaccount.to_string()
}

pub async fn get_icp_balance(request: BalanceRequest) -> BalanceResult {
  let token_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("urpxs-4aaaa-aaaap-qb6mq-cai").unwrap(), "CheckToken", (request.token,)).await;
  match token_validation {
    Err(_err) => {
      return BalanceResult{
          token: "".to_string(),
          balance: "Can't access Id service".to_string()
      };
    }
    Ok((new_token, )) => {
      if new_token == "".to_string() {
          return BalanceResult{
              token: new_token,
              balance: "".to_string()
          };
      }
      else{
        let balance = account_balance(
            MAINNET_LEDGER_CANISTER_ID,
            AccountBalanceArgs {
              account: AccountIdentifier::from_hex(request.address.as_str()).unwrap()
            }
          ).await;
        match balance {
          Ok(tokens) => {
            return BalanceResult{
              token: new_token,
              balance: tokens.e8s().to_string()
            };
          },
          Err((reject_code, error)) => {
            return BalanceResult{
              token: new_token,
              balance: error
            };
          }
        }
      }
    }
  }
}

pub async fn send(request: SendRequest) -> SendResult {
    let mut hasher = Sha256::new();
    hasher.update(request.token);
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
    // block_index.unwrap().unwrap().to_string()
    SendResult{
      token: "".to_string(),
      block_id: "".to_string()
    }
}