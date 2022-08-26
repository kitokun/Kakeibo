use crate::db::connect::{connect_db};
use crate::api::controller::request::money::Transaction;
use crate::api::controller::request::supplier::Supplier;

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

pub async fn select_all_transactions(pool: &sqlx::MySqlPool) -> Result<Vec<Transaction>, sqlx::Error> {
    println!("do");

    let transactions = sqlx::query_as::<_, Transaction>(
        "SELECT * FROM money_transactions"
    )
    .fetch_all(pool)
    .await?;

    Ok(transactions)
}

pub async fn create_new_supplier(pool: &sqlx::MySqlPool, supplier: &Supplier) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO transaction_suppliers (supplier) VALUES (?)")
    .bind(&supplier.supplier)
    .execute(pool)
    .await.unwrap();

    Ok(())
}