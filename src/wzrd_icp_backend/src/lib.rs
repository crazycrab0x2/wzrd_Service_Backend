use ic_cdk::{
    export:: {candid::CandidType, Principal},
    query, update,
};
use std::{cell::RefCell};
use ic_cdk::api::time;
use std::collections::BTreeMap;
use std::vec::Vec;
use std::io::Write;

// use crypto::aes::{ebc_decryptor, ebc_encryptor, KeySize};
// use crypto::blockmodes::PkcsPadding;
// use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
// use crypto::digest::Digest;
// use crypto::pbkdf2::pbkdf2;
// use rand_core::{RngCore, OsRng};
// use rand::Rng;

type IdStore = BTreeMap<String, Principal>;
type PhoneNumberStore = BTreeMap<String, Principal>;
type EmailAddressStore = BTreeMap<String, Principal>;
type ProfileStore = BTreeMap<Principal, Profile>;

#[derive(Clone, Debug, Default, CandidType)]
pub struct Profile {
    pub phone_number: Option<String>,
    pub email_address: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

thread_local! {
    static PROFILE_STORE: RefCell<ProfileStore> = RefCell::default();
    static ID_STORE: RefCell<IdStore> = RefCell::default();
    static PHONE_NUMBER_STORE: RefCell<PhoneNumberStore> = RefCell::default();
    static EMAIL_ADDRESS_STORE: RefCell<EmailAddressStore> = RefCell::default();
}

fn add_id(id: String, principal: Principal) -> Result<(), String> {
    ID_STORE.with(|id_store| {
        id_store.borrow_mut().insert(id.clone(), principal);
    });

    Ok(())
}

fn has_id(id: &str) -> bool {
    ID_STORE.with(|id_store| {
        let store = id_store.borrow();
        // binding.get(id).is_some()
        store.contains_key(id)
    })
}

fn add_phone_number(phone_number: Option<String>, principal: Principal) -> Result<(), String> {
    if phone_number.is_some() {
        PHONE_NUMBER_STORE.with(|store| {
            store.borrow_mut().insert(phone_number.unwrap(), principal);
        });
    }
    return Ok(());
}

fn has_phone_number(phone_number: &String) -> bool {
    PHONE_NUMBER_STORE.with(|phone_number_store| {
        let binding = phone_number_store.borrow();
        binding.get(phone_number).is_some()
    })
}

fn add_email_address(email_address: Option<String>, principal: Principal) -> Result<(), String> {
    if email_address.is_some() {
        EMAIL_ADDRESS_STORE.with(|store| {
            store.borrow_mut().insert(email_address.unwrap(), principal);
        });
    }
    Ok(())
}

fn has_email_address(email_address: &String) -> bool {
    EMAIL_ADDRESS_STORE.with(|email_address_store| {
        let binding = email_address_store.borrow();
        binding.get(email_address).is_some()
    })
}

fn generate_random_number() -> u64 {
    let current_time = time();
    let pseudo_random_number = current_time % u64::MAX;

    pseudo_random_number
}

fn make_challenge() -> String {
    let challenge = generate_random_number();

    challenge.to_string()
}

fn derive_key(password: &str, salt: &[u8]) -> Vec<u8> {
    let mut key = vec![0; 32]; // 256 bits
    
    key
}

fn encrypt(data: &str, key: &[u8], iv: &[u8]) -> Vec<u8> {
    // let mut encryptor = cbc_encryptor(KeySize::KeySize256, key, iv, PkcsPadding);
    let mut buffer:Vec<u8>  = Vec::new();
    buffer.write(data.as_bytes()).unwrap();
    let mut final_result:Vec<u8>  = Vec::new();
    // loop {
    //     let mut read_buffer = [0; 4096];
    //     let result = buffer.read(&mut read_buffer).unwrap();
    //     // encryptor
    //     //     .crypt(&read_buffer[..result], &mut final_result)
    //     //     .unwrap();
    //     if result == 0 {
    //         break;
    //     }
    // }
    final_result
}

fn decrypt(data: &[u8], key: &[u8]) -> String {

    let min_length = data.len().min(key.len());

    let result:Vec<u8> = data
        .iter()
        .zip(key.iter())
        .map(|(&a, &b)| a^ b)
        .collect();

    let remaining1 = data[min_length..].to_vec();
    let remaining2 = key[min_length..].to_vec();

    let final_result = [&result[..], &remaining1[..], &remaining2[..]].concat();

    String::from_utf8(final_result).unwrap()
}

#[query(name = "loginRequest")]
pub fn login_request() -> String {
    make_challenge()
}

#[query(name = "checkId")]
pub fn check_id(id: String) -> bool {
    has_id(&id)
}

#[query]
pub fn getprofile(id: String) -> Profile {

    println!("{:#?}", id);
    ID_STORE.with(|id_store| {
        PROFILE_STORE.with(|profile_store| {
            id_store
                .borrow()
                .get(&id)
                .and_then(|id| profile_store.borrow().get(id).cloned())
                .unwrap_or_default()
        })
    })
}

#[update(name = "reserveID")]
pub fn reserve_id(id: String, phone_number: String, email_address: String) -> bool {
    if has_id(&id) {
        return false;
    }

    if has_phone_number(&phone_number) {
        return false;
    }

    if has_email_address(&email_address) {
        return false;
    }

    let caller = ic_cdk::caller();

    create_profile(
        id,
        caller,
        None,
        None,
        Some(phone_number),
        Some(email_address),
    )
    .unwrap();
    return true;
}

fn create_profile(
    id: String,
    principal: Principal,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
    email_address: Option<String>,
) -> Result<(), String> {
    let _ = add_id(id, principal);
    let _ = add_phone_number(phone_number.clone(), principal);
    let _ = add_email_address(email_address.clone(), principal);

    let profile = Profile {
        first_name,
        last_name,
        phone_number,
        email_address,
    };
    PROFILE_STORE.with(|profile_store| {
        profile_store.borrow_mut().insert(principal, profile);
    });

    Ok(())
}

#[query]
pub fn register(
    id: String,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
    email_address: Option<String>,
) -> () {
    // let res = check_id(id.clone());
    // if !res {
    //     return false;
    // }
    let caller = ic_cdk::caller();

    create_profile(
        id,
        caller,
        first_name,
        last_name,
        phone_number,
        email_address,
    )
    .unwrap();

    // true
}

#[update]
pub fn register_with_challenge(
    challenge: String,
    id: String,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
    email_address: Option<String>,
) -> bool {
    let _id = decrypt(id.as_bytes(), challenge.as_bytes());
    let _first_name = if first_name.is_some() {
        decrypt(first_name.unwrap_or(String::new()).as_bytes(), challenge.as_bytes())
    } else {
        String::new()
    };
        
    let _last_name = if last_name.is_some() {
        decrypt(last_name.unwrap_or(String::new()).as_bytes(), challenge.as_bytes())
    } else {
        String::new()
    };

    let _phone_number = if phone_number.is_some() {
        decrypt(phone_number.unwrap_or(String::new()).as_bytes(), challenge.as_bytes())
    } else {
        String::new()
    };

    let _email_address = if email_address.is_some() {
        decrypt(email_address.unwrap_or(String::new()).as_bytes(), challenge.as_bytes())
    } else {
        String::new()
    };

    register(_id, Some(_first_name), Some(_last_name), Some(_phone_number), Some(_email_address))
}

#[update]
fn authenticate(challenge: String, _id: String) -> bool {
    let id = decrypt(_id.as_bytes(), challenge.as_bytes());
    has_id(&id)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_create_profile() {
        let id = String::from("abcde");
        let user = Principal::anonymous();

        create_profile(
            id,
            user,
            Some(String::from("john")),
            Some(String::from("do")),
            Some(String::from("12345")),
            Some(String::from("john@mail.com")),
        )
        .unwrap();

        // check id
        assert_eq!(has_id(&String::from("abcde")), true);
        assert_eq!(has_id(&String::from("wrong")), false);

        // check phone_number
        assert_eq!(has_phone_number(&String::from("12345")), true);
        assert_eq!(has_phone_number(&String::from("12252")), false);

        // check email_address
        assert_eq!(has_email_address(&String::from("john@mail.com")), true);
        assert_eq!(has_email_address(&String::from("jahn@mail.com")), false);
    }
}
