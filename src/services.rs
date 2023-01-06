use actix_web::{App, get, HttpResponse, HttpServer, post, Responder};
use actix_web::web::{Json, Path, Data};

use crate::{
    messages::GetUsers, messages::GetGroups, messages::AddGroup,
    messages::EnterGroup, messages::MakeAdmin,
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
        Ok(Ok(info)) => {
            HttpResponse::Ok().json(info)
        },
        Ok(Err(_)) => HttpResponse::BadRequest().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users")
    }
}

#[get("/groups")]
pub async fn get_groups(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(GetGroups).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::BadRequest().json("No groups found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve groups")
    }
}


#[post("/users/add_group")]
pub async fn add_group(state: Data<AppState>, data: Json<AddGroup>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(data.0).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::BadRequest().json("The group with this name already exists"),
        _ => HttpResponse::InternalServerError().json("Unable to add group")
    }
}


#[post("/users/join_group")]
pub async fn join_group(state: Data<AppState>, data: Json<EnterGroup>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(data.0).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(error)) => HttpResponse::BadRequest().json(error.to_string()),
        _ => HttpResponse::InternalServerError().json("Unable to add group")
    }
}

#[post("/users/make_admin")]
pub async fn make_admin(state: Data<AppState>, data: Json<MakeAdmin>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(data.0).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(error)) => HttpResponse::Forbidden().json(error.to_string()),
        _ => HttpResponse::InternalServerError().json("Unable to make admin")
    }
}

