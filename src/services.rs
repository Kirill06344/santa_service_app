use actix_web::{get, HttpResponse, Responder};
use santa_project::establish_connection;

use santa_project::actions::{find_users};


pub async fn get_users() -> impl Responder {
    let mut conn = establish_connection();
    match find_users(&mut conn) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("Not found users")
    }
}

pub async fn add_group() -> impl Responder {

}