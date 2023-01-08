#![allow(unused)]
#![allow(clippy::all)]

use std::fmt;
use std::fmt::Formatter;
use std::ops::Add;
use diesel::prelude::*;
use diesel::sql_types::{Bool, Integer};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub login: String
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Username: {}",self.login)
    }
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub closed: bool
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut str = format!("Group {}.", self.name);
        if self.closed {
            str = str.add("This group is closed!");
        } else {
            str = str.add("This group isn't closed.");
        }
        write!(f,"{}", str)
    }
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserToGroup {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32,
    pub is_admin: bool
}


#[derive(Queryable, Serialize, Deserialize)]
pub struct Santa {
    pub id: i32,
    pub user_id: i32,
    pub present_id: i32,
    pub group_id: i32
}