use candid::Principal;
use ic_cdk::export::candid::CandidType;
use ic_cdk::api::time;
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, CandidType)]
pub struct GroupMessage {
    pub id: String,
    pub sender_id: String,
    pub reply_id: Option<String>,
    pub content: String,
    pub timestamp: String
}

#[derive(Clone, Debug, Default, CandidType)]
pub struct DirectMessage {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub reply_id: Option<String>,
    pub content: String,
    pub timestamp: String,
    pub viewed: bool
}

#[derive(Clone, Debug, Default, CandidType)]
pub struct Group {
    pub group_id: String,
    pub group_name: String,
    pub group_description: Option<String>,
    pub group_members: Vec<String>,
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
    id: String, 
    group_id: String, 
    group_name: String, 
    group_description: Option<String>
) -> String {

    let user_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap(), "CheckUser", (id.clone(),)).await;
    match user_validation {
        Err(_err) => {
            return "Can't access store".to_string();
        }
        Ok(result) => {
            let (id_valid,): (bool,) = result;
            if !id_valid {
                return "Invalid User ID!".to_string();
            }
            if has_group_id(group_id.clone()) {
                return "Group ID Alreay Exist".to_string();
            }
            GROUP_STORE.with(|group_store| {
                let new_group = Group{
                    group_id: group_id.clone(),
                    group_name,
                    group_description,
                    group_members: vec![id.clone()],
                };
                group_store.borrow_mut().push(new_group);
            });
            USER_GROUP_STORE.with(|user_group_store| {
                let mut new_group_list;
                if user_group_store.borrow().get(&id).is_none() {
                    new_group_list = vec![];
                } else {
                    new_group_list = user_group_store.borrow().get(&id).unwrap().clone();
                }
                new_group_list.push(group_id);
                user_group_store.borrow_mut().insert(id, new_group_list);
            });
            "Success!".to_string()
        }
    }    
}

pub async fn join_group(
    id: String, 
    group_id: String
) -> String {
    let user_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap(), "CheckUser", (id.clone(),)).await;
    match user_validation {
        Err(_err) => {
            return "Can't access store".to_string();
        }
        Ok(result) => {
            let (id_valid,): (bool,) = result;
            if !id_valid {
                return "Invalid User ID!".to_string();
            }
            if !has_group_id(group_id.clone()) {
                return "Invalid Group ID".to_string();
            }
            GROUP_STORE.with(|group_store| {
                if let Some(group) = group_store.borrow_mut().iter_mut().find(|group| *group.group_id == group_id){
                    let mut new_members = group.group_members.clone();
                    if !new_members.iter().find(|&member| *member == id).is_some(){
                        new_members.push(id.clone());
                    }
                    group.group_members = new_members;
                }
            });
            USER_GROUP_STORE.with(|user_group_store| {
                let mut new_group_list;
                if user_group_store.borrow().get(&id).is_none() {
                    new_group_list = vec![];
                } else {
                    new_group_list = user_group_store.borrow().get(&id).unwrap().clone();
                }
                if !new_group_list.iter().find(|&group| *group == group_id).is_some(){
                    new_group_list.push(group_id);
                }
                user_group_store.borrow_mut().insert(id, new_group_list);
            });
            "Success!".to_string()
        }
    }
}

