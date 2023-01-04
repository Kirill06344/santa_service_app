use actix_web::{get, HttpResponse, HttpServer, post, Responder};
use actix_web::web::{Json, Path};
use santa_project::establish_connection;

use santa_project::actions::{find_users, add_group_with_name};

#[get("/users/get_all_users")]
pub async fn get_users() -> impl Responder {
    let mut conn = establish_connection();
    match find_users(&mut conn) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("Not found users")
    }
}

#[post("/users/{id}/add_group")]
pub async fn add_group(path: Path<i32>, name: Json<String>) -> impl Responder {
    let id: i32 = path.into_inner();
    let mut conn = establish_connection();
    match add_group_with_name(&mut conn, &name.to_string(), id) {
        Ok(group) => HttpResponse::Ok().json(group),
        Err(err) => HttpResponse::NotFound().json(err.to_string())
    }
}