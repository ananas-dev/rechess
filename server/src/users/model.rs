use chrono::NaiveDateTime;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool};
use uuid::Uuid;
use validator_derive::Validate;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 5))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

impl User {
    pub async fn create(pool: &PgPool, new_user: NewUser) -> Result<User> {
        let password_hash = "aa";

        let user = sqlx::query_as!(
            User,
            "insert into users (username, email, password_hash) values ($1, $2, $3) returning *",
            new_user.username,
            new_user.email,
            password_hash
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>> {
        let users = sqlx::query_as!(User, "select * from users")
            .fetch_all(pool)
            .await?;

        Ok(users)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<User> {
        let user = sqlx::query_as!(User, "select * from users where id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool> {
        let user = sqlx::query!("delete from users where id = $1", id)
            .execute(pool)
            .await?;

        Ok(user.rows_affected() == 1)
    }
}
