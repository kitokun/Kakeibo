mod api;
mod db;

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error>{
    println!("Hello, world!");
    api::controller::money_transaction::test();
    let pool = db::connect::connect_db().await?;

    println!("{:?}", pool);

    sqlx::query("CREATE TABLE test_table(id INT(10) NOT NULL, text VARCHAR(30) NOT NULL);").execute(&pool).await?;

    loop{}
}
