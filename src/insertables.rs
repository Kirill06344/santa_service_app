use crate::schema::user_group;
use diesel::Insertable;
use serde::{Serialize, Deserialize};

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = user_group)]
pub struct InsertableUserGroup {
    pub user_id: i32,
    pub group_id: i32,
    pub is_admin: bool
}