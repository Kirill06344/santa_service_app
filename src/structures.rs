use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u32,
    pub group_name: String,
    pub admin_login: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonalId {
    pub id: Option<u32>,
    pub login: String,
}