use crate::models::{User, Group};
use actix::Message;
use diesel::QueryResult;

//указываем тип, что должен вернуть актер
//тот, кто получает GetUsers -> "QueryResult<Vec<User>>"
#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct GetUsers;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Group>>")]
pub struct GetGroups;

#[derive(Message)]
#[rtype(result = "QueryResult<Group>")]
pub struct AddGroup {
    pub name: String,
    pub user_id: i32
}

#[derive(Message)]
#[rtype(result = "QueryResult<String>")]
pub struct EnterGroup {
    pub name: String,
    pub user_id: i32
}

