use crate::lib::DbActor;
use crate::messages::GetUsers;
use actix::Handler;
use diesel::{self, prelude::*};
use crate::models::User;

use crate::schema::users::dsl::*;


//handler который обрабатывает сообщения определенного типа, приходящие на актера, которго он имплементит
impl Handler<GetUsers> for DbActor {
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, msg: GetUsers, ctx: &mut Self::Context) -> Self::Result {
        //получаем наше соединение
        let mut conn = self.0.get().expect("GetUsers unable");
        users.get_results::<User>(& mut conn)
    }
}