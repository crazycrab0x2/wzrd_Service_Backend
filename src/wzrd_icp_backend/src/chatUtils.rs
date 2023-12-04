use ic_cdk::export::candid::CandidType;
use std::cell::RefCell;
use std::collections::BTreeMap;

use crate::idUtils;

#[derive(Clone, Debug, Default, CandidType)]
pub struct Message {
    pub id: String,
    pub sender_id: String,
    pub group_id: Option<String>,
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

type MessageStore = Vec<Message>;
type GroupStore = Vec<Group>;
type UserGroupStore = BTreeMap<String, Vec<String>>;
type UserFriendStore = BTreeMap<String, Vec<String>>;

thread_local! {
    pub static MESSAGE_STORE: RefCell<MessageStore> = RefCell::default();
    pub static GROUP_STORE: RefCell<GroupStore> = RefCell::default();
    pub static USER_GROUP_STORE: RefCell<UserGroupStore> = RefCell::default();
    pub static USER_FRIEND_STORE: RefCell<UserFriendStore> = RefCell::default();
}

pub fn create_group(id: String, group_id: String, group_name: String, group_description: Option<String>) -> String {
    let mut res = idUtils::has_id(&id);
    if !res {
        return "Invalid User ID".to_string();
    }
    res = has_group_id(group_id.clone());
    if !res {
        return "Invalid Group ID".to_string();
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
    let mut res = idUtils::has_id(&id);
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
    let mut res = idUtils::has_id(&id);
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

pub fn get_friend_list() -> bool {true}

pub fn get_group_messages() -> bool {true}

pub fn get_friend_messages() -> bool {true}

pub fn send_group_message() -> bool {true}

pub fn send_friend_message() -> bool {true}

pub fn view_message() -> bool {true}


pub fn has_group_id(id: String) -> bool {
    GROUP_STORE.with(|group_store| {
        group_store.borrow().iter().find(|group| *group.group_id == id).is_some()
    })
}

// pub fn has_phone_number(phone_number: &String) -> bool {
//     PHONE_NUMBER_STORE.with(|phone_number_store| {
//         let binding = phone_number_store.borrow();
//         binding.get(phone_number).is_some()
//     })
// }


// pub fn has_email_address(email_address: &String) -> bool {
//     EMAIL_ADDRESS_STORE.with(|email_address_store| {
//         let binding = email_address_store.borrow();
//         binding.get(email_address).is_some()
//     })
// }

// pub fn add_id(id: String, principal: Principal) -> Result<(), String> {
//     ID_STORE.with(|id_store| {
//         id_store.borrow_mut().insert(id.clone(), principal);
//     });

//     Ok(())
// }

// pub fn add_phone_number(phone_number: Option<String>, principal: Principal) -> Result<(), String> {
//     if phone_number.is_some() {
//         PHONE_NUMBER_STORE.with(|store| {
//             store.borrow_mut().insert(phone_number.unwrap(), principal);
//         });
//     }
//     return Ok(());
// }

// pub fn add_email_address(email_address: Option<String>, principal: Principal) -> Result<(), String> {
//     if email_address.is_some() {
//         EMAIL_ADDRESS_STORE.with(|store| {
//             store.borrow_mut().insert(email_address.unwrap(), principal);
//         });
//     }
//     Ok(())
// }

// pub fn create_profile(
//     id: String,
//     principal: Principal,
//     first_name: Option<String>,
//     last_name: Option<String>,
//     phone_number: Option<String>,
//     email_address: Option<String>,
// ) -> Result<(), String> {
//     let _ = add_id(id.clone(), principal);
//     let _ = add_phone_number(phone_number.clone(), principal);
//     let _ = add_email_address(email_address.clone(), principal);

//     let profile = Profile {
//         id,
//         first_name,
//         last_name,
//         phone_number,
//         email_address,
//     };
//     PROFILE_STORE.with(|profile_store| {
//         profile_store.borrow_mut().insert(principal, profile);
//     });

//     Ok(())
// }

// pub fn get_profile(id: String) -> Profile {
//     let none_profile = Profile{
//         id: "".to_string(),
//         first_name: None,
//         last_name: None,
//         phone_number: None,
//         email_address: None,
//     };
//     let principal = ID_STORE.with(|id_store| id_store.borrow().get(&id).unwrap().clone());
//     PROFILE_STORE.with(|profile_store| profile_store.borrow().get(&principal).unwrap_or(&none_profile).clone())
// }