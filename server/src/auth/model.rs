use color_eyre::Result;
use serde::Deserialize;
use sqlx::PgPool;
use validator_derive::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct Auth {
    #[validate(length(min = 5))]
    pub username: String,
    #[validate(length(min = 8))]
    pub password: String,
}

/*
impl AuthData {
    pub async fn validate_creds(pool: &PgPool, data: AuthData) -> Result<bool> {

    }
}
*/
