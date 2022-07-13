use crate::api::controller::request::money::Transaction;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[post("/money_transaction/create")]
async fn create_transaction(new_transaction: web::Json<Transaction>) -> impl Responder {
    println!("new_transaction");
    println!("{:?}", new_transaction);
    HttpResponse::Ok().body("ok")
}

#[get("/money_transaction/get")]
async fn get_transaction() -> impl Responder {
    HttpResponse::Ok().body("good")
}