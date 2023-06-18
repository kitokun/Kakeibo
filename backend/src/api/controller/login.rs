use crate ::api::controller::request::login::LoginForm;
use crate ::api::service::user::obtain_user_by_name;

use actix_web::{post, web, HttpResponse, Responder};

#[post("/login")]
async fn login(login_form: web::Json<LoginForm>) -> impl Responder {
    if let Err(error) = login_form.validate() {
        return HttpResponse::BadRequest().json(error)
    }

    let result = obtain_user_by_name(&login_form.user_name).await;
    match result {
        Ok(user) => {
            // TODO: compare password
            return HttpResponse::Ok().json(user)
        },
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string())
        },
    }
}