use actix_web::{App, get, HttpResponse, HttpServer, post, Responder};
use actix_web::web::{Json, Path, Data};

use crate::{
    messages::GetUsers, messages::GetGroups, messages::AddGroup,
    messages::EnterGroup,
    AppState, DbActor
};

use actix::Addr;
use actix::fut::err;

#[get("/users")]
pub async fn get_users(state: Data<AppState>) -> impl Responder {
    //Здесь получаем адрес нашего пула
    let db: Addr<DbActor> = state.as_ref().db.clone();

    //отправляем сообщение актеру, так как у нас 5 потоков, то сможем отправлять 5 сообщений одновременно
    match db.send(GetUsers).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users")
    }
}

#[get("/groups")]
pub async fn get_groups(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(GetGroups).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No groups found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve groups")
    }
}

#[post("/users/{id}/add_group")]
pub async fn add_group(state: Data<AppState>, path: Path<i32>, name_group: Json<String>) -> impl Responder {
    let id: i32 = path.into_inner();
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(AddGroup {
        name: name_group.to_string(),
        user_id: id
    }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("The group with this name already exists"),
        _ => HttpResponse::InternalServerError().json("Unable to add group")
    }
}

#[post("/users/{id}/enter_group")]
pub async fn enter_group(state: Data<AppState>, path: Path<i32>, name_group: Json<String>) -> impl Responder {
    let id: i32 = path.into_inner();
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(EnterGroup {
        name: name_group.to_string(),
        user_id: id
    }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(error)) => HttpResponse::BadRequest().json(error.to_string()),
        _ => HttpResponse::InternalServerError().json("Unable to add group")
    }
}