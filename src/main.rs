mod api;
mod db;

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error>{
    println!("Hello, world!");
    api::controller::money_transaction::test();
    let pool = db::connect::connect_db().await?;

    println!("{:?}", pool);

    crate::db::init::init_db(&pool).await;

    loop{}
}
