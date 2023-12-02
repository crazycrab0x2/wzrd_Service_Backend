use ic_cdk::{
    export:: {candid::CandidType, Principal}
};
use std::{cell::RefCell};
use std::collections::BTreeMap;

type IdStore = BTreeMap<String, Principal>;
type ProfileStore = BTreeMap<Principal, Profile>;
type PhoneNumberStore = BTreeMap<String, Principal>;
type EmailAddressStore = BTreeMap<String, Principal>;
type PasskeyStore = BTreeMap<String, String>;

#[derive(Clone, Debug, Default, CandidType)]
pub struct Profile {
    pub id: String,
    pub phone_number: Option<String>,
    pub email_address: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

thread_local! {
    pub static PROFILE_STORE: RefCell<ProfileStore> = RefCell::default();
    pub static ID_STORE: RefCell<IdStore> = RefCell::default();
    pub static PASSKEY_STORE: RefCell<PasskeyStore> = RefCell::default();
    pub static PHONE_NUMBER_STORE: RefCell<PhoneNumberStore> = RefCell::default();
    pub static EMAIL_ADDRESS_STORE: RefCell<EmailAddressStore> = RefCell::default();
}

pub fn set_passkey(id: String, passkey: String) -> bool{
    PASSKEY_STORE.with(|passkey_store| {
        *passkey_store.borrow_mut().entry(id).or_insert("None".to_string()) = passkey;
    });
    true
}

pub fn get_passkey(id: String) -> String {
    PASSKEY_STORE.with(|passkey_store| {
        passkey_store.borrow().get(&id).unwrap_or(&"None".to_string()).clone()
    })
}

pub fn has_id(id: &String) -> bool {
    ID_STORE.with(|id_store| {
        let store = id_store.borrow();
        store.get(id).is_some()
    })
}

pub fn has_phone_number(phone_number: &String) -> bool {
    PHONE_NUMBER_STORE.with(|phone_number_store| {
        let binding = phone_number_store.borrow();
        binding.get(phone_number).is_some()
    })
}


pub fn has_email_address(email_address: &String) -> bool {
    EMAIL_ADDRESS_STORE.with(|email_address_store| {
        let binding = email_address_store.borrow();
        binding.get(email_address).is_some()
    })
}

pub fn add_id(id: String, principal: Principal) -> Result<(), String> {
    ID_STORE.with(|id_store| {
        id_store.borrow_mut().insert(id.clone(), principal);
    });

    Ok(())
}

pub fn add_phone_number(phone_number: Option<String>, principal: Principal) -> Result<(), String> {
    if phone_number.is_some() {
        PHONE_NUMBER_STORE.with(|store| {
            store.borrow_mut().insert(phone_number.unwrap(), principal);
        });
    }
    return Ok(());
}

pub fn add_email_address(email_address: Option<String>, principal: Principal) -> Result<(), String> {
    if email_address.is_some() {
        EMAIL_ADDRESS_STORE.with(|store| {
            store.borrow_mut().insert(email_address.unwrap(), principal);
        });
    }
    Ok(())
}

pub fn create_profile(
    id: String,
    principal: Principal,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
    email_address: Option<String>,
) -> Result<(), String> {
    let _ = add_id(id.clone(), principal);
    let _ = add_phone_number(phone_number.clone(), principal);
    let _ = add_email_address(email_address.clone(), principal);

    let profile = Profile {
        id,
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

pub fn get_profile(id: String) -> Profile {
    let none_profile = Profile{
        id: "".to_string(),
        first_name: None,
        last_name: None,
        phone_number: None,
        email_address: None,
    };
    let principal = ID_STORE.with(|id_store| id_store.borrow().get(&id).unwrap().clone());
    PROFILE_STORE.with(|profile_store| profile_store.borrow().get(&principal).unwrap_or(&none_profile).clone())
}

pub fn login(id: String, passkey: String) -> bool {
    PASSKEY_STORE.with(|passkey_store| {
        if let Some(value) = passkey_store.borrow().get(&id) {
            if *value == passkey{
                return true;
            }
        }
        return false;
    })
}