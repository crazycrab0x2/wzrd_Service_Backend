use candid::parser::test::check;
use ic_cdk::{
    export::{
        candid::{CandidType, Deserialize},
        Principal,
    },
    query, update,
};
use core::borrow;
use std::cell::RefCell;
use std::collections::BTreeMap;

type IdStore = BTreeMap<String, Principal>;
type PhoneNumberStore = BTreeMap<String, Principal>;
type EmailAddressStore = BTreeMap<String, Principal>;
type ProfileStore = BTreeMap<Principal, Profile>;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Profile {
    pub phone_number: Option<String>,
    pub email_address: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>
}

thread_local! {
    static PROFILE_STORE: RefCell<ProfileStore> = RefCell::default();
    static ID_STORE: RefCell<IdStore> = RefCell::default();
    static PHONE_NUMBER_STORE: RefCell<PhoneNumberStore> = RefCell::default();
    static EMAIL_ADDRESS_STORE: RefCell<EmailAddressStore> = RefCell::default();
}

#[query(name = "checkId")]
fn check_id(id: String) -> bool {
    ID_STORE.with(|id_store| {
        let binding = id_store.borrow();
        binding.get(&id).is_some()
    })
} 

#[query(name = "getProfile")]
fn get_profile(id: String) -> Profile {
    ID_STORE.with(|id_store| {
        PROFILE_STORE.with(|profile_store| {
            id_store
                .borrow()
                .get(&id)
                .and_then(|id| profile_store.borrow().get(id).cloned()).unwrap_or_default()
        })
    })
}

#[update(name = "reserveID")]
fn reserve_id(id: String, phone_number: Option<String>, email_address: Option<String>) -> bool {
    let mut res = check_id(id);
    if res {
        return false;
    }

    if phone_number.is_some() {
        let phone_number = phone_number.unwrap();
        let find = PHONE_NUMBER_STORE.with(|phone_number_store| {
            let binding = phone_number_store.borrow();
            binding.get(&phone_number).is_some()
        });
    }

    if email_address.is_some() {
        let email_address = email_address.unwrap();
        let find: bool = EMAIL_ADDRESS_STORE.with(|email_address_store| {
            let binding = email_address_store.borrow();
            binding.get(&email_address).is_some()
        });
    }

    let principal_id = ic_cdk::api::caller();
    ID_STORE.with(|id_store| {
        id_store
            .borrow_mut()
            .insert(id.clone(), principal_id);
    });

    if phone_number.is_some() {
        let phone_number = phone_number.unwrap();
        PHONE_NUMBER_STORE.with(|phone_number_store| {
            phone_number_store
                .borrow_mut()
                .insert(phone_number, principal_id);
        });
    }

    if email_address.is_some() {
        let email_address = email_address.unwrap();
        EMAIL_ADDRESS_STORE.with(|email_address_store| {
            email_address_store
                .borrow_mut()
                .insert(email_address, principal_id);
        });
    }

    let profile = Profile {
        phone_number,
        email_address,
        first_name: None,
        last_name: None,
    };
    PROFILE_STORE.with(|profile_store| {
        profile_store.borrow_mut().insert(principal_id, profile);
    });

    return true;
}

#[update]
fn register(id: String, first_name: Option<String>, last_name: Option<String>, phone_number: Option<String>, email_address: Option<String>) -> bool {
    let res =  check_id(id.clone());
    if !res {
        return false;
    }
    let principal = ID_STORE.with(|id_store| {
        id_store.borrow().get(&id).unwrap()
    });
    let profile = PROFILE_STORE.with(|profile_store| {
        profile_store.borrow().get_mut(principal).unwrap_or_default()
    });

    profile.first_name =  first_name;
    profile.last_name = last_name;
    profile.phone_number = phone_number;
    profile.email_address = email_address;

    true
}

#[update]
fn authenticate(id: String) -> bool {
    return true;
}