use crate::api::controller::request::transaction_entity::TransactionEntity;
use crate::api::service::transaction_entity::
    {
        store_transaction_entity,
        get_all_transaction_entities
    };
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[post("/transaction_entity")]
async fn register_transaction_entity(new_transaction_entity: web::Json<TransactionEntity>) -> impl Responder {
    store_transaction_entity(&new_transaction_entity).await.unwrap_or(());

    HttpResponse::Ok().json("ok")
}

#[get("/transaction_entity")]
async fn get_transaction_entities() -> HttpResponse {
    let transaction_entities = get_all_transaction_entities().await;

    match transaction_entities {
        Ok(transaction_entities) => {
            return HttpResponse::Ok().json(transaction_entities)
        },
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string())
        },
    }
}