use actix_web::{web, App, HttpServer, web::Data};
use actix::SyncArbiter;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection
};
use std::env;
use dotenvy::dotenv;

mod lib;
mod services;
mod messages;
mod actors;
mod models;
mod schema;
mod insertables;


use lib::{get_pool, AppState, DbActor};
use services::{get_users, add_group, get_groups, enter_group};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    //Получаем URL нашей db из .env файла
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //Создаем пул, который будет хранить наши соединения
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&database_url);
    //запускаем SyncArbiter и отправляем адресс нашего Actor pool
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState{db: db_addr.clone()}))
            .service(add_group)
            .service(get_users)
            .service(get_groups)
            .service(enter_group)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
