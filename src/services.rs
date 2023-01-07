use std::ops::DerefMut;
use actix_web::{App, get, HttpResponse, HttpServer, post, Responder};
use actix_web::web::{Json, Path, Data};

use serde::{Deserialize};


use crate::{
    messages::GetUsers, messages::GetGroups, messages::AddGroup,
    messages::EnterGroup, messages::MakeAdmin, messages::GetIdFromLogin,
    messages::Resign,
    AppState, DbActor
};

use crate::errors::Errors;
use crate::errors::Errors::{AccessDenied, NotUpdated, CantFindGroupByName, NotUniqueGroupName};

use actix::Addr;
use actix::fut::err;


#[post("/get_login_id")]
pub async fn get_id_from_login(state: Data<AppState>, data: Json<String>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let str_struct = GetIdFromLogin {
        login: data.0
    };

    match db.send(str_struct).await {
        Ok(Ok(info)) =>{
            HttpResponse::Ok().json(info)
        },
        Ok(Err(Errors)) => HttpResponse::InternalServerError().json("Error in db!"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users")
    }
}

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
pub async fn add_group(state: Data<AppState>, data: Json<MakeAdmin>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let msg = AddGroup {
        name: data.0.group_name,
        user_id: data.0.user_id
    };

    match db.send(msg).await {
        Ok(Ok(info)) => {
            let name = info.name;
            HttpResponse::Ok().json(format!("Group with name {name}"))
        },
        Ok(Err(error)) => {
            match error {
                Errors::NotUniqueGroupName => HttpResponse::NotAcceptable().json("Group already exists!"),
                Errors::NotUpdated => HttpResponse::NotModified().json("Unable to add in user_group table!"),
                _ => HttpResponse::InternalServerError().json("Something went wrong!")
            }
        }
        _ => HttpResponse::InternalServerError().json("Unable to connect!")
    }
}


#[post("/users/join_group")]
pub async fn join_group(state: Data<AppState>, data: Json<MakeAdmin>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let msg = EnterGroup {
        name: data.0.group_name.clone(),
        user_id: data.0.user_id.clone()
    };

    match db.send(msg).await {
        Ok(Ok(info)) => {
            HttpResponse::Ok().json(format!("You succesfully join group with name {name}", name = data.group_name))
        },
        Ok(Err(error)) => {
            match error {
                Errors::CantFindGroupByName => HttpResponse::NotAcceptable().json("Can't find group with this name!"),
                Errors::NotUpdated => HttpResponse::Conflict().json("You are already in group!"),
                _ => HttpResponse::InternalServerError().json("Something went wrong!")
            }
        }
        _ => HttpResponse::InternalServerError().json("Unable to connect!")
    }
}

#[post("/users/make_admin")]
pub async fn make_admin(state: Data<AppState>, data: Json<MakeAdmin>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let copy = data.0.clone();

    match db.send(data.0).await {
        Ok(Ok(info)) => {
            HttpResponse::Ok().json(format!("User {x} is now admin in group {y}", x = copy.admin_name, y = copy.admin_name))
        },
        Ok(Err(error)) => {
            match error {
                Errors::CantFindUserName => HttpResponse::NotAcceptable().json("Can't find user with this name!"),
                Errors::CantFindGroupByName => HttpResponse::NotAcceptable().json("Can't find group with this name!"),
                Errors::AccessDenied => HttpResponse::Forbidden().json("Access denied. You are not an admin!"),
                Errors::NotUpdated => HttpResponse::Conflict().json("This user is already admin or he doesn't in this group"),
                _ => HttpResponse::InternalServerError().json("Something went wrong!")
            }
        }
        _ => HttpResponse::InternalServerError().json("Unable to make admin")
    }
}

#[post("/users/resign")]
pub async fn resign(state: Data<AppState>, data: Json<MakeAdmin>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    let copy = data.0.clone();

    let msg = Resign {
        group_name: data.group_name.clone(),
        user_id: data.user_id.clone()
    };

    match db.send(msg).await {
        Ok(Ok(info)) => {
            HttpResponse::Ok().json(format!("Now you are not an admin in group {x}", x = copy.group_name))
        },
        Ok(Err(error)) => {
            match error {
                Errors::CantFindGroupByName => HttpResponse::NotAcceptable().json("Can't find group with this name!"),
                Errors::AloneAdmin => HttpResponse::Forbidden().json("You are alone admin! Don't leave your post!!!!"),
                Errors::NotUpdated => HttpResponse::Conflict().json(format!("You were not an admin in group {x}", x = copy.group_name)),
                _ => HttpResponse::InternalServerError().json("Something went wrong!")
            }
        }
        _ => HttpResponse::InternalServerError().json("Unable to connect!")
    }
}

