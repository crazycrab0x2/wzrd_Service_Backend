use ic_cdk::{query, update};
// use ic_cdk::api::time;
// use std::vec::Vec;
// use std::io::Write;
mod id_utils;
mod chat_utils;
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

// chat service

#[update(name = "CreateGroup")]
pub fn create_group(id: String, group_id: String, group_name: String, group_description: Option<String>) -> String {
    chat_utils::create_group(id, group_id, group_name, group_description)
}

#[update(name = "JoinGroup")]
pub fn join_group(id: String, group_id: String) -> String {
    chat_utils::join_group(id, group_id)
}

#[update(name = "LeaveGroup")]
pub fn leave_group(id: String, group_id: String) -> String {
    chat_utils::leave_group(id, group_id)
}

#[query(name = "GetGroupMembers")]
pub fn get_group_members(group_id: String) -> Vec<String> {
    chat_utils::get_group_members(group_id)
}

#[query(name = "GetJoinedGroup")]
pub fn get_group_list(id: String) -> Vec<String> {
    chat_utils::get_group_list(id)
}

#[query(name = "GetGoupMessage")]
pub fn get_group_messages(group_id: String) -> Vec<chat_utils::Message> {
    chat_utils::get_group_messages(group_id)
}

#[update(name = "SendGroupMessage")]
pub fn send_group_message(id: String, group_id: String, receiver_id: Option<String>, reply_id: Option<String>, content: String) -> String {
    chat_utils::send_group_message(id, group_id, receiver_id, reply_id, content)
}


// fn generate_random_number() -> u64 {
//     let current_time = time();
//     let pseudo_random_number = current_time % u64::MAX;

//     pseudo_random_number
// }

// fn make_challenge() -> String {
//     let challenge = generate_random_number();

//     challenge.to_string()
// }

// fn derive_key(password: &str, salt: &[u8]) -> Vec<u8> {
//     let mut key = vec![0; 32]; // 256 bits
    
//     key
// }

// fn encrypt(data: &str, key: &[u8], iv: &[u8]) -> Vec<u8> {
//     // let mut encryptor = cbc_encryptor(KeySize::KeySize256, key, iv, PkcsPadding);
//     let mut buffer:Vec<u8>  = Vec::new();
//     buffer.write(data.as_bytes()).unwrap();
//     let mut final_result:Vec<u8>  = Vec::new();
//     // loop {
//     //     let mut read_buffer = [0; 4096];
//     //     let result = buffer.read(&mut read_buffer).unwrap();
//     //     // encryptor
//     //     //     .crypt(&read_buffer[..result], &mut final_result)
//     //     //     .unwrap();
//     //     if result == 0 {
//     //         break;
//     //     }
//     // }
//     final_result
// }

// fn decrypt(data: &[u8], key: &[u8]) -> String {

//     let min_length = data.len().min(key.len());

//     let result:Vec<u8> = data
//         .iter()
//         .zip(key.iter())
//         .map(|(&a, &b)| a^ b)
//         .collect();

//     let remaining1 = data[min_length..].to_vec();
//     let remaining2 = key[min_length..].to_vec();

//     let final_result = [&result[..], &remaining1[..], &remaining2[..]].concat();

//     String::from_utf8(final_result).unwrap()
// }

// #[query(name = "loginRequest")]
// pub fn login_request() -> String {
//     make_challenge()
// }









// #[update]
// pub fn register_with_challenge(
//     challenge: String,
//     id: String,
//     first_name: Option<String>,
//     last_name: Option<String>,
//     phone_number: Option<String>,
//     email_address: Option<String>,
// ) -> bool {
//     let _id = decrypt(id.as_bytes(), challenge.as_bytes());
//     let _first_name = if first_name.is_some() {
//         decrypt(first_name.unwrap_or(String::new()).as_bytes(), challenge.as_bytes())
//     } else {
//         String::new()
//     };
        
//     let _last_name = if last_name.is_some() {
//         decrypt(last_name.unwrap_or(String::new()).as_bytes(), challenge.as_bytes())
//     } else {
//         String::new()
//     };

//     let _phone_number = if phone_number.is_some() {
//         decrypt(phone_number.unwrap_or(String::new()).as_bytes(), challenge.as_bytes())
//     } else {
//         String::new()
//     };

//     let _email_address = if email_address.is_some() {
//         decrypt(email_address.unwrap_or(String::new()).as_bytes(), challenge.as_bytes())
//     } else {
//         String::new()
//     };

//     register(_id, Some(_first_name), Some(_last_name), Some(_phone_number), Some(_email_address))
// }



// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     #[test]
//     fn test_create_profile() {
//         let id = String::from("abcde");
//         let user = Principal::anonymous();

//         create_profile(
//             id,
//             user,
//             Some(String::from("john")),
//             Some(String::from("do")),
//             Some(String::from("12345")),
//             Some(String::from("john@mail.com")),
//         )
//         .unwrap();

//         // check id
//         assert_eq!(util::has_id(String::from("abcde")), true);
//         assert_eq!(util::has_id(String::from("wrong")), false);

//         // check phone_number
//         assert_eq!(has_phone_number(String::from("12345")), true);
//         assert_eq!(has_phone_number(String::from("12252")), false);

//         // check email_address
//         assert_eq!(has_email_address(String::from("john@mail.com")), true);
//         assert_eq!(has_email_address(String::from("jahn@mail.com")), false);
//     }
// }
