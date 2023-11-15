use ic_cdk::{
    export::{candid::CandidType, Principal},
    query, update,
};
use std::cell::RefCell;
use std::collections::BTreeMap;

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

fn has_id(id: &String) -> bool {
    ID_STORE.with(|id_store| {
        let binding = id_store.borrow();
        binding.get(id).is_some()
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

#[query(name = "checkId")]
pub fn check_id(id: String) -> bool {
    has_id(&id)
}

#[query(name = "getProfile")]
pub fn get_profile(id: String) -> Profile {
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

#[update]
pub fn register(
    id: String,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
    email_address: Option<String>,
) -> bool {
    let res = check_id(id.clone());
    if !res {
        return false;
    }
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

    true
}

#[update]
fn authenticate(_id: String) -> bool {
    return true;
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
