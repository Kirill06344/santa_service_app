use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fmt::{format, Formatter};
use std::ops::Add;
use crate::lib::DbActor;
use crate::messages::{GetUsers, GetGroups, AddGroup, EnterGroup, MakeAdmin, GetIdFromLogin, Resign, LeaveGroup, DeleteGroup, StartSanta};
use actix::Handler;
use diesel::{self, prelude::*};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::errors::Errors;
use crate::models::{User, Group, UserToGroup};
use crate::insertables::{InsertableUserGroup, InsertableSanta};

extern crate rand;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

use crate::schema::users::dsl::*;
use crate::schema::groups::dsl::*;
use crate::schema::user_group::dsl::*;


impl Handler<GetIdFromLogin> for DbActor {
    type Result = Result<i32, Errors>;

    fn handle(&mut self, msg: GetIdFromLogin, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get();

        if conn.is_err() {
            return Err(Errors::DbConnectionError);
        }
        let mut conn = conn.unwrap();

        let u_id = users.filter(login.eq(msg.login.clone())).get_result::<User>(& mut conn);

        return if u_id.is_err() {
            let res = diesel::insert_into(users)
                .values(login.eq(msg.login))
                .get_result::<User>(&mut conn);
            if !res.is_err() { Ok(res.unwrap().id) } else { Err(Errors::DbConnectionError) }
        } else {
            Ok(u_id.unwrap().id)
        }

    }
}

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
    type Result = Result<Group, Errors>;

    fn handle(&mut self, msg: AddGroup, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Database is unable");

        let inserted_group = diesel::insert_into(groups)
            .values(name.eq(msg.name)).
            get_result::<Group>(& mut conn);

        if inserted_group.is_err() {
            return Err(Errors::NotUniqueGroupName);
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
            return Err(Errors::NotUpdated);
        }

        return Ok(inserted_group.unwrap());
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
    type Result = Result<UserToGroup, Errors>;

    fn handle(&mut self, msg: EnterGroup, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Database is unable");

        let gr_id = find_group_id_by_name(& mut conn, msg.name);
        if gr_id == -1 {
            return Err(Errors::CantFindGroupByName);
        }

        let already_in_group = user_group.filter(user_id.eq(msg.user_id)).filter(group_id.eq(gr_id)).execute(& mut conn);

        return if !already_in_group.is_err() && already_in_group.unwrap() == 0 {
            let dep_group = InsertableUserGroup {
                user_id: msg.user_id,
                group_id: gr_id,
                is_admin: false
            };
            let res = diesel::insert_into(user_group).values(&dep_group).get_result::<UserToGroup>(&mut conn);
            if !res.is_err() { Ok(res.unwrap()) } else { Err(Errors::DbConnectionError) }
        } else {
            Err(Errors::NotUpdated)
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

pub fn is_admin_in_group_other_curr_user(conn: & mut PooledConnection<ConnectionManager<PgConnection>>, u_id: i32, g_id: i32) -> bool {
    let res: QueryResult<usize> = user_group.filter(group_id.eq(g_id))
        .filter(is_admin.eq(true))
        .filter(user_id.ne(u_id))
        .execute(conn);
    if res.is_err() || res.unwrap() == 0 {
        return false;
    }
    return true;
}


fn check_admin_access(conn: & mut PooledConnection<ConnectionManager<PgConnection>>, u_id: i32, group_name: String) -> Result<i32, Errors> {
    let gr_id = find_group_id_by_name(conn, group_name);
    if gr_id == -1 {
        return Err(Errors::CantFindGroupByName);
    }

    if !is_admin_in_group(conn, u_id, gr_id) {
        return Err(Errors::AccessDenied);
    }

    Ok(gr_id)
}

impl Handler<MakeAdmin> for DbActor {
    type Result = Result<UserToGroup, Errors>;

    fn handle(&mut self, msg: MakeAdmin, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Database is unable");

        let gr_id = check_admin_access(& mut conn, msg.user_id, msg.group_name);

        let gr_id = if !gr_id.is_err() {gr_id.unwrap()} else {return Err(gr_id.err().unwrap());};

         let future_admin_id = users.filter(login.eq(msg.admin_name)).get_result::<User>(& mut conn);
       if future_admin_id.is_err(){
            return Err(Errors::CantFindUserName);
        }

        let a = diesel::update(user_group
            .filter(user_id.eq(future_admin_id.unwrap().id))
            .filter(group_id.eq(gr_id)))
            .set(is_admin.eq(true))
            .get_result::<UserToGroup>(& mut conn);

        return if !a.is_err() {Ok(a.unwrap())} else {Err(Errors::DbConnectionError)} ;
    }
}

impl Handler<Resign> for DbActor {
    type Result = Result<UserToGroup, Errors>;

    fn handle(&mut self, msg: Resign, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Database is unable");

        let gr_id = check_admin_access(& mut conn, msg.user_id, msg.group_name);

        let gr_id = if !gr_id.is_err() {gr_id.unwrap()} else {return Err(gr_id.err().unwrap());};

        return if is_admin_in_group_other_curr_user(&mut conn, msg.user_id, gr_id) {
            let res = diesel::update(user_group
                .filter(group_id.eq(gr_id))
                .filter(user_id.eq(msg.user_id)))
                .set(is_admin.eq(false))
                .get_result::<UserToGroup>(&mut conn);
            if !res.is_err() { Ok(res.unwrap()) } else { Err(Errors::DbConnectionError) }
        } else {
            Err(Errors::AloneAdmin)
        }


    }
}

impl Handler<LeaveGroup> for DbActor {
    type Result = Result<String, Errors>;

    fn handle(&mut self, msg: LeaveGroup, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Database is unable");


        let gr_id = find_group_id_by_name(& mut conn, msg.group_name.clone());
        if gr_id == -1 {
            return Err(Errors::CantFindGroupByName);
        }

        let is_user_admin = is_admin_in_group(& mut conn, msg.user_id, gr_id);
        let is_admin_there = is_admin_in_group_other_curr_user(&mut conn, msg.user_id, gr_id);


        if !is_user_admin || (is_user_admin && is_admin_there) {
            let deleted_rows = diesel::delete(
                user_group
                    .filter(user_id.eq(msg.user_id))
                    .filter(group_id.eq(gr_id))
            ).execute(& mut conn);

            return if !deleted_rows.is_err() && deleted_rows.unwrap() != 0 {
                Ok(format!("You successfully leaved a group {n}", n = msg.group_name.clone()))
            } else {
                Err(Errors::NotUpdated)
            }

        }
        Err(Errors::AloneAdmin)
    }
}

impl Handler<DeleteGroup> for DbActor {
    type Result = Result<String, Errors>;

    fn handle(&mut self, msg: DeleteGroup, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Database is unable");

        let gr_id = check_admin_access(& mut conn, msg.user_id, msg.group_name.clone());

        let gr_id = if !gr_id.is_err() {gr_id.unwrap()} else {return Err(gr_id.err().unwrap());};

        let delete_users_from_group = diesel::delete(user_group
            .filter(group_id.eq(gr_id)))
            .execute(& mut conn);

        if !delete_users_from_group.is_err() {
            use crate::schema::groups::dsl::id;
            return if !diesel::delete(groups.filter(id.eq(gr_id))).execute(& mut conn).is_err()
                    {Ok(format!("The group with name {n} has been deleted", n = msg.group_name))}
                    else {Err(Errors::DbConnectionError)};
        }

        Err(Errors::DbConnectionError)
    }
}

pub fn lottery(user_storage: &mut Vec<i32>) -> HashMap<i32, i32> {
    let mut rng = rand::thread_rng();
    let mut user_storage_clone =  user_storage.clone();
    user_storage_clone.shuffle(& mut rng);
    let mut user_storage_copy = VecDeque::from(user_storage_clone.to_vec());
    user_storage.shuffle(&mut rng);

    let mut map_tuples: HashMap<i32, i32> = HashMap::new();



    for i in 0..user_storage.len() - 1 {
        if user_storage[i] == user_storage_copy[0] {
          user_storage_copy.swap(0, user_storage_copy.len() - 1);
        }
        map_tuples.insert(user_storage[i].clone(), user_storage_copy.pop_front().unwrap());
    }

    let a = user_storage.last().unwrap();
    let b = user_storage_copy.back().unwrap();

    if a == b  {
        let tmp = map_tuples.get(&user_storage[0]).unwrap().clone();
        map_tuples.insert(user_storage[0], b.clone());
        map_tuples.insert(a.clone(), tmp.clone());
    }

    map_tuples

}

impl Handler<StartSanta> for DbActor {
    type Result = Result<String, Errors>;

    fn handle(&mut self, msg: StartSanta, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Database is unable");

        let gr_id = check_admin_access(& mut conn, msg.user_id, msg.group_name.clone());

        let gr_id = if !gr_id.is_err() {gr_id.unwrap()} else {return Err(gr_id.err().unwrap());};

        let user_storage: Vec<UserToGroup>;
        match user_group.filter(group_id.eq(gr_id)).get_results(& mut conn) {
           Ok(res) => {user_storage = res},
           Err(_) =>  {return Err(Errors::DbConnectionError)}
        }

        if user_storage.len() < 3 {
            return Err(Errors::NotEnoughParticipants)
        }

        let mut user_storage :Vec<i32> = user_storage.iter().map(|u| u.user_id).collect();

        let map = lottery(& mut user_storage);

        for (&santa_id, &present) in map.iter() {
            let tmp = InsertableSanta {
                user_id: santa_id,
                present_id: present,
                group_id: gr_id.clone()
            };
            use crate::schema::santa::dsl::*;
            let res = diesel::insert_into(santa).values(&tmp).execute(& mut conn);
            if res.is_err() || res.unwrap() == 0 {
                return Err(Errors::DbConnectionError);
            }
        }


        use crate::schema::groups::dsl::id;
        if !diesel::update(groups.filter(id.eq(gr_id))).set(closed.eq(true)).execute(& mut conn).is_err() {
            Ok("You started Secret Santa!!!".to_string())
        } else {
            Err(Errors::DbConnectionError)
        }
    }
}