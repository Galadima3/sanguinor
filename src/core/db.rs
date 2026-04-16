use anyhow::Ok;
use axum_session::{Key, SessionConfig, SessionStore};
use axum_session_sqlx::SessionSqlitePool;
use sqlx::{Executor, SqlitePool};

pub async fn init_db() -> anyhow::Result<SqlitePool> {
    use sqlx::sqlite::SqliteConnectOptions;
    use std::str::FromStr;

    let options = SqliteConnectOptions::from_str("sqlite://db.sqlite")?.create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;

    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL
        )
    "#,
    )
    .await?;

    let guest_exists: Option<(i32,)> = sqlx::query_as("SELECT ID FROM users WHERE id = ?1")
        .bind(1)
        .fetch_optional(&pool)
        .await?;

    if guest_exists.is_none() {
        sqlx::query("INSERT INTO users (username, password) VALUES (?1, ?2)")
            .bind("guest")
            .bind("guest")
            .execute(&pool)
            .await?;
    }

    Ok(pool)
}

pub async fn init_session(pool: SqlitePool) -> anyhow::Result<SessionStore<SessionSqlitePool>> {
    let config = SessionConfig::default()
        .with_table_name("sessions")
        .with_key(Key::generate());

    let store = SessionStore::<SessionSqlitePool>::new(Some(pool.clone().into()), config).await?;

    Ok(store)
}
