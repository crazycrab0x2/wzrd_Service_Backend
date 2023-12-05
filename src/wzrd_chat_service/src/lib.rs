use ic_cdk::{query, update};
// use ic_cdk::api::time;
// use std::vec::Vec;
// use std::io::Write;
mod chat_utils;
// use crypto::aes::{ebc_decryptor, ebc_encryptor, KeySize};
// use crypto::blockmodes::PkcsPadding;
// use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
// use crypto::digest::Digest;
// use crypto::pbkdf2::pbkdf2;
// use rand_core::{RngCore, OsRng};
// use rand::Rng;

#[update(name = "CreateGroup")]
pub async fn create_group(id: String, group_id: String, group_name: String, group_description: Option<String>) -> String {
    chat_utils::create_group(id, group_id, group_name, group_description).await
}

#[update(name = "JoinGroup")]
pub async fn join_group(id: String, group_id: String) -> String {
    chat_utils::join_group(id, group_id).await
}

#[update(name = "LeaveGroup")]
pub async fn leave_group(id: String, group_id: String) -> String {
    chat_utils::leave_group(id, group_id).await
}

#[query(name = "GetGroupMembers")]
pub fn get_group_members(group_id: String) -> Vec<String> {
    chat_utils::get_group_members(group_id)
}

#[update(name = "GetJoinedGroup")]
pub async fn get_group_list(id: String) -> Vec<String> {
    chat_utils::get_group_list(id).await
}

#[query(name = "GetGoupMessage")]
pub fn get_group_messages(group_id: String) -> Vec<chat_utils::GroupMessage> {
    chat_utils::get_group_messages(group_id)
}

#[update(name = "SendGroupMessage")]
pub async fn send_group_message(id: String, group_id: String, reply_id: Option<String>, content: String) -> String {
    chat_utils::send_group_message(id, group_id, reply_id, content).await
}

#[update(name = "SendDirectMessage")]
pub async fn send_direct_message(id: String, receiver_id: String, reply_id: Option<String>, content: String) -> String {
    chat_utils::send_direct_message(id, receiver_id, reply_id, content).await
}

#[update(name = "GetConnectedMembers")]
pub async fn get_friend_list(id:String) -> Vec<String> {
    chat_utils::get_friend_list(id).await
}

#[update(name = "GetDirectMessages")]
pub async fn get_friend_messages(sender_id: String, receiver_id: String) -> Vec<chat_utils::DirectMessage> {
    chat_utils::get_friend_messages(sender_id, receiver_id).await
}