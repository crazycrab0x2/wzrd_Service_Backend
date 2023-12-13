use ic_cdk::api::time;
use ic_cdk::export::candid::CandidType;
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, CandidType)]
pub struct Profile {
    pub phone_number: Option<String>,
    pub email_address: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Clone, Debug, Default, CandidType)]
pub struct RegisterRequestData {
    pub challenge: String,
    pub exclude_credentials: Vec<String>,
}

#[derive(Clone, Debug, Default, CandidType)]
pub struct AuthenticationRequestData {
    pub challenge: String,
    pub allow_credentials: Vec<String>,
}

type KeyIdStore = BTreeMap<String, Vec<String>>; //(user_name => vec<key_id>)
type PublicKeyStore = BTreeMap<String, String>; //(key_id => public_key)
type ProfileStore = BTreeMap<String, Profile>;  //(user_name => profile)

thread_local! {
    pub static KEY_ID_STORE: RefCell<KeyIdStore> = RefCell::default();
    pub static PUBLIC_KEY_STORE: RefCell<PublicKeyStore> = RefCell::default();
    pub static PROFILE_STORE: RefCell<ProfileStore> = RefCell::default();
}

pub fn register_request(user_name: String) -> RegisterRequestData {
    KEY_ID_STORE.with(|key_id_store| {
        let credential_list = key_id_store
            .borrow()
            .get(&user_name)
            .unwrap_or(&vec![])
            .clone();
        RegisterRequestData {
            challenge: get_challenge(),
            exclude_credentials: credential_list,
        }
    })
}

pub fn register(user_name: String, key_id: String, public_key: String) -> bool {
    KEY_ID_STORE.with(|key_id_store| {
        let mut key_id_list = key_id_store
            .borrow()
            .get(&user_name)
            .unwrap_or(&vec![])
            .clone();
        if key_id_list.contains(&key_id) {
            false
        } else {
            key_id_list.push(key_id.clone());
            key_id_store.borrow_mut().insert(user_name, key_id_list);
            PUBLIC_KEY_STORE.with(|public_key_store| {
                public_key_store.borrow_mut().insert(key_id, public_key);
            });
            true
        }
    })
}

pub fn authentication_request(user_name: String) -> AuthenticationRequestData {
    if user_name == "".to_string() {
        AuthenticationRequestData {
            challenge: get_challenge(),
            allow_credentials: vec![],
        }
    } 
    else {
        KEY_ID_STORE.with(|key_id_store| {
            if !key_id_store.borrow().get(&user_name).is_some() {
                AuthenticationRequestData {
                    challenge: "".to_string(),
                    allow_credentials: vec![],
                }
            } else {
                AuthenticationRequestData {
                    challenge: get_challenge(),
                    allow_credentials: key_id_store.borrow().get(&user_name).unwrap().clone(),
                }
            }
        })
    }
}

pub fn authentication(user_name: String, key_id: String, authenticator_data: String, signature: String) -> bool {
    KEY_ID_STORE.with(|key_id_store|{
        let user_key_list = key_id_store.borrow().get(&user_name).unwrap_or(&vec![]).clone();
        if user_key_list.contains(&key_id) {
            PUBLIC_KEY_STORE.with(|public_key_store| {
                let mut public_key = public_key_store.borrow().get(&key_id).unwrap("".to_string()).clone();
                if public_key != "".to_string() {
                    // authorize authenticator_data and signature with stored public_key corresponding with key_id
                }
                else{
                    false
                }
            })
        }
        else{
            false
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

pub fn has_user(user_name: &String) -> bool {
    KEY_ID_STORE.with(|key_id_store| key_id_store.borrow().get(user_name).is_some())
}

pub fn set_profile(
    user_name: String,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
    email_address: Option<String>,
) -> bool {
    let profile = Profile {
        first_name,
        last_name,
        phone_number,
        email_address,
    };
    PROFILE_STORE.with(|profile_store| {
        profile_store.borrow_mut().insert(user_name, profile);
    });
    true
}

pub fn get_profile(user_name: String) -> Profile {
    let none_profile = Profile {
        first_name: None,
        last_name: None,
        phone_number: None,
        email_address: None,
    };
    PROFILE_STORE.with(|profile_store| {
        profile_store
            .borrow()
            .get(&user_name)
            .unwrap_or(&none_profile)
            .clone()
    })
}
