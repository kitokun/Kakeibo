use crate::db::connect::{connect_db};
use crate::api::controller::request::money::Transaction;

pub async fn create_transaction(pool: &sqlx::MySqlPool, transaction: &Transaction) -> Result<(), sqlx::Error> {
    println!("do");

    sqlx::query(
        "INSERT INTO money_transactions (destination, source, amount, nominal, description) 
        VALUES (?, ?, ?, ?, ?)
        "
    )
    .bind(&transaction.destination)
    .bind(&transaction.source)
    .bind(&transaction.amount)
    .bind(&transaction.nominal)
    .bind(&transaction.description)
    .execute(pool)
    .await.unwrap();

    println!("done");
    Ok(())
}

pub async fn select_all_transactions(pool: &sqlx::MySqlPool) -> Result<Vec<Transaction>, sqlx::Error> {
    println!("do");

    let transactions = sqlx::query_as::<_, Transaction>(
        "SELECT * FROM money_transactions"
    )
    .fetch_all(pool)
    .await?;

    Ok(transactions)
}
