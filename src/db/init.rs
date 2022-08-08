use std::fs;

pub async fn init_db(pool: &sqlx::MySqlPool) -> Result<(), Box<dyn std::error::Error>>{
    let init_query = fs::read_to_string("./init.sql")?;
    println!("{}", init_query);
    sqlx::query(&init_query).execute(pool).await?;
    Ok(())
}