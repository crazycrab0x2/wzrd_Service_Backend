use ic_cdk::export::candid::CandidType;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::id_utils;

#[derive(Clone, Debug, Default, CandidType)]
pub struct Message {
    pub id: usize,
    pub sender_id: String,
    pub receiver_id: Option<String>,
    pub reply_id: Option<String>,
    pub timestamp: String,
    pub viewed: bool,
}

#[derive(Clone, Debug, Default, CandidType)]
pub struct Group {
    pub group_id: String,
    pub group_name: String,
    pub group_description: Option<String>,
    pub group_members: Vec<String>,
}

type GroupMessageStore = BTreeMap<String, Vec<Message>>;
type GroupStore = Vec<Group>;
type UserGroupStore = BTreeMap<String, Vec<String>>;
type UserFriendStore = BTreeMap<String, Vec<String>>;

thread_local! {
    pub static GROUP_MESSAGE_STORE: RefCell<GroupMessageStore> = RefCell::default();
    pub static GROUP_STORE: RefCell<GroupStore> = RefCell::default();
    pub static USER_GROUP_STORE: RefCell<UserGroupStore> = RefCell::default();
    pub static USER_FRIEND_STORE: RefCell<UserFriendStore> = RefCell::default();
}

pub fn create_group(id: String, group_id: String, group_name: String, group_description: Option<String>) -> String {
    let mut res = id_utils::has_id(&id);
    if !res {
        return "Invalid User ID".to_string();
    }
    res = has_group_id(group_id.clone());
    if res {
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

pub fn join_group(id: String, group_id: String) -> String {
    let mut res = id_utils::has_id(&id);
    if !res {
        return "Invalid User ID".to_string();
    }
    res = has_group_id(group_id.clone());
    if !res {
        return "Invalid Group ID".to_string();
    }
    GROUP_STORE.with(|group_store| {
        if let Some(group) = group_store.borrow_mut().iter_mut().find(|group| *group.group_id == group_id){
            let mut new_members = group.group_members.clone();
            new_members.push(id.clone());
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
        new_group_list.push(group_id);
        user_group_store.borrow_mut().insert(id, new_group_list);
    });
    "Success!".to_string()
}

pub fn leave_group(id: String, group_id: String) -> String {
    let mut res = id_utils::has_id(&id);
    if !res {
        return "Invalid User ID".to_string();
    }
    res = has_group_id(group_id.clone());
    if !res {
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

pub fn get_group_members(group_id: String) -> Vec<String> {
    let res = has_group_id(group_id.clone());
    if !res {
        return vec![];
    }
    GROUP_STORE.with(|group_store| {
        group_store.borrow().iter().find(|&group| *group.group_id == group_id).unwrap().clone().group_members
    })
}

pub fn get_group_list(id: String) -> Vec<String> {
    USER_GROUP_STORE.with(|user_group_store| {
        user_group_store.borrow().get(&id).unwrap_or(&vec![]).clone()
    })
}

pub fn get_group_messages(group_id: String) -> Vec<Message> {
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

pub fn send_group_message(id: String, group_id: String, receiver_id: Option<String>, reply_id: Option<String>) -> String {
    let mut res = id_utils::has_id(&id);
    if !res {
        return "Invalid User ID".to_string();
    }
    if receiver_id.is_some(){
        res = id_utils::has_id(&receiver_id.clone().unwrap());
        if !res {
            return "Invalid Receiver ID".to_string();
        }
    }
    res = has_group_id(group_id.clone());
    if !res {
        return "Invalid Group ID".to_string();
    }
    GROUP_MESSAGE_STORE.with(|group_message_store| {
        let mut new_message_list;
        let messgae_id;
        if group_message_store.borrow().get(&group_id).is_none() {
            new_message_list = vec![];
            messgae_id = 0;
        }
        else{
            new_message_list = group_message_store.borrow().get(&group_id).unwrap().clone();
            messgae_id = new_message_list.len();
        }
        let message = Message { 
            id: messgae_id, 
            sender_id: id, 
            receiver_id: receiver_id, 
            reply_id: reply_id, 
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).expect("None").as_secs().to_string(), 
            viewed: false 
        };
        new_message_list.push(message);
        group_message_store.borrow_mut().insert(group_id, new_message_list);
    });
    "Success!".to_string()
}

pub fn send_friend_message() -> bool {true}

pub fn view_message() -> bool {true}

pub fn get_friend_messages() -> bool {true}

pub fn get_friend_list() -> bool {true}

pub fn has_group_id(id: String) -> bool {
    GROUP_STORE.with(|group_store| {
        group_store.borrow().iter().find(|group| *group.group_id == id).is_some()
    })
}

