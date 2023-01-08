use std::collections::{HashMap, VecDeque};
use diesel::{self, prelude::*};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::errors::Errors;
use crate::models::Group;

extern crate rand;
use rand::seq::SliceRandom;


use crate::schema::groups::dsl::*;
use crate::schema::user_group::dsl::*;


pub fn find_group_id_by_name(conn: & mut PooledConnection<ConnectionManager<PgConnection>>, group_name: String) -> i32 {
    let founded_group = groups.filter(name.eq(group_name)).get_result::<Group>(conn);
    if founded_group.is_err() {
        return -1;
    }
    founded_group.unwrap().id
}

pub fn is_admin_in_group(conn: & mut PooledConnection<ConnectionManager<PgConnection>>, u_id: i32, g_id: i32) -> bool {
    let res: QueryResult<usize> = user_group.filter(user_id.eq(u_id)).filter(group_id.eq(g_id)).filter(is_admin.eq(true)).execute(conn);
    if res.is_err() || res.unwrap() == 0 {
        return false;
    }
    return true;
}

pub fn get_group_status(conn: & mut PooledConnection<ConnectionManager<PgConnection>>, g_id: i32) -> Result<bool, Errors> {
    use crate::schema::groups::dsl::id;
    let res = groups.filter(id.eq(g_id)).get_result::<Group>(conn);
    return if res.is_err() {Err(Errors::DbConnectionError)} else {Ok(res.unwrap().closed)}
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


pub fn check_admin_access(conn: & mut PooledConnection<ConnectionManager<PgConnection>>, u_id: i32, group_name: String) -> Result<i32, Errors> {
    let gr_id = find_group_id_by_name(conn, group_name);
    if gr_id == -1 {
        return Err(Errors::CantFindGroupByName);
    }

    if !is_admin_in_group(conn, u_id, gr_id) {
        return Err(Errors::AccessDenied);
    }

    Ok(gr_id)
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
    } else {
        map_tuples.insert(*a, *b);
    }

    map_tuples
}

pub fn check_group_closed(conn: & mut PooledConnection<ConnectionManager<PgConnection>>, group_name: String) -> Result<i32, Errors> {
    let gr_id = find_group_id_by_name(conn, group_name);
    if gr_id == -1 {
        return Err(Errors::CantFindGroupByName);
    }
    match get_group_status(conn, gr_id) {
        Ok(is_closed) => {if is_closed {return Err(Errors::GroupClosed);}}
        Err(error) => {return Err(error);}
    }

    Ok(gr_id)
}