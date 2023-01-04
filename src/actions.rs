use diesel::prelude::*;
use crate::models::User;


pub fn find_users(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
    use crate::schema::users::dsl::*;
    users.load::<User>(conn)
}