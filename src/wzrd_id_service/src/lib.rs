use ic_cdk::{query, update};
// use ic_cdk::api::time;
// use std::vec::Vec;
// use std::io::Write;
mod id_utils;
// use crypto::aes::{ebc_decryptor, ebc_encryptor, KeySize};
// use crypto::blockmodes::PkcsPadding;
// use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
// use crypto::digest::Digest;
// use crypto::pbkdf2::pbkdf2;
// use rand_core::{RngCore, OsRng};
// use rand::Rng;

#[update(name = "SetPasskey")]
pub fn set_passkey(id: String, passkey: String) -> bool{
    id_utils::set_passkey(id, passkey)
}

#[query(name = "GetPasskey")]
pub fn get_passkey(id: String) -> String {
    id_utils::get_passkey(id)
}

#[query(name = "checkId")]
pub fn check_id(id: String) -> bool {
    id_utils::has_id(&id)
}

#[query(name = "getPrincipal")]
pub fn get_principal(id: String) -> String {
    // ic_cdk::caller().to_string()
    id_utils::ID_STORE.with(|id_store| id_store.borrow().get(&id).unwrap().clone().to_string())
}

#[query(name="Login")]
fn login(id: String, passkey: String) -> bool {
    // let id = decrypt(_id.as_bytes(), challenge.as_bytes());
    // util::has_id(id)
    id_utils::login(id, passkey)
}

#[update(name = "Register")]
pub fn register(
    id: String,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
    email_address: Option<String>,
) -> bool {
    let res = id_utils::has_id(&id);
    if res {
        return false;
    }

    let caller = ic_cdk::caller();

    let _ = id_utils::add_id(id.clone(), caller);
    let _ = id_utils::add_phone_number(phone_number.clone(), caller);
    let _ = id_utils::add_email_address(email_address.clone(), caller);

    id_utils::create_profile(
        id.clone(),
        caller,
        first_name,
        last_name,
        phone_number,
        email_address,
    )
    .unwrap();
    true
}

#[update(name = "reserveID")]
pub fn reserve_id(id: String, phone_number: String, email_address: String) -> bool {
    if id_utils::has_id(&id) {
        return false;
    }

    if id_utils::has_phone_number(&phone_number) {
        return false;
    }

    if id_utils::has_email_address(&email_address) {
        return false;
    }

    let caller = ic_cdk::caller();

    id_utils::create_profile(
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

#[query(name = "getProfile")]
pub fn get_profile(id: String) -> id_utils::Profile {
    id_utils::get_profile(id)
}
