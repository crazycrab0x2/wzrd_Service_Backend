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

#[query(name = "RegisterRequest")]
pub fn register_request(user_name: String) -> id_utils::RegisterRequestData {
    id_utils::register_request(user_name)
}

#[update(name = "Register")]
pub fn register(user_name: String, key_id: String, public_key: String) -> bool {
    id_utils::register(user_name, key_id, public_key)
}

#[query(name = "AuthenticationRequest")]
pub fn authentication_request(user_name: String) -> id_utils::AuthenticationRequestData {
    id_utils::authentication_request(user_name)
}

#[query(name = "Authentication")]
pub fn authentication(user_name: String, key_id: String, authenticator_data: String, signature: String) -> bool {
    id_utils::authentication(user_name, key_id, authenticator_data, signature)
}

#[query(name = "CheckUser")]
pub fn check_user(user_name: String) -> bool {
    id_utils::has_user(&user_name)
}

#[query(name = "GetPrincipal")]
pub fn get_principal() -> String {
    ic_cdk::caller().to_string()
}

#[query(name = "GetProfile")]
pub fn get_profile(user_name: String) -> id_utils::Profile {
    id_utils::get_profile(user_name)
}

#[update(name = "SetProfile")]
pub fn set_profile(user_name: String, first_name: Option<String>, last_name: Option<String>, email_address: Option<String>, phone_number: Option<String>) -> bool {
    id_utils::set_profile(user_name, first_name, last_name, email_address, phone_number)
}
