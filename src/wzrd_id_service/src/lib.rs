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
pub fn register_request(user_name: String) -> String {
    id_utils::register_request(user_name)
}

#[update(name = "Register")]
pub fn register(params: id_utils::RegisterParams) -> String {
    id_utils::register(params)
}

#[query(name = "AuthenticationRequest")]
pub fn authentication_request(user_name: String) -> String {
    id_utils::authentication_request(user_name)
}

#[query(name = "Authentication")]
pub fn authentication(params: id_utils::AuthenticationParams) -> String {
    id_utils::authentication(params)
}

#[query(name = "CheckUser")]
pub fn check_user(user_name: String) -> bool {
    id_utils::has_user(&user_name)
}

#[query(name = "CheckToken")]
pub fn check_token(token: String) -> String {
    id_utils::check_token(token)
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
pub fn set_profile(params: id_utils::SetProfileParams) -> bool {
    id_utils::set_profile(params)
}
