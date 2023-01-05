use crate::models::User;
use actix::Message;
use diesel::QueryResult;


#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct GetUsers;

