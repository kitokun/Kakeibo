use crate::api::controller::request::money::Transaction;
use crate::api::controller::request::supplier::Supplier;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::api::service::service_money_transaction::
    {store_transaction, get_all_transactions, store_supplier};

#[post("/money_transaction/create")]
async fn create_new_transaction(new_transaction: web::Json<Transaction>) -> impl Responder {
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

#[post("/supplier/register")]
async fn register_supplier(new_supplier: web::Json<Supplier>) -> impl Responder {
    store_supplier(&new_supplier).await.unwrap_or(());

    HttpResponse::Ok().json("ok")
}