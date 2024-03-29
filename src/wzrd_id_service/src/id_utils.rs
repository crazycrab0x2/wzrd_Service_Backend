use ic_cdk::api::time;
use ic_cdk::export::candid::CandidType;
use std::cell::RefCell;
use std::collections::BTreeMap;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use candid::Deserialize;

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct RegisterParams{
    pub user_name: String,
    pub key_id: String,
    pub public_key: String
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct RequestResult{
    pub error: String,
    pub result: String
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct SetProfileResult{
    pub error: String,
    pub result: bool
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct GetProfileResult{
    pub error: String,
    pub result: Option<Profile>
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct AuthResult{
    pub error: String,
    pub result: bool
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct AuthenticationParams {
    pub user_name: String,
    pub key_id: String,
    pub signature: String,
    pub authenticator_data: String
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct Profile {
    pub phone: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct FidoKey {
    pub key_id: String,
    pub public_key: String,
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct SetProfileParams {
    pub user_name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>
}

type KeyStore = BTreeMap<String, FidoKey>; //(user_name => Fido key)
type ProfileStore = BTreeMap<String, Profile>; //(user_name => vec<key_id>)

thread_local! {
    pub static KEY_STORE: RefCell<KeyStore> = RefCell::default();
    pub static PROFILE_STORE: RefCell<ProfileStore> = RefCell::default();
}

pub fn register_request(username: String) -> RequestResult {
    KEY_STORE.with( |key_store| {
        if key_store.borrow().get(&username).is_some() {
            RequestResult{
                error: "Username already registered".to_string(),
                result: "".to_string()
            }
        }
        else{
            RequestResult{
                error: "".to_string(),
                result: get_challenge()
            }
        }
    })
}

pub fn register(params: RegisterParams) -> AuthResult {
    KEY_STORE.with( |key_store| {
        if key_store.borrow().get(&params.user_name).is_some() {
            AuthResult{
                error: "Username already registered".to_string(),
                result: false
            }
        }
        else{
            let new_key = FidoKey{
                key_id: params.key_id,
                public_key: params.public_key
            };
            key_store.borrow_mut().insert(params.user_name, new_key);
            AuthResult{
                error: "".to_string(),
                result: true
            }
        }
    })
}

pub fn authentication_request(user_name: String) -> RequestResult {
    KEY_STORE.with( |key_store| {
        if key_store.borrow().get(&user_name).is_some() {
            RequestResult{
                result: get_challenge(),
                error: "".to_string()
            }
        }
        else{
            RequestResult{
                error: "Username not registered".to_string(),
                result: "".to_string()
            }
        }
    })
}

pub fn authentication(params: AuthenticationParams) -> RequestResult {
    KEY_STORE.with( |key_store| {
        if key_store.borrow().get(&params.user_name).is_some() {
            let key = key_store.borrow().get(&params.user_name).unwrap().clone();
            if key.key_id == params.key_id {
                RequestResult{
                    error: "".to_string(),
                    result: generate_token(params.user_name, params.key_id)
                }
            }
            else{
                RequestResult{
                    error: "Authentication failed".to_string(),
                    result: "".to_string()
                }
            }
        }
        else{
            RequestResult{
                error: "Username not registered".to_string(),
                result: "".to_string()
            }
        }
    })
}

pub fn get_challenge() -> String {
    let mut number = time();
    let char_set = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
        "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9"
    ];
    let mut result: String = "".to_string();
    while number > 0 {
        let index = (number % 62) as usize;
        result.push_str(char_set[index]);
        number /= 62;
    }
    result
}

pub fn reverse_timestamp(timestamp: String) -> u64 {
    let char_set = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
        "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9"
    ];
    let mut num = 0u64;
    for (index, c) in timestamp.chars().enumerate() {
        num += (char_set.iter().position(|&x| x.to_string() == c.to_string()).unwrap() as u64) * 62u64.pow(index.clone() as u32);
    }
    num
}

pub fn generate_token(user_name: String, key_id: String) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"wzrd-secret-key").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("username", user_name);
    claims.insert("keyId", key_id);
    claims.insert("timestamp", get_challenge());
    let token_str = claims.sign_with_key(&key).unwrap();
    token_str
}

pub fn check_token(token: String) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"wzrd-secret-key").unwrap();
    let veri_claims;
    let result: Result<BTreeMap<String, String>, jwt::Error> = token.as_str().verify_with_key(&key);
    match result{
        Ok(okay_result) => veri_claims = okay_result,
        Err(_) => return "".to_string()
    };
    let user_name = &veri_claims["username"];
    let key_id = &veri_claims["keyId"];
    let timestamp_str = &veri_claims["timestamp"];
    let timestamp = reverse_timestamp(timestamp_str.clone());

    if time() - timestamp > 3600000000000 {
        "".to_string()
    }
    else{
        KEY_STORE.with( |key_store| {
            if key_store.borrow().get(user_name).is_some() {
                let key = key_store.borrow().get(user_name).unwrap().clone();
                if key.key_id == *key_id {
                    generate_token(user_name.clone(), key_id.clone())
                }
                else{
                    "".to_string()
                }
            }
            else{
                "".to_string()
            }
        })
    }
}

pub fn has_user(user_name: &String) -> bool {
    KEY_STORE.with(|key_store| key_store.borrow().get(user_name).is_some())
}

pub fn set_profile(params: SetProfileParams) -> SetProfileResult {
    KEY_STORE.with( |key_store| {
        if key_store.borrow().get(&params.user_name).is_some() {
            let profile = Profile {
                first_name: params.first_name,
                last_name: params.last_name,
                phone: params.phone,
                email: params.email,
            };
            PROFILE_STORE.with(|profile_store| {
                profile_store.borrow_mut().insert(params.user_name, profile);
            });
            SetProfileResult{
                error: "".to_string(),
                result: true
            }
        }
        else{
            SetProfileResult{
                error: "Username doesn't exist".to_string(),
                result: false
            }
        }
    })
}

pub fn get_profile(user_name: String) -> GetProfileResult {
    KEY_STORE.with(|key_store| {
        if key_store.borrow().get(&user_name).is_some() {
            PROFILE_STORE.with(|profile_store| {
                if profile_store.borrow().get(&user_name).is_some() {
                    let user_profile = profile_store.borrow().get(&user_name).unwrap().clone();
                    GetProfileResult{
                        error: "".to_string(),
                        result: Some(user_profile)
                    }
                }
                else{
                    GetProfileResult{
                        error: "User profile doesn't exist".to_string(),
                        result: None
                    }
                }
            })
        }
        else {
            GetProfileResult{
                error: "Username doesn't exist".to_string(),
                result: None
            }
        }
    })
}
