use crate::models::{User, Group, UserToGroup};
use actix::Message;
use diesel::QueryResult;
use serde::Deserialize;
use crate::errors::Errors;


//указываем тип, что должен вернуть актер
//тот, кто получает GetUsers -> "QueryResult<Vec<User>>"
#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct GetUsers;

#[derive(Message, Deserialize)]
#[rtype(result = "Result<i32, Errors>")]
pub struct GetIdFromLogin {
    pub login: String
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Group>>")]
pub struct GetGroups;

#[derive(Message, Deserialize)]
#[rtype(result = "Result<Group, Errors>")]
pub struct AddGroup {
    pub name: String,
    pub user_id: i32
}

#[derive(Message, Deserialize)]
#[rtype(result = "Result<UserToGroup, Errors>")]
pub struct EnterGroup {
    pub name: String,
    pub user_id: i32
}


#[derive(Message, Deserialize, Clone)]
#[rtype(result = "Result<UserToGroup, Errors>")]
pub struct MakeAdmin {
    pub group_name: String,
    pub user_id: i32,
    pub admin_name: String
}

#[derive(Message, Deserialize)]
#[rtype(result = "Result<UserToGroup, Errors>")]
pub struct Resign {
    pub group_name: String,
    pub user_id: i32
}

#[derive(Message, Deserialize)]
#[rtype(result = "Result<String, Errors>")]
pub struct LeaveGroup {
    pub group_name: String,
    pub user_id: i32
}

#[derive(Message, Deserialize)]
#[rtype(result = "Result<String, Errors>")]
pub struct DeleteGroup {
    pub group_name: String,
    pub user_id: i32
}

#[derive(Message, Deserialize)]
#[rtype(result = "Result<String, Errors>")]
pub struct StartSanta {
    pub group_name: String,
    pub user_id: i32
}

#[derive(Message, Deserialize)]
#[rtype(result = "Result<String, Errors>")]
pub struct GetYourPresent {
    pub group_name: String,
    pub user_id: i32
}