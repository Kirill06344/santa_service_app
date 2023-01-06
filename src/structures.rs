use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct User {
    pub group_name: String,
    pub user_id: i32,
    pub admin_name: String,
}
