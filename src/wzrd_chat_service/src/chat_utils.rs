use candid::{Deserialize, Principal};
use ic_cdk::export::candid::CandidType;
use ic_cdk::api::time;
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct GroupMessage {
    pub id: String,
    pub sender_id: String,
    pub reply_id: Option<String>,
    pub content: String,
    pub timestamp: String
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct DirectMessage {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub reply_id: Option<String>,
    pub content: String,
    pub timestamp: String,
    pub viewed: bool
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct Group {
    pub group_id: String,
    pub group_name: String,
    pub group_description: Option<String>,
    pub group_members: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct CreateGroupParams {
    pub user_name: String, 
    pub group_id: String, 
    pub group_name: String, 
    pub group_description: Option<String>
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct LeaveGroupParams {
    pub user_name: String, 
    pub group_id: String
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct JoinGroupParam {
    pub user_name: String, 
    pub group_id: String
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct SendGroupMessageParam {
    pub user_name: String, 
    pub group_id: String, 
    pub reply_id: Option<String>,
    pub content: String
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct SendDirectMessageParam {
    pub sender: String, 
    pub receiver: String, 
    pub reply_id: Option<String>,
    pub content: String
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct GetDirectMessageParam {
    pub sender: String, 
    pub receiver: String
}

type GroupMessageStore = BTreeMap<String, Vec<GroupMessage>>;
type DirectMessageStore = Vec<DirectMessage>;
type GroupStore = Vec<Group>;
type UserGroupStore = BTreeMap<String, Vec<String>>;
type UserFriendStore = BTreeMap<String, Vec<String>>;

thread_local! {
    pub static GROUP_MESSAGE_STORE: RefCell<GroupMessageStore> = RefCell::default();
    pub static DIRECT_MESSAGE_STORE: RefCell<DirectMessageStore> = RefCell::default();
    pub static GROUP_STORE: RefCell<GroupStore> = RefCell::default();
    pub static USER_GROUP_STORE: RefCell<UserGroupStore> = RefCell::default();
    pub static USER_FRIEND_STORE: RefCell<UserFriendStore> = RefCell::default();
}

pub async fn create_group(
    params: CreateGroupParams
) -> String {

    let user_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("urpxs-4aaaa-aaaap-qb6mq-cai").unwrap(), "CheckUser", (params.user_name.clone(),)).await;
    match user_validation {
        Err(_err) => {
            return "Can't access store".to_string();
        }
        Ok(result) => {
            let (id_valid,): (bool,) = result;
            if !id_valid {
                return "Invalid User ID!".to_string();
            }
            if has_group_id(params.group_id.clone()) {
                return "Group ID Alreay Exist".to_string();
            }
            GROUP_STORE.with(|group_store| {
                let new_group = Group{
                    group_id: params.group_id.clone(),
                    group_name: params.group_name,
                    group_description: params.group_description,
                    group_members: vec![params.user_name.clone()],
                };
                group_store.borrow_mut().push(new_group);
            });
            USER_GROUP_STORE.with(|user_group_store| {
                let mut new_group_list;
                if user_group_store.borrow().get(&params.user_name).is_none() {
                    new_group_list = vec![];
                } else {
                    new_group_list = user_group_store.borrow().get(&params.user_name).unwrap().clone();
                }
                new_group_list.push(params.group_id);
                user_group_store.borrow_mut().insert(params.user_name, new_group_list);
            });
            "Success!".to_string()
        }
    }    
}

pub async fn join_group(
    params: JoinGroupParam
) -> String {
    let user_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("urpxs-4aaaa-aaaap-qb6mq-cai").unwrap(), "CheckUser", (params.user_name.clone(),)).await;
    match user_validation {
        Err(_err) => {
            return "Can't access store".to_string();
        }
        Ok(result) => {
            let (id_valid,): (bool,) = result;
            if !id_valid {
                return "Invalid User ID!".to_string();
            }
            if !has_group_id(params.group_id.clone()) {
                return "Invalid Group ID".to_string();
            }
            GROUP_STORE.with(|group_store| {
                if let Some(group) = group_store.borrow_mut().iter_mut().find(|group| *group.group_id == params.group_id){
                    let mut new_members = group.group_members.clone();
                    if !new_members.iter().find(|&member| *member == params.user_name).is_some(){
                        new_members.push(params.user_name.clone());
                    }
                    group.group_members = new_members;
                }
            });
            USER_GROUP_STORE.with(|user_group_store| {
                let mut new_group_list;
                if user_group_store.borrow().get(&params.user_name).is_none() {
                    new_group_list = vec![];
                } else {
                    new_group_list = user_group_store.borrow().get(&params.user_name).unwrap().clone();
                }
                if !new_group_list.iter().find(|&group| *group == params.group_id).is_some(){
                    new_group_list.push(params.group_id);
                }
                user_group_store.borrow_mut().insert(params.user_name, new_group_list);
            });
            "Success!".to_string()
        }
    }
}

pub async fn leave_group(
    params: LeaveGroupParams
) -> String {
    let user_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("urpxs-4aaaa-aaaap-qb6mq-cai").unwrap(), "CheckUser", (params.user_name.clone(),)).await;
    match user_validation {
        Err(_err) => {
            return "Can't access store".to_string();
        }
        Ok(result) => {
            let (id_valid,): (bool,) = result;
            if !id_valid {
                return "Invalid User ID!".to_string();
            }
            if !has_group_id(params.group_id.clone()) {
                return "Invalid Group ID".to_string();
            }
            GROUP_STORE.with(|group_store| {
                if let Some(group) = group_store.borrow_mut().iter_mut().find(|group| *group.group_id == params.group_id){
                    let mut new_members = group.group_members.clone();
                    new_members.retain(|member| *member != params.user_name);
                    group.group_members = new_members;
                }
            });
            USER_GROUP_STORE.with(|user_group_store| {
                let mut new_group_list;
                if !user_group_store.borrow().get(&params.user_name).is_none() {
                    new_group_list = user_group_store.borrow().get(&params.user_name).unwrap().clone();
                    new_group_list.retain(|groupid| *groupid != params.group_id);
                    user_group_store.borrow_mut().insert(params.user_name, new_group_list);
                }
            });
            "Success!".to_string()
        }
    }
}                                      

pub fn get_group_members(
    group_id: String
) -> Vec<String> {
    let res = has_group_id(group_id.clone());
    if !res {
        return vec![];
    }
    GROUP_STORE.with(|group_store| {
        group_store.borrow().iter().find(|&group| *group.group_id == group_id).unwrap().clone().group_members
    })
}

pub async fn get_group_list(
    user_name: String
) -> Vec<String> {
    let user_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("urpxs-4aaaa-aaaap-qb6mq-cai").unwrap(), "CheckUser", (user_name.clone(),)).await;
    match user_validation {
        Err(_err) => {
            return vec![];
        }
        Ok(result) => {
            let (id_valid,): (bool,) = result;
            if !id_valid {
                return vec![];
            }
            USER_GROUP_STORE.with(|user_group_store| {
                user_group_store.borrow().get(&user_name).unwrap_or(&vec![]).clone()
            })
        }
    }
}

pub fn get_group_messages(
    group_id: String
) -> Vec<GroupMessage> {
    let res = has_group_id(group_id.clone());
    if !res {
        vec![]
    }
    else{
        GROUP_MESSAGE_STORE.with(|group_message_store| {
            let empty = vec![];
            group_message_store.borrow().get(&group_id).unwrap_or(&empty).clone()
        })
    }
}

pub async fn send_group_message(
    params: SendGroupMessageParam
) -> String {
    let user_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("urpxs-4aaaa-aaaap-qb6mq-cai").unwrap(), "CheckUser", (params.user_name.clone(),)).await;
    match user_validation {
        Err(_err) => {
            return "Can't access store".to_string();
        }
        Ok(result) => {
            let (id_valid,): (bool,) = result;
            if !id_valid {
                return "Invalid User ID!".to_string();
            } 
            if !has_group_id(params.group_id.clone()) {
                return "Invalid Group ID".to_string();
            }
            GROUP_MESSAGE_STORE.with(|group_message_store| {
                let mut new_message_list;
                let message_id;
                if group_message_store.borrow().get(&params.group_id).is_none() {
                    new_message_list = vec![];
                    message_id = 0;
                }
                else{
                    new_message_list = group_message_store.borrow().get(&params.group_id).unwrap().clone();
                    message_id = new_message_list.len();
                }
                let message = GroupMessage { 
                    id: message_id.to_string(), 
                    sender_id: params.user_name, 
                    reply_id: params.reply_id,
                    content: params.content, 
                    timestamp: (time()/1000000).to_string()
                };
                new_message_list.push(message);
                group_message_store.borrow_mut().insert(params.group_id, new_message_list);
            });
            "Success!".to_string()
        }
    }
}

pub async fn send_direct_message(
    params: SendDirectMessageParam
) -> String {
    let sender_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("urpxs-4aaaa-aaaap-qb6mq-cai").unwrap(), "CheckUser", (params.sender.clone(),)).await;
    match sender_validation {
        Err(_err) => {
            return "Can't access store".to_string();
        }
        Ok(result) => {
            let (id_valid,): (bool,) = result;
            if !id_valid {
                return "Invalid User ID!".to_string();
            }
            let receiver_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("urpxs-4aaaa-aaaap-qb6mq-cai").unwrap(), "CheckUser", (params.receiver.clone(),)).await;
            match receiver_validation {
                Err(_err) => {
                    return "Can't access store".to_string();
                }
                Ok(result) => {
                    let (id_valid,): (bool,) = result;
                    if !id_valid {
                        return "Invalid User ID!".to_string();
                    }
                    DIRECT_MESSAGE_STORE.with(|direct_message_store| {
                        let message = DirectMessage { 
                            id: direct_message_store.borrow().clone().len().to_string(), 
                            sender_id: params.sender.clone(), 
                            receiver_id: params.receiver.clone(), 
                            reply_id: params.reply_id,
                            content: params.content, 
                            timestamp: (time()/1000000).to_string(), 
                            viewed: false 
                        };
                        direct_message_store.borrow_mut().push(message);
                    });
                    USER_FRIEND_STORE.with(|user_friend_store| {
                        let mut sender_friend_list;
                        let mut receiver_friend_list;
                        if user_friend_store.borrow().get(&params.sender).is_none() {
                            sender_friend_list = vec![];
                        } else {
                            sender_friend_list = user_friend_store.borrow().get(&params.sender).unwrap().clone();
                        }
                        if !sender_friend_list.iter().find(|&id| *id == params.receiver).is_some(){
                            sender_friend_list.push(params.receiver.clone());
                        }
                        user_friend_store.borrow_mut().insert(params.sender.clone(), sender_friend_list);

                        if user_friend_store.borrow().get(&params.receiver).is_none() {
                            receiver_friend_list = vec![];
                        } else {
                            receiver_friend_list = user_friend_store.borrow().get(&params.receiver).unwrap().clone();
                        }
                        if !receiver_friend_list.iter().find(|&id| *id == params.sender).is_some(){
                            receiver_friend_list.push(params.sender);
                        }
                        user_friend_store.borrow_mut().insert(params.receiver, receiver_friend_list);
                    });
                    "Success!".to_string()
                }
            }
        }
    }
}

pub async fn get_friend_list(
    user_name: String
) -> Vec<String> {
    let sender_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("urpxs-4aaaa-aaaap-qb6mq-cai").unwrap(), "CheckUser", (user_name.clone(),)).await;
    match sender_validation {
        Err(_err) => {
            return vec![];
        }
        Ok(result) => {
            let (id_valid,): (bool,) = result;
            if !id_valid {
                return vec![];
            }
            USER_FRIEND_STORE.with(|user_friend_store| {
                user_friend_store.borrow().get(&user_name).unwrap_or(&vec![]).clone()
            })
        }
    }
}

pub async fn get_direct_messages(
    params: GetDirectMessageParam
) -> Vec<DirectMessage> {
    let sender_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("urpxs-4aaaa-aaaap-qb6mq-cai").unwrap(), "CheckUser", (params.sender.clone(),)).await;
    match sender_validation {
        Err(_err) => {
            return vec![];
        }
        Ok(result) => {
            let (id_valid,): (bool,) = result;
            if !id_valid {
                return vec![];
            }
            let receiver_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("urpxs-4aaaa-aaaap-qb6mq-cai").unwrap(), "CheckUser", (params.receiver.clone(),)).await;
            match receiver_validation {
                Err(_err) => {
                    return vec![];
                }
                Ok(result) => {
                    let (id_valid,): (bool,) = result;
                    if !id_valid {
                        return vec![];
                    }
                    // user friend store...
                    DIRECT_MESSAGE_STORE.with(|direct_message_store| {
                        let dms = direct_message_store.borrow().clone();
                        dms.iter().filter(|&message| 
                            message.sender_id == params.sender && 
                            message.receiver_id == params.receiver || 
                            message.sender_id == params.receiver && 
                            message.receiver_id == params.sender)
                        .map(|msg|{
                            let tmp = msg.clone();
                            tmp
                        }).collect()
                    })
                }
            }
        }
    }
}

pub fn view_message(
    message_id: String
) -> bool {
    DIRECT_MESSAGE_STORE.with(|direct_message_store| {
        if let Some(message) = direct_message_store.borrow_mut().iter_mut().find(|message| message.id == message_id){
            message.viewed = true;
            true
        }
        else{
            false
        }
    })
}


pub fn has_group_id(
    id: String
) -> bool {
    GROUP_STORE.with(|group_store| {
        group_store.borrow().iter().find(|group| *group.group_id == id).is_some()
    })
}

