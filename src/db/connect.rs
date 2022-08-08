use sqlx::mysql::MySqlPool;
use std::env;

pub async fn connect_db() -> Result<MySqlPool, sqlx::Error> {
    println!("connect_db");

    let database = env::var("MYSQL_DATABASE").expect("MYSQL_DATABASE is not set");
    let user = env::var("MYSQL_USER").expect("MYSQL_USER is not set");
    let password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD is not set");
    let port = env::var("MYSQL_PORT").unwrap_or("3306".to_string());
    let host = env::var("MYSQL_HOST").unwrap();

    let database_url = format!(
         "mysql://{}:{}@{}:{}/{}",
        user, password, host, port, database
    );

    println!("{}", database_url);

    let pool = sqlx::mysql::MySqlPoolOptions::new()
    .max_connections(20)
    .connect(&database_url)
    .await
    .unwrap();
    
    println!("{:?}", pool);

    Ok(pool)
}