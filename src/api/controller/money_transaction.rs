use crate::api::controller::request::money::Transaction;
use crate::api::controller::request::transaction_id::TransactionId;
use crate::api::controller::request::asset::Asset;
use actix_web::{get, post, delete, web, App, HttpResponse, HttpServer, Responder};
use crate::api::service::money_transaction::
    {
        store_transaction,
        delete_transaction_by_id,
        get_all_transactions,
        get_assets
    };
use num_traits::cast::ToPrimitive;

#[post("/money_transaction/create")]
async fn create_new_transaction(new_transaction: web::Json<Transaction>) -> impl Responder {
    result = store_transaction(&new_transaction).await;
    match result {
        Ok(_) => {
            return HttpResponse::Ok().json("ok")
        },
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string())
        },
    }
    
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

#[delete("/money_transaction")]
async fn cancel_transaction(transaction_id: web::Query<TransactionId>) -> HttpResponse {
    
    println!("{}", &transaction_id.id);
    result = delete_transaction_by_id(&transaction_id).await;
    match result {
        Ok(_) => {
            return HttpResponse::Ok().json("ok")
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
