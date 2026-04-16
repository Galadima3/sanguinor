use async_trait::async_trait;
use axum_session_auth::Authentication;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, prelude::FromRow};

#[derive(Clone)]
pub struct User {
    pub id: i64,
    pub anonymous: bool,
    pub username: String,
}
#[async_trait]
impl Authentication<User, i64, SqlitePool> for User {
    async fn load_user(userid: i64, pool: Option<&SqlitePool>) -> Result<User, anyhow::Error> {
        if userid == 1 {
            Ok(User {
                id: userid,
                anonymous: true,
                username: "guest".to_string(),
            })
        } else {
            let user: UserSql = sqlx::query_as("SELECT * FROM users WHERE id = ?1")
                .bind(&userid)
                .fetch_one(pool.unwrap())
                .await?;
                
            Ok(User {
                id: user.id as i64,
                anonymous: false,
                username: user.username,
            })
        }
    }

    fn is_active(&self) -> bool {
        !self.anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }
    fn is_authenticated(&self) -> bool {
        !self.anonymous
    }
}

#[derive(Deserialize, Serialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
}

#[derive(FromRow)]
pub struct UserSql {
    pub id: i64,
    pub username: String,
    pub password: String, //Password Hash
}


