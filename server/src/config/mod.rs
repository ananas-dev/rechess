use crate::redis::{InfoCommand, RedisActor};

use actix::prelude::*;
use futures::prelude::*;
use color_eyre::{Result, owo_colors::OwoColorize};
use eyre::WrapErr;
use log::info;
use serde::Deserialize;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use redis::aio::Connection;
use std::sync::Arc;

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

        envy::from_env().context("loading configuration from environement")
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
        RedisActor::new(&self.redis_url).await.start()
    }
}