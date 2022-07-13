mod api;
mod db;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error>{
    //let pool = db::connect::connect_db().await?;

    //println!("{:?}", pool);

    //crate::db::init::init_db(&pool).await;

    actix_web::HttpServer::new(|| actix_web::App::new()
        .service(crate::api::controller::money_transaction::create_transaction)
        .service(crate::api::controller::money_transaction::get_transaction))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
