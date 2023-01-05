use actix_web::{App, get, HttpResponse, HttpServer, post, Responder};
use actix_web::web::{Json, Path, Data};

use crate::{
    messages::GetUsers,
    AppState, DbActor
};

use actix::Addr;

#[get("/users/get_all_users")]
pub async fn get_users(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(GetUsers).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users")
    }
}

#[post("/users/{id}/add_group")]
pub async fn add_group(path: Path<i32>, name: Json<String>) -> impl Responder {
 HttpResponse::Ok().json("sads")
}