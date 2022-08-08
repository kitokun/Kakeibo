use crate::api::controller::request::money::Transaction;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::api::service::service_money_transaction::{store_transaction, get_all_transactions};

#[post("/money_transaction/create")]
async fn create_new_transaction(new_transaction: web::Json<Transaction>) -> impl Responder {
    println!("new_transaction");
    println!("{:?}", new_transaction);
    store_transaction(&new_transaction).await.unwrap_or(());

    HttpResponse::Ok().json("ok")
}

#[get("/money_transaction/get")]
async fn get_transaction() -> HttpResponse {
    let transactions = get_all_transactions().await;

    match transactions {
        Ok(transactions) => {
            println!("{:?}", transactions);
            return HttpResponse::Ok().json(transactions)
        },
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string())
        },
    }

}