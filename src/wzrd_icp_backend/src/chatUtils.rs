use ic_cdk::{
    export:: {candid::CandidType, Principal}
};
use std::{cell::RefCell};
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

thread_local! {
    pub static MESSAGE_STORE: RefCell<MessageStore> = RefCell::default();
    pub static GROUP_STORE: RefCell<GroupStore> = RefCell::default();
    pub static USER_GROUP_STORE: RefCell<UserGroupStore> = RefCell::default();
}

pub fn create_group(id: String, group_id: String, group_name: String, group_description: Option<String>) -> String {
    let res = idUtils::has_id(&id);
    if res {
        return "Invalid User ID".to_string();
    }
    GROUP_STORE.with(|group_store| {
        let new_group = Group{
            group_id: group_id.clone(),
            group_name,
            group_description,
            group_members: vec![id],
        };
        group_store.borrow_mut().push(new_group);
    });
    // USER_GROUP_STORE.with(|user_group_store| {
    //     let ug_store = user_group_store.borrow_mut();
    //     if ug_store.get_mut(&id).is_some(){
    //         ug_store.get_mut(&id).unwrap().push(group_id);
    //     }
    //     else{
    //         ug_store.insert(id, vec![group_id]);
    //     }
    // });
    "Success!".to_string()
}

pub fn join_group(id: String, group_id: String) -> String {
    let res = idUtils::has_id(&id);
    if res {
        return "Invalid User ID".to_string();
    }
    GROUP_STORE.with(|group_store| {
        if let Some(group) = group_store.borrow_mut().iter_mut().find(|group| *group.group_id == group_id){
            let mut new_members = group.group_members.clone();
            new_members.push(id);
            group.group_members = new_members;
        }
    });
    "Success!".to_string()
}

pub fn leave_group(id: String, group_id: String) -> String {
    let res = idUtils::has_id(&id);
    if res {
        return "Invalid User ID".to_string();
    }
    GROUP_STORE.with(|group_store| {
        if let Some(group) = group_store.borrow_mut().iter_mut().find(|group| *group.group_id == group_id){
            let mut new_members = group.group_members.clone();
            // *group = Group{
            //     group_id: group.clone().group_id,
            //     group_name: group.clone().group_name,
            //     group_description: group.clone().group_description,
            //     group_members: new_members,
            // };
            new_members.retain(|member| *member != id);
            group.group_members = new_members;
        }
    });
    "Success!".to_string()
}                                      

pub fn get_group_members(group_id: String) -> Vec<String> {
    GROUP_STORE.with(|group_store| {
        group_store.borrow().iter().find(|&group| *group.group_id == group_id).unwrap().clone().group_members
    })
}

// pub fn get_group_list(id: String) -> Vec<Group> {
//     // GROUP_STORE.with(|group_store| {
//     //     group_store.borrow().iter().filter(|&group| group.group_id == )
//     // })
// }

pub fn get_friend_list() -> bool {true}

pub fn get_group_messages() -> bool {true}

pub fn get_friend_messages() -> bool {true}

pub fn send_group_message() -> bool {true}

pub fn send_friend_message() -> bool {true}

pub fn view_message() -> bool {true}


// pub fn has_id(id: &String) -> bool {
//     idUtils::ID_STORE.with(|id_store| {
//         let store = id_store.borrow();
//         store.get(id).is_some()
//     })
// }

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