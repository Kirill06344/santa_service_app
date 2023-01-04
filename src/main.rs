use actix_web::{web, App, HttpServer, route};


mod services;
use services::{get_users, add_group};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
                web::scope("/users")
                .route("/get_all_users", web::get().to(get_users))
                .route("/add_group", web::post().to(add_group))
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

