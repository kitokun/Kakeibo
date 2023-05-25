use crate::api::controller::request::user::User;

pub async fn retrieve_all_users(pool: &sqlx::MySqlPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>(
        "SELECT * FROM users"
    )
    .fetch_all(pool)
    .await;

    match users {
        Ok(users) => {
            return Ok(users)
        },
        Err(error) => {
            return Err(error)
        },
    }
}

pub async fn retrieve_user_by_id(pool: &sqlx::MySqlPool, id: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await;

    match user {
        Ok(user) => {
            return Ok(user)
        },
        Err(error) => {
            return Err(error)
        },
    }
}

pub async fn save_user(pool: &sqlx::MySqlPool, user: &User) -> Result<(), sqlx::Error> {
    let result = sqlx::query("INSERT INTO users (id, name) VALUES (?, ?)")
    .bind(&user.id)
    .bind(&user.name)
    .execute(pool)
    .await;

    match result {
        Ok(_) => {
            return Ok(())
        },
        Err(error) => {
            return Err(error)
        },
    }
}
