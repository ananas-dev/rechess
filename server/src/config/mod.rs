use actix::prelude::*;
use actix_redis::RedisActor;
use color_eyre::{owo_colors::OwoColorize, Result};
use eyre::WrapErr;
use futures::prelude::*;
use log::info;
use serde::Deserialize;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use std::time::Duration;

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub database_url: String,
    pub redis_url: String,
    pub secret_key: String,
}

impl Config {
    pub fn from_env() -> Result<Config> {
        info!("Loading configuration");

        envy::from_env().context("loading configuration from environment")
    }

    pub async fn db_pool(&self) -> Result<PgPool> {
        info!("Creating database connection pool");

        PgPoolOptions::new()
            .connect_timeout(Duration::from_secs(30))
            .connect(&self.database_url)
            .await
            .context("creating database connection pool")
    }

    pub async fn redis_con(&self) -> Addr<RedisActor> {
        RedisActor::start(&self.redis_url)
    }
}
