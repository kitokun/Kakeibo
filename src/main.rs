mod api;
mod db;
extern crate env_logger;

use crate::api::controller::money_transaction::{get_transaction, post_transaction, delete_transaction, get_asset};
use crate::api::controller::transaction_entity::{get_transaction_entities, post_transaction_entity};
use crate::api::controller::user::{get_all_users, get_user_by_id, register_user};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let pool = db::connect::connect_db().await?;

    println!("{:?}", pool);

    //crate::db::init::init_db(&pool).await?;

    actix_web::HttpServer::new(|| {
        let cors = actix_cors::Cors::default()
                .allowed_origin("http://localhost:3000")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_header(actix_web::http::header::CONTENT_TYPE);
        actix_web::App::new()
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            
            .service(
                actix_web::web::scope("/api")
                    .service(get_transaction)
                    .service(post_transaction)
                    .service(delete_transaction)
                    .service(get_asset)
                    .service(get_transaction_entities)
                    .service(post_transaction_entity)
                    .service(get_all_users)
                    .service(get_user_by_id)
                    .service(register_user)
            )
        })
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    Ok(())
}
