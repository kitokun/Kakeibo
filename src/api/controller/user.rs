use crate::api::controller::request::user::User;
use crate::api::service::user::{
    create_user,
    obtain_all_users,
    obtain_user_by_id
};

use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/users")]
async fn get_all_users() -> impl Responder {
    let result = obtain_all_users().await;
    match result {
        Ok(users) => {
            return HttpResponse::Ok().json(users)
        },
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string())
        },
    }
}

#[get("/user/{id}")]
async fn get_user_by_id(user_id: web::Path<u32>) -> impl Responder {
    let str_user_id = user_id.to_string();
    let result = obtain_user_by_id(&str_user_id).await;
    match result {
        Ok(user) => {
            return HttpResponse::Ok().json(user)
        },
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string())
        },
    }
}

#[post("/user")]
async fn register_user(new_user: web::Json<User>) -> impl Responder {
    let result = create_user(&new_user).await;
    match result {
        Ok(_) => {
            return HttpResponse::Ok().json("ok")
        },
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string())
        },
    }
}