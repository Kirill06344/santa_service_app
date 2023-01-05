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


use lib::{get_pool, AppState, DbActor};
use services::{get_users, add_group};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&database_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState{db: db_addr.clone()}))
            .service(add_group)
            .service(get_users)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

