use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub group_name: String,
    pub admin_login: String,
}
