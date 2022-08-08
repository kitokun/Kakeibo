mod api;
mod db;
extern crate env_logger;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let pool = db::connect::connect_db().await?;

    println!("{:?}", pool);

    crate::db::init::init_db(&pool).await?;

    actix_web::HttpServer::new(|| {
        let cors = actix_cors::Cors::default()
                .allowed_origin("http://localhost:3000")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_header(actix_web::http::header::CONTENT_TYPE);
        actix_web::App::new()
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .service(crate::api::controller::money_transaction::create_new_transaction)
            .service(crate::api::controller::money_transaction::get_transaction)})
            .bind("0.0.0.0:8080")?
            .run()
            .await?;

    Ok(())
}
