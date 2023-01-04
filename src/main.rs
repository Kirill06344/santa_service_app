use actix_web::{web, App, HttpServer, route};


mod services;
use services::{get_users, add_group};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(add_group)
            .service(get_users)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

