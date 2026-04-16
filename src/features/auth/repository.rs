use sqlx::SqlitePool;
use crate::features::auth::model::UserSql;

pub async fn user_exists(pool: &SqlitePool, username: &str) -> bool {
    sqlx::query("SELECT 1 FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(pool)
        .await
        .unwrap()
        .is_some()
}

pub async fn get_user_by_username(pool: &SqlitePool, username: &str) -> Result<Option<UserSql>, sqlx::Error> {
    let user = sqlx::query_as::<_, UserSql>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(pool)
        .await?;

    Ok(user)
        
}

pub async fn insert_user(
    pool: &SqlitePool,
    username: &str,
    hashed_password: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (username, password) VALUES (?, ?)")
        .bind(username)
        .bind(hashed_password)
        .execute(pool)
        .await?;

    Ok(())
}
