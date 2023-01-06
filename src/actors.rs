use std::fmt;
use std::fmt::Formatter;
use std::ops::Add;
use crate::lib::DbActor;
use crate::messages::{GetUsers, GetGroups, AddGroup, EnterGroup, MakeAdmin};
use actix::Handler;
use diesel::{self, prelude::*};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use serde::de::value::Error;
use crate::models::{User, Group, UserToGroup};
use crate::insertables::InsertableUserGroup;

use crate::schema::users::dsl::*;
use crate::schema::groups::dsl::*;
use crate::schema::user_group::dsl::*;

//handler который обрабатывает сообщения определенного типа, приходящие на актера, которго он имплементит
impl Handler<GetUsers> for DbActor {
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, msg: GetUsers, ctx: &mut Self::Context) -> Self::Result {
        //получаем наше соединение
        let mut conn = self.0.get().expect("Database is unable");
        users.get_results::<User>(& mut conn)
    }
}

impl Handler<GetGroups> for DbActor {
    type Result = QueryResult<Vec<Group>>;

    fn handle(&mut self, msg: GetGroups, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Database is unable");

        groups.get_results::<Group>(& mut conn)
    }
}

impl Handler<AddGroup> for DbActor {
    type Result = QueryResult<Group>;

    fn handle(&mut self, msg: AddGroup, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Database is unable");

        let inserted_group = diesel::insert_into(groups)
            .values(name.eq(msg.name)).
            get_result::<Group>(& mut conn);

        if inserted_group.is_err() {
            return inserted_group;
        }

        let dep_group = InsertableUserGroup {
            user_id: msg.user_id,
            group_id: inserted_group.as_ref().unwrap().id,
            is_admin: true
        };

        let a = diesel::insert_into(user_group)
            .values(&dep_group).execute(& mut conn);

        if a.is_err() || a.unwrap() == 0 {
            use crate::schema::groups::dsl::id;
            diesel::delete(groups).filter(id.eq(inserted_group.as_ref().unwrap().id)).execute(& mut conn).expect("Error while deleting");
            return Err(diesel::result::Error::NotFound);
        }

        return inserted_group;
    }
}

pub fn find_group_id_by_name(conn: & mut PooledConnection<ConnectionManager<PgConnection>>, group_name: String) -> i32 {
    let founded_group = groups.filter(name.eq(group_name)).get_result::<Group>(conn);
    if founded_group.is_err() {
        return -1;
    }
    founded_group.unwrap().id
}

impl Handler<EnterGroup> for DbActor{
    type Result = QueryResult<String>;

    fn handle(&mut self, msg: EnterGroup, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Database is unable");

        let gr_id = find_group_id_by_name(& mut conn, msg.name);
        if gr_id == -1 {
            return Ok("Group with this name doesn't exists!".to_string());
        }

        let already_in_group = user_group.filter(user_id.eq(msg.user_id)).filter(group_id.eq(gr_id)).execute(& mut conn);

        if !already_in_group.is_err() && already_in_group.unwrap() == 0 {
            let dep_group = InsertableUserGroup {
                user_id: msg.user_id,
                group_id: gr_id,
                is_admin: false
            };

            match diesel::insert_into(user_group).values(&dep_group).execute(& mut conn) {
                Ok(info) => Ok("You added into group!".to_string()),
                Err(_) => Err(diesel::result::Error::NotFound)
            }
        } else {
            return Ok("You are already in this group!".to_string());
        }

    }
}

pub fn is_admin_in_group(conn: & mut PooledConnection<ConnectionManager<PgConnection>>, u_id: i32, g_id: i32) -> bool {
    let res: QueryResult<usize> = user_group.filter(user_id.eq(u_id)).filter(group_id.eq(g_id)).filter(is_admin.eq(true)).execute(conn);
    if res.is_err() || res.unwrap() == 0 {
        return false;
    }
    return true;
}


impl Handler<MakeAdmin> for DbActor {
    type Result = QueryResult<UserToGroup>;

    fn handle(&mut self, msg: MakeAdmin, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Database is unable");

        let gr_id = find_group_id_by_name(& mut conn, msg.group_name);
        if gr_id == -1 {
            return Err(diesel::result::Error::NotFound);
        }


        if !is_admin_in_group(& mut conn, msg.user_id, gr_id) {
            return Err(diesel::result::Error::NotFound);
        }

        diesel::update(user_group
            .filter(user_id.eq(msg.future_admin_id))
            .filter(group_id.eq(gr_id)))
            .set(is_admin.eq(true))
            .get_result::<UserToGroup>(& mut conn)
    }
}



