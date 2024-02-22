use ic_cdk::update;
mod chat_utils;

#[update(name = "a.CreateGroup")]
pub async fn create_group(params: chat_utils::CreateGroupParams) -> chat_utils::CreateGroupResponse {
    chat_utils::create_group(params).await
}

#[update(name = "b.JoinGroup")]
pub async fn join_group(params: chat_utils::JoinGroupParams) -> chat_utils::JoinGroupResponse {
    chat_utils::join_group(params).await
}

#[update(name = "c.LeaveGroup")]
pub async fn leave_group(params: chat_utils::LeaveGroupParams) -> chat_utils::LeaveGroupResponse {
    chat_utils::leave_group(params).await
}

#[update(name = "d.GetJoinedGroup")]
pub async fn get_group_list(params: chat_utils::GetJoinedGroupParams) -> chat_utils::GetJoinedGroupResponse {
    chat_utils::get_group_list(params).await
}

#[update(name = "e.GetGroupMembers")]
pub async fn get_group_members(params: chat_utils::GetGroupMembersParams) -> chat_utils::GetGroupMembersResponse {
    chat_utils::get_group_members(params).await
}

#[update(name = "f.SendGroupMessage")]
pub async fn send_group_message(params: chat_utils::SendGroupMessageParams) -> chat_utils::SendGroupMessageResponse {
    chat_utils::send_group_message(params).await
}

#[update(name = "g.GetGoupMessage")]
pub async fn get_group_messages(params: chat_utils::GetGroupMessageParams) -> chat_utils::GetGroupMessageResponse {
    chat_utils::get_group_messages(params).await
}

#[update(name = "h.GetConnectedMembers")]
pub async fn get_friend_list(params: chat_utils::GetConnectedMemberParams) -> chat_utils::GetConnectedMemberResponse {
    chat_utils::get_friend_list(params).await
}

#[update(name = "i.SendDirectMessage")]
pub async fn send_direct_message(params: chat_utils::SendDirectMessageParams) -> chat_utils::SendDirectMessageResponse {
    chat_utils::send_direct_message(params).await
}

#[update(name = "j.GetDirectMessages")]
pub async fn get_direct_messages(params: chat_utils::GetDirectMessageParams) -> chat_utils::GetDirectMessageResponse {
    chat_utils::get_direct_messages(params).await
}

#[update(name = "k.ViewMessage")]
pub async fn view_message(params: chat_utils::ViewMessageParams) -> chat_utils::ViewMessageResponse {
    chat_utils::view_message(params).await
}