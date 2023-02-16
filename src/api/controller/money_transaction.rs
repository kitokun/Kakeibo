use crate::api::controller::request::money::Transaction;
use crate::api::controller::request::asset::Asset;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::api::service::money_transaction::
    {store_transaction, get_all_transactions, get_assets};
use num_traits::cast::ToPrimitive;

#[post("/money_transaction/create")]
async fn create_new_transaction(new_transaction: web::Json<Transaction>) -> impl Responder {
    store_transaction(&new_transaction).await.unwrap_or(());

    HttpResponse::Ok().json("ok")
}

#[get("/money_transaction")]
async fn get_transaction() -> HttpResponse {
    let transactions = get_all_transactions().await;

    match transactions {
        Ok(transactions) => {
            return HttpResponse::Ok().json(transactions)
        },
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string())
        },
    }
}

#[get("/asset")]
async fn assets() -> HttpResponse {
    let raw_asset = get_assets().await;


    match raw_asset {
        Ok(raw_asset) => {
            println!("{:?}", raw_asset.to_i64().unwrap());
            let asset = Asset {
                amount: raw_asset.to_i64().unwrap(),
            };
            return HttpResponse::Ok().json(asset)
        },
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string())
        },
    }
}
