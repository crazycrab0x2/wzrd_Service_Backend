use candid::{Deserialize, Principal};
use ic_cdk::export::candid::CandidType;
use ic_cdk::api::time;
use std::cell::RefCell;
use std::collections::BTreeMap;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;

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
    pub token: String, 
    pub group_id: String, 
    pub group_name: String, 
    pub group_description: Option<String>
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct CreateGroupResponse {
    pub token: String, 
    pub error: String, 
    pub result: bool
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct LeaveGroupParams {
    pub token: String, 
    pub group_id: String
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct LeaveGroupResponse {
    pub token: String, 
    pub error: String, 
    pub result: bool
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct JoinGroupParams {
    pub token: String, 
    pub group_id: String
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct JoinGroupResponse {
    pub token: String, 
    pub error: String, 
    pub result: bool
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct GetGroupMembersParams {
    pub token: String, 
    pub group_id: String
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct GetGroupMembersResponse {
    pub token: String, 
    pub error: String, 
    pub result: Vec<String>
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct GetJoinedGroupParams {
    pub token: String
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct GetJoinedGroupResponse {
    pub token: String, 
    pub error: String, 
    pub result: Vec<String>
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct GetGroupMessageParams {
    pub token: String, 
    pub group_id: String
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct GetGroupMessageResponse {
    pub token: String, 
    pub error: String, 
    pub result: Vec<GroupMessage>
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct SendGroupMessageParams {
    pub token: String, 
    pub group_id: String, 
    pub reply_id: Option<String>,
    pub content: String
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct SendGroupMessageResponse {
    pub token: String, 
    pub error: String, 
    pub result: bool
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct SendDirectMessageParams {
    pub token: String, 
    pub receiver: String, 
    pub reply_id: Option<String>,
    pub content: String
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct SendDirectMessageResponse {
    pub token: String, 
    pub error: String, 
    pub result: bool
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct GetConnectedMemberParams {
    pub token: String
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct GetConnectedMemberResponse {
    pub token: String, 
    pub error: String, 
    pub result: Vec<String>
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct GetDirectMessageParams {
    pub token: String, 
    pub receiver: String
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct GetDirectMessageResponse {
    pub token: String, 
    pub error: String, 
    pub result: Vec<DirectMessage>
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct ViewMessageParams {
    pub token: String, 
    pub msg_id: String
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct ViewMessageResponse {
    pub token: String, 
    pub result: bool
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
) -> CreateGroupResponse {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckToken", (params.token,)).await;
    match user_validation {
        Err(_err) => {
            CreateGroupResponse {
                token: "".to_string(),
                error: "Can't access ID service".to_string(), 
                result: false
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                return CreateGroupResponse {
                    token,
                    error: "Invalid token".to_string(),
                    result: false
                };
            }
            else if has_group_id(params.group_id.clone()) {
                return CreateGroupResponse {
                    token,
                    error: "Group ID Alreay Exist".to_string(),
                    result: false
                };
            }
            else {
                let user_name = get_user_name(token.clone());
                GROUP_STORE.with(|group_store| {
                    let new_group = Group{
                        group_id: params.group_id.clone(),
                        group_name: params.group_name,
                        group_description: params.group_description,
                        group_members: vec![user_name.clone()],
                    };
                    group_store.borrow_mut().push(new_group);
                });
                USER_GROUP_STORE.with(|user_group_store| {
                    let mut new_group_list;
                    if user_group_store.borrow().get(&user_name).is_none() {
                        new_group_list = vec![];
                    } else {
                        new_group_list = user_group_store.borrow().get(&user_name).unwrap().clone();
                    }
                    new_group_list.push(params.group_id);
                    user_group_store.borrow_mut().insert(user_name, new_group_list);
                });
                CreateGroupResponse {
                    token,
                    error: "".to_string(),
                    result: true
                }
            }
        }
    }    
}

pub async fn join_group(
    params: JoinGroupParams
) -> JoinGroupResponse {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckToken", (params.token.clone(),)).await;
    match user_validation {
        Err(_err) => {
            JoinGroupResponse{
                token: "".to_string(),
                error: "Can't access ID service".to_string(),
                result: false
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                return JoinGroupResponse{
                    token,
                    error: "Invalid token".to_string(),
                    result: false
                };
            }
            else if !has_group_id(params.group_id.clone()) {
                return JoinGroupResponse{
                    token,
                    error: "Group ID doesn't exist".to_string(),
                    result: false
                };
            }
            else{
                let user_name = get_user_name(token.clone());
                GROUP_STORE.with(|group_store| {
                    if let Some(group) = group_store.borrow_mut().iter_mut().find(|group| *group.group_id == params.group_id){
                        let mut new_members = group.group_members.clone();
                        if !new_members.iter().find(|&member| *member == user_name).is_some(){
                            new_members.push(user_name.clone());
                        }
                        group.group_members = new_members;
                    }
                });
                USER_GROUP_STORE.with(|user_group_store| {
                    let mut new_group_list;
                    if user_group_store.borrow().get(&user_name).is_none() {
                        new_group_list = vec![];
                    } else {
                        new_group_list = user_group_store.borrow().get(&user_name).unwrap().clone();
                    }
                    if !new_group_list.iter().find(|&group| *group == params.group_id).is_some(){
                        new_group_list.push(params.group_id);
                    }
                    user_group_store.borrow_mut().insert(user_name, new_group_list);
                });
                JoinGroupResponse{
                    token,
                    error: "".to_string(),
                    result: true
                }
            }
        }
    }
}

pub async fn leave_group(
    params: LeaveGroupParams
) -> LeaveGroupResponse {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckToken", (params.token.clone(),)).await;
    match user_validation {
        Err(_err) => {
            return LeaveGroupResponse{
                token: "".to_string(),
                error: "Can't access ID service".to_string(),
                result: false
            };
        }
        Ok((token,)) => {
            if token == "".to_string() {
                return LeaveGroupResponse{
                    token,
                    error: "Invalid token".to_string(),
                    result: false
                };
            }
            else if !has_group_id(params.group_id.clone()) {
                return LeaveGroupResponse{
                    token,
                    error: "Group ID doesn't exist".to_string(),
                    result: false
                };
            }
            else{
                let user_name = get_user_name(token.clone());
                GROUP_STORE.with(|group_store| {
                    if let Some(group) = group_store.borrow_mut().iter_mut().find(|group| *group.group_id == params.group_id){
                        let mut new_members = group.group_members.clone();
                        new_members.retain(|member| *member != user_name);
                        group.group_members = new_members;
                    }
                });
                USER_GROUP_STORE.with(|user_group_store| {
                    let mut new_group_list;
                    if !user_group_store.borrow().get(&user_name).is_none() {
                        new_group_list = user_group_store.borrow().get(&user_name).unwrap().clone();
                        new_group_list.retain(|groupid| *groupid != params.group_id);
                        user_group_store.borrow_mut().insert(user_name, new_group_list);
                    }
                });
                LeaveGroupResponse{
                    token,
                    error: "".to_string(),
                    result: true
                }
            }
        }
    }
}                                      

pub async fn get_group_members(
    params: GetGroupMembersParams
) -> GetGroupMembersResponse {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckToken", (params.token.clone(),)).await;
    match user_validation {
        Err(_err) => {
            return GetGroupMembersResponse{
                token: "".to_string(),
                error: "Can't access ID service".to_string(),
                result: [].to_vec()
            };
        }
        Ok((token,)) => {
            if token == "".to_string() {
                return GetGroupMembersResponse{
                    token,
                    error: "Invalid token".to_string(),
                    result: [].to_vec()
                };
            }
            else if !has_group_id(params.group_id.clone()) {
                return GetGroupMembersResponse{
                    token,
                    error: "Group ID doesn't exist".to_string(),
                    result: [].to_vec()
                };
            }
            else{
                GROUP_STORE.with(|group_store| {
                    let result = group_store.borrow().iter().find(|&group| *group.group_id == params.group_id).unwrap().clone().group_members;
                    GetGroupMembersResponse{
                        token,
                        error: "".to_string(),
                        result
                    }
                })
            }
        }
    }
}

pub async fn get_group_list(
    params: GetJoinedGroupParams
) -> GetJoinedGroupResponse {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckToken", (params.token.clone(),)).await;
    match user_validation {
        Err(_err) => {
            GetJoinedGroupResponse{
                token: "".to_string(),
                error: "Can't access ID service".to_string(),
                result: [].to_vec()
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                GetJoinedGroupResponse{
                    token,
                    error: "Invalid token".to_string(),
                    result: [].to_vec()
                }
            }
            else{
                let user_name = get_user_name(token.clone());
                USER_GROUP_STORE.with(|user_group_store| {
                    let result = user_group_store.borrow().get(&user_name).unwrap_or(&vec![]).clone();
                    GetJoinedGroupResponse{
                        token,
                        error: "".to_string(),
                        result
                    }
                })
            }
        }
    }
}

pub async fn get_group_messages(
    params: GetGroupMessageParams
) -> GetGroupMessageResponse {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckToken", (params.token.clone(),)).await;
    match user_validation {
        Err(_err) => {
            GetGroupMessageResponse{
                token: "".to_string(),
                error: "Can't access ID service".to_string(),
                result: [].to_vec()
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                GetGroupMessageResponse{
                    token,
                    error: "Invalid token".to_string(),
                    result: [].to_vec()
                }
            }
            else if !has_group_id(params.group_id.clone()) {
                GetGroupMessageResponse{
                    token,
                    error: "Group ID doesn't exist".to_string(),
                    result: [].to_vec()
                }
            }
            else{
                GROUP_MESSAGE_STORE.with(|group_message_store| {
                    let result = group_message_store.borrow().get(&params.group_id).unwrap_or(&vec![]).clone();
                    GetGroupMessageResponse{
                        token,
                        error: "".to_string(),
                        result
                    }
                })
            }
        }
    }
}

pub async fn send_group_message(
    params: SendGroupMessageParams
) -> SendGroupMessageResponse {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckToken", (params.token.clone(),)).await;
    match user_validation {
        Err(_err) => {
            SendGroupMessageResponse{
                token: "".to_string(),
                error: "Can't access ID service".to_string(),
                result: false
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                SendGroupMessageResponse{
                    token,
                    error: "Invalid token".to_string(),
                    result: false
                }
            }
            else if !has_group_id(params.group_id.clone()) {
                SendGroupMessageResponse{
                    token,
                    error: "Group ID doesn't exist".to_string(),
                    result: false
                }
            }
            else{
                let user_name = get_user_name(token.clone());
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
                        sender_id: user_name, 
                        reply_id: params.reply_id,
                        content: params.content, 
                        timestamp: (time()/1000000).to_string()
                    };
                    new_message_list.push(message);
                    group_message_store.borrow_mut().insert(params.group_id, new_message_list);
                });
                SendGroupMessageResponse{
                    token,
                    error: "".to_string(),
                    result: true
                }
            }
        }
    }
}

pub async fn send_direct_message(
    params: SendDirectMessageParams
) -> SendDirectMessageResponse {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckToken", (params.token.clone(),)).await;
    match user_validation {
        Err(_err) => {
            SendDirectMessageResponse{
                token: "".to_string(),
                error: "Can't access ID service".to_string(),
                result: false
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                SendDirectMessageResponse{
                    token,
                    error: "Invalid token".to_string(),
                    result: false
                }
            }
            else{
                let receiver_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckUser", (params.receiver.clone(),)).await;
                match receiver_validation {
                    Err(_err) => {
                        SendDirectMessageResponse{
                            token,
                            error: "Can't access ID service".to_string(),
                            result: false
                        }
                    }
                    Ok((id_valid,)) => {
                        if !id_valid {
                            SendDirectMessageResponse{
                                token,
                                error: "Invalid receiver id".to_string(),
                                result: false
                            }
                        }
                        else{
                            let sender = get_user_name(token.clone());
                            DIRECT_MESSAGE_STORE.with(|direct_message_store| {
                                let message = DirectMessage { 
                                    id: direct_message_store.borrow().clone().len().to_string(), 
                                    sender_id: sender.clone(), 
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
                                if user_friend_store.borrow().get(&sender).is_none() {
                                    sender_friend_list = vec![];
                                } else {
                                    sender_friend_list = user_friend_store.borrow().get(&sender).unwrap().clone();
                                }
                                if !sender_friend_list.iter().find(|&id| *id == params.receiver).is_some(){
                                    sender_friend_list.push(params.receiver.clone());
                                }
                                user_friend_store.borrow_mut().insert(sender.clone(), sender_friend_list);

                                if user_friend_store.borrow().get(&params.receiver).is_none() {
                                    receiver_friend_list = vec![];
                                } else {
                                    receiver_friend_list = user_friend_store.borrow().get(&params.receiver).unwrap().clone();
                                }
                                if !receiver_friend_list.iter().find(|&id| *id == sender).is_some(){
                                    receiver_friend_list.push(sender);
                                }
                                user_friend_store.borrow_mut().insert(params.receiver, receiver_friend_list);
                            });
                            SendDirectMessageResponse{
                                token,
                                error: "".to_string(),
                                result: true
                            }
                        }
                    }
                }
            }
        }
    }
}

pub async fn get_friend_list(
    params: GetConnectedMemberParams
) -> GetConnectedMemberResponse {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckToken", (params.token.clone(),)).await;
    match user_validation {
        Err(_err) => {
            GetConnectedMemberResponse{
                token: "".to_string(),
                error: "Can't access ID service".to_string(),
                result: [].to_vec()
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                GetConnectedMemberResponse{
                    token,
                    error: "Invalid token".to_string(),
                    result: [].to_vec()
                }
            }
            else{
                let user_name = get_user_name(token.clone());
                USER_FRIEND_STORE.with(|user_friend_store| {
                    let result = user_friend_store.borrow().get(&user_name).unwrap_or(&vec![]).clone();
                    GetConnectedMemberResponse{
                        token,
                        error: "".to_string(),
                        result
                    }
                })
            }
        }
    }
}

pub async fn get_direct_messages(
    params: GetDirectMessageParams
) -> GetDirectMessageResponse{
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckToken", (params.token.clone(),)).await;
    match user_validation {
        Err(_err) => {
            GetDirectMessageResponse{
                token: "".to_string(),
                error: "Can't access ID service".to_string(),
                result: [].to_vec()
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                GetDirectMessageResponse{
                    token,
                    error: "Invalid token".to_string(),
                    result: [].to_vec()
                }
            }
            else{
                let receiver_validation = ic_cdk::call::<(String,), (bool,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckUser", (params.receiver.clone(),)).await;
                match receiver_validation {
                    Err(_err) => {
                        GetDirectMessageResponse{
                            token,
                            error: "Can't access ID service".to_string(),
                            result: [].to_vec()
                        }
                    }
                    Ok((id_valid,)) => {
                        if !id_valid {
                            GetDirectMessageResponse{
                                token,
                                error: "Invalid token".to_string(),
                                result: [].to_vec()
                            }
                        }
                        else{  
                            let sender = get_user_name(token.clone());                      
                            DIRECT_MESSAGE_STORE.with(|direct_message_store| {
                                let dms = direct_message_store.borrow().clone();
                                let result: Vec<DirectMessage> = dms.iter().filter(|&message| 
                                    message.sender_id == sender && 
                                    message.receiver_id == params.receiver || 
                                    message.sender_id == params.receiver && 
                                    message.receiver_id == sender)
                                .map(|msg|{
                                    let tmp = msg.clone();
                                    tmp
                                }).collect();
                                GetDirectMessageResponse{
                                    token,
                                    error: "".to_string(),
                                    result
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}

pub async fn view_message(params: ViewMessageParams) -> ViewMessageResponse {
    let user_validation = ic_cdk::call::<(String,), (String,)>(Principal::from_text("4dov3-miaaa-aaaap-qb7za-cai").unwrap(), "CheckToken", (params.token.clone(),)).await;
    match user_validation {
        Err(_err) => {
            ViewMessageResponse{
                token: "".to_string(),
                result: false
            }
        }
        Ok((token,)) => {
            if token == "".to_string() {
                ViewMessageResponse{
                    token,
                    result: false
                }
            }
            else{
                DIRECT_MESSAGE_STORE.with(|direct_message_store| {
                    if let Some(message) = direct_message_store.borrow_mut().iter_mut().find(|message| message.id == params.msg_id){
                        message.viewed = true;
                        ViewMessageResponse{
                            token,
                            result: true
                        }
                    }
                    //no message with given id
                    else{
                        ViewMessageResponse{
                            token,
                            result: false
                        }
                    }
                })
            }
        }
    }
}


fn has_group_id(
    id: String
) -> bool {
    GROUP_STORE.with(|group_store| {
        group_store.borrow().iter().find(|group| *group.group_id == id).is_some()
    })
}

pub fn get_user_name(
    token: String
) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"wzrd-secret-key").unwrap();
    let veri_claims;
    let result: Result<BTreeMap<String, String>, jwt::Error> = token.as_str().verify_with_key(&key);
    match result{
        Ok(okay_result) => veri_claims = okay_result,
        Err(_) => return "".to_string()
    };
    let user_name = &veri_claims["username"];
    user_name.clone()
 }