pub async fn leave_group(
    id: String, 
    group_id: String
) -> String {
    let user_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap(), "CheckUser", (id.clone(),)).await;
    match user_validation {
        Err(_err) => {
            return "Can't access store".to_string();
        }
        Ok(result) => {
            let (id_valid,): (bool,) = result;
            if !id_valid {
                return "Invalid User ID!".to_string();
            }
            if !has_group_id(group_id.clone()) {
                return "Invalid Group ID".to_string();
            }
            GROUP_STORE.with(|group_store| {
                if let Some(group) = group_store.borrow_mut().iter_mut().find(|group| *group.group_id == group_id){
                    let mut new_members = group.group_members.clone();
                    new_members.retain(|member| *member != id);
                    group.group_members = new_members;
                }
            });
            USER_GROUP_STORE.with(|user_group_store| {
                let mut new_group_list;
                if !user_group_store.borrow().get(&id).is_none() {
                    new_group_list = user_group_store.borrow().get(&id).unwrap().clone();
                    new_group_list.retain(|groupid| *groupid != group_id);
                    user_group_store.borrow_mut().insert(id, new_group_list);
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
    id: String
) -> Vec<String> {
    let user_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap(), "CheckUser", (id.clone(),)).await;
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
                user_group_store.borrow().get(&id).unwrap_or(&vec![]).clone()
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
    id: String, 
    group_id: String, 
    reply_id: Option<String>,
    content: String
) -> String {
    let user_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap(), "CheckUser", (id.clone(),)).await;
    match user_validation {
        Err(_err) => {
            return "Can't access store".to_string();
        }
        Ok(result) => {
            let (id_valid,): (bool,) = result;
            if !id_valid {
                return "Invalid User ID!".to_string();
            } 
            if !has_group_id(group_id.clone()) {
                return "Invalid Group ID".to_string();
            }
            GROUP_MESSAGE_STORE.with(|group_message_store| {
                let mut new_message_list;
                let message_id;
                if group_message_store.borrow().get(&group_id).is_none() {
                    new_message_list = vec![];
                    message_id = 0;
                }
                else{
                    new_message_list = group_message_store.borrow().get(&group_id).unwrap().clone();
                    message_id = new_message_list.len();
                }
                let message = GroupMessage { 
                    id: message_id.to_string(), 
                    sender_id: id, 
                    reply_id,
                    content, 
                    timestamp: (time()/1000000).to_string()
                };
                new_message_list.push(message);
                group_message_store.borrow_mut().insert(group_id, new_message_list);
            });
            "Success!".to_string()
        }
    }
}

pub async fn send_direct_message(
    sender_id: String, 
    receiver_id: String, 
    reply_id: Option<String>,
    content: String
) -> String {
    let sender_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap(), "CheckUser", (sender_id.clone(),)).await;
    match sender_validation {
        Err(_err) => {
            return "Can't access store".to_string();
        }
        Ok(result) => {
            let (id_valid,): (bool,) = result;
            if !id_valid {
                return "Invalid User ID!".to_string();
            }
            let receiver_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap(), "CheckUser", (receiver_id.clone(),)).await;
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
                            sender_id: sender_id.clone(), 
                            receiver_id: receiver_id.clone(), 
                            reply_id,
                            content, 
                            timestamp: (time()/1000000).to_string(), 
                            viewed: false 
                        };
                        direct_message_store.borrow_mut().push(message);
                    });
                    USER_FRIEND_STORE.with(|user_friend_store| {
                        let mut sender_friend_list;
                        let mut receiver_friend_list;
                        if user_friend_store.borrow().get(&sender_id).is_none() {
                            sender_friend_list = vec![];
                        } else {
                            sender_friend_list = user_friend_store.borrow().get(&sender_id).unwrap().clone();
                        }
                        if !sender_friend_list.iter().find(|&id| *id == receiver_id).is_some(){
                            sender_friend_list.push(receiver_id.clone());
                        }
                        user_friend_store.borrow_mut().insert(sender_id.clone(), sender_friend_list);

                        if user_friend_store.borrow().get(&receiver_id).is_none() {
                            receiver_friend_list = vec![];
                        } else {
                            receiver_friend_list = user_friend_store.borrow().get(&receiver_id).unwrap().clone();
                        }
                        if !receiver_friend_list.iter().find(|&id| *id == sender_id).is_some(){
                            receiver_friend_list.push(sender_id);
                        }
                        user_friend_store.borrow_mut().insert(receiver_id, receiver_friend_list);
                    });
                    "Success!".to_string()
                }
            }
        }
    }
}

pub async fn get_friend_list(id: String) -> Vec<String> {
    let sender_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap(), "CheckUser", (id.clone(),)).await;
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
                user_friend_store.borrow().get(&id).unwrap_or(&vec![]).clone()
            })
        }
    }
}

pub async fn get_friend_messages(sender_id:String, receiver_id: String) -> Vec<DirectMessage> {
    let sender_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap(), "CheckUser", (sender_id.clone(),)).await;
    match sender_validation {
        Err(_err) => {
            return vec![];
        }
        Ok(result) => {
            let (id_valid,): (bool,) = result;
            if !id_valid {
                return vec![];
            }
            let receiver_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap(), "CheckUser", (receiver_id.clone(),)).await;
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
                            message.sender_id == sender_id && 
                            message.receiver_id == receiver_id || 
                            message.sender_id == receiver_id && 
                            message.receiver_id == sender_id)
                        .map(|msg|{
                            let tmp = msg.clone();
                            DirectMessage { 
                                id: tmp.id, 
                                sender_id: tmp.sender_id, 
                                receiver_id: tmp.receiver_id, 
                                reply_id: tmp.reply_id, 
                                content: tmp.content, 
                                timestamp: tmp.timestamp, 
                                viewed: tmp.viewed
                            }
                        }).collect()
                    })
                }
            }
        }
    }
}

pub fn view_message(message_id: String) -> String {
    DIRECT_MESSAGE_STORE.with(|direct_message_store| {
        if let Some(message) = direct_message_store.borrow_mut().iter_mut().find(|message| message.id == message_id){
            message.viewed = true;
            return "Success!".to_string();
        }
        else{
            return "Invalid Message ID!".to_string();
        }
    })
}


pub fn has_group_id(id: String) -> bool {
    GROUP_STORE.with(|group_store| {
        group_store.borrow().iter().find(|group| *group.group_id == id).is_some()
    })
}

