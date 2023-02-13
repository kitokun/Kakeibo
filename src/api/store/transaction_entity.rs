use crate::db::connect::{connect_db};
use crate::api::controller::request::transaction_entity::TransactionEntity;

pub async fn select_all_transaction_entities(pool: &sqlx::MySqlPool) -> Result<Vec<TransactionEntity>, sqlx::Error> {
    println!("do");

    let transactions = sqlx::query_as::<_, TransactionEntity>(
        "SELECT * FROM transaction_entities"
    )
    .fetch_all(pool)
    .await?;

    Ok(transactions)
}

pub async fn create_new_transaction_entity(pool: &sqlx::MySqlPool, transaction_entity: &TransactionEntity) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO transaction_entities (transaction_entity, transaction_entity_type) VALUES (?, ?)")
    .bind(&transaction_entity.transaction_entity)
    .bind(&transaction_entity.transaction_entity_type)
    .execute(pool)
    .await.unwrap();

    Ok(())
}
