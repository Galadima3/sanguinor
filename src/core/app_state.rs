use sqlx::SqlitePool;

#[derive(Clone)]
pub struct _AppState <'a> {
    pub db: &'a SqlitePool,
}