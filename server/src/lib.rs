use actix::{Actor, Addr, SyncContext};

use::diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool}
};

//Addr хранит адресс нашего актера
pub struct AppState {
    pub db: Addr<DbActor>
}

//Кортежная структура
pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);

//impl используется для реализации методов объекта
//конструкция impl .. for нужна для добавления методов из другой структуры
impl Actor for DbActor {
    //необходимо, чтобы запускать актера с SyncArbeiter
    type Context = SyncContext<Self>;
}

pub fn get_pool(db_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    //создаем менеджера, который отвечает за соединения с БД
    let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder().build(manager).expect("Error building the connection")
}


