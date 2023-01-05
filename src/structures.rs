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
