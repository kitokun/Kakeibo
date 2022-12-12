use crate::db::connect::{connect_db};
use crate::api::controller::request::money::Transaction;
use crate::api::controller::request::supplier::Supplier;
use crate::api::store::store_money_transaction::
    {create_transaction,
     select_all_transactions,
     create_new_supplier,
     get_sum_amount};

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

pub async fn store_supplier(supplier: &Supplier) -> Result<(), sqlx::Error> {
    let pool = connect_db().await?;

    create_new_supplier(&pool, supplier).await?;

    Ok(())
}

pub async fn get_assets() -> Result<sqlx::types::BigDecimal, sqlx::Error> {
    let pool = connect_db().await?;

    Ok(get_sum_amount(&pool).await?)
}