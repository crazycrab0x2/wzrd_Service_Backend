use ic_cdk::{
    export:: {candid::CandidType, Principal},
    query, update,
};
use std::{cell::RefCell};
use std::collections::BTreeMap;

type IdStore = BTreeMap<String, Principal>;
type ProfileStore = BTreeMap<Principal, Profile>;
type PhoneNumberStore = BTreeMap<String, Principal>;
type EmailAddressStore = BTreeMap<String, Principal>;

#[derive(Clone, Debug, Default, CandidType)]
pub struct Profile {
    pub phone_number: Option<String>,
    pub email_address: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

thread_local! {
    pub static PROFILE_STORE: RefCell<ProfileStore> = RefCell::default();
    pub static ID_STORE: RefCell<IdStore> = RefCell::default();
    pub static PHONE_NUMBER_STORE: RefCell<PhoneNumberStore> = RefCell::default();
    pub static EMAIL_ADDRESS_STORE: RefCell<EmailAddressStore> = RefCell::default();
}

pub fn has_id(id: String) -> bool {
    ID_STORE.with(|id_store| {
        let store = id_store.borrow();
        store.get(&id).is_some()
        // store.contains_key(&id)
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