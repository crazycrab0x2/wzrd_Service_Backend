use ic_cdk::{query, update};
mod chat_utils;
// use ic_cdk::api::time;
// use std::vec::Vec;
// use std::io::Write;
// use crypto::aes::{ebc_decryptor, ebc_encryptor, KeySize};
// use crypto::blockmodes::PkcsPadding;
// use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
// use crypto::digest::Digest;
// use crypto::pbkdf2::pbkdf2;
// use rand_core::{RngCore, OsRng};
// use rand::Rng;
#[update(name = "CreateGroup")]
pub async fn create_group(params: chat_utils::CreateGroupParams) -> String {
    chat_utils::create_group(params).await
}

#[update(name = "JoinGroup")]
pub async fn join_group(params: chat_utils::JoinGroupParam) -> String {
    chat_utils::join_group(params).await
}

#[update(name = "LeaveGroup")]
pub async fn leave_group(params: chat_utils::LeaveGroupParams) -> String {
    chat_utils::leave_group(params).await
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
pub async fn send_group_message(params: chat_utils::SendGroupMessageParam) -> String {
    chat_utils::send_group_message(params).await
}

#[update(name = "SendDirectMessage")]
pub async fn send_direct_message(params: chat_utils::SendDirectMessageParam) -> String {
    chat_utils::send_direct_message(params).await
}

#[update(name = "GetConnectedMembers")]
pub async fn get_friend_list(id:String) -> Vec<String> {
    chat_utils::get_friend_list(id).await
}

#[update(name = "GetDirectMessages")]
pub async fn get_direct_messages(params: chat_utils::GetDirectMessageParam) -> Vec<chat_utils::DirectMessage> {
    chat_utils::get_direct_messages(params).await
}

#[update(name = "ViewMessage")]
pub fn view_message(message_id: String) -> String {
    chat_utils::view_message(message_id)
}