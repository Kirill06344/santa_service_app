use diesel::insert_into;
use diesel::prelude::*;
use crate::models::{User, Group};


pub fn find_users(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
    use crate::schema::users::dsl::*;
    users.load::<User>(conn)
}

pub fn add_group_with_name(conn: &mut PgConnection, gr_name: &String, u_id: i32) -> QueryResult<Group> {
    use crate::schema::groups::dsl::*;
    let res = insert_into(groups).values(name.eq(gr_name)).get_result::<Group>(conn);
    if res.is_err() {
        return Err(diesel::NotFound);
    }
    use crate::schema::user_group::dsl::*;
    insert_into(user_group).values((user_id.eq(u_id), group_id.eq(res.as_ref().unwrap().id))).execute(conn).expect("qwerty");

    //use crate::schema::admins::dsl::*;
    //insert_into(admins).values((user_id.eq(u_id), group_id.eq(res.unwrap().id))).execute(conn).expect("qwerty");

    res
}
