use crate::db::connect::{connect_db};
use crate::api::controller::request::money::Transaction;
use crate::api::controller::request::transaction_id::TransactionId;
pub async fn create_transaction(pool: &sqlx::MySqlPool, transaction: &Transaction) -> Result<(), sqlx::Error> {

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

    Ok(())
}

pub async fn delete_transaction(pool: &sqlx::MySqlPool, id: &TransactionId) -> Result<(), sqlx::Error> {

    println!("{}", &id.id);
    sqlx::query(
        "DELETE FROM money_transactions where id = ?"
    )
    .bind(&id.id)
    .execute(pool)
    .await.unwrap();

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

pub async fn get_sum_amount(pool: &sqlx::MySqlPool) -> Result<sqlx::types::BigDecimal, sqlx::Error> {
    let raw_result = sqlx::query!(
        "SELECT sum(amount) as sum FROM money_transactions"
    )
    .fetch_one(pool)
    .await?;

    let result = raw_result.sum.unwrap();

    Ok(result)
}