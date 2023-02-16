use crate::db::connect::{connect_db};
use crate::api::controller::request::money::Transaction;
use crate::api::controller::request::transaction_id::TransactionId;
use crate::api::store::money_transaction::
    {
        create_transaction,
        delete_transaction,
        select_all_transactions,
        get_sum_amount
    };

pub async fn store_transaction(transaction: &Transaction) -> Result<(), sqlx::Error> {
    let pool = connect_db().await?;

    create_transaction(&pool, transaction).await?;

    Ok(())
}

pub async fn delete_transaction_by_id(id: &TransactionId) -> Result<(), sqlx::Error> {
    let pool = connect_db().await?;

    delete_transaction(&pool, id).await?;

    Ok(())
}

pub async fn get_all_transactions() -> Result<Vec<Transaction>, sqlx::Error> {
    let pool = connect_db().await?;

    let transactions = select_all_transactions(&pool).await?;

    Ok(transactions)
}

pub async fn get_assets() -> Result<sqlx::types::BigDecimal, sqlx::Error> {
    let pool = connect_db().await?;

    Ok(get_sum_amount(&pool).await?)
}