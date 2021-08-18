use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use color_eyre::Result;
use rand_core::OsRng;
use std::sync::Arc;

pub struct CryptoService {
    pub key: Arc<String>,
}

impl CryptoService {
    pub async fn hash_password(&self, password: String) -> Result<String> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        Ok(argon2
            .hash_password_simple(&password.as_bytes(), salt.as_ref())
            .unwrap()
            .to_string())
    }
}
