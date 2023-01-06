use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Commands {
    CreateGroup(String), // group name
    DeleteGroup(String), // group name
    JoinGroup(String), // group name
    LeaveGroup(String), // group name
    AssignAdmin(String), // user name
    DeleteAdmin,
    GenerateSantas(String), // group name
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub login: String,
    pub command: Commands,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct a {
    pub name: String,
    pub user_id: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct zxc {
    pub id: i32,
    pub name: String,
    pub closed: bool,
}

