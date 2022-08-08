use crate::db::connect::{connect_db};
use crate::api::controller::request::money::Transaction;
use crate::api::store::store_money_transaction::{create_transaction, select_all_transactions};

pub async fn store_transaction(transaction: &Transaction) -> Result<(), sqlx::Error> {
    let pool = connect_db().await?;

    create_transaction(&pool, transaction).await?;

    Ok(())
}

pub async fn get_all_transactions() -> Result<Vec<Transaction>, sqlx::Error> {
    let pool = connect_db().await?;

    let transactions = select_all_transactions(&pool).await?;

    Ok(transactions)
}