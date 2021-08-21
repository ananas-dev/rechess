mod auth;
mod chess_server;
mod config;
mod users;
mod redis;

use crate::config::Config;

use std::sync::{atomic::AtomicUsize, Arc};

use actix::prelude::*;
use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web, web::Data, App, HttpServer};
use color_eyre::Result;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = Config::from_env().expect("Server configuration");

    let app_state = Arc::new(AtomicUsize::new(0));
    let pool = config.db_pool().await.expect("Data configuration");
    let redis = config.redis_con().await;
    let server = chess_server::ChessServer::new(app_state.clone()).start();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(Data::new(app_state.clone()))
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(redis.clone()))
            //.app_data(Data::new(redis.clone()))
            .app_data(Data::new(server.clone()))
            .service(web::scope("/ws").configure(chess_server::config))
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .service(web::scope("/users").configure(users::config))
                        .service(web::scope("/auth").configure(auth::config)),
                ),
            )
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
