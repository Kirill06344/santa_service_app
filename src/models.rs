use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub login: String
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub closed: bool
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserToGroup {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Admins {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Santa {
    pub id: i32,
    pub user_id: i32,
    pub present_id: i32,
    pub group_id: i32
}