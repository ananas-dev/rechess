mod auth;
mod config;
mod room;
mod room_manager;
mod room_manager_ng;
mod users;
mod websocket;

use crate::config::Config;

use std::sync::{atomic::AtomicUsize, Arc};

use actix::prelude::*;
use actix_cors::Cors;
use actix_session::{CookieSession, Session};
use actix_web::{http, middleware::Logger, web, web::Data, App, HttpServer};
use color_eyre::Result;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<()> {
    // Inject .env into env var
    dotenv().ok();
    // Init the logger
    env_logger::init();
    // Init color eyre
    color_eyre::install()?;

    let config = Config::from_env().expect("Server configuration");

    let app_state = Arc::new(AtomicUsize::new(0));
    let pool = config.db_pool().await.expect("Data configuration");
    let redis = config.redis_con().await;
    let server = room_manager_ng::RoomManager::new().start();

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
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .app_data(Data::new(app_state.clone()))
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(redis.clone()))
            //.app_data(Data::new(redis.clone()))
            .app_data(Data::new(server.clone()))
            .service(web::scope("/ws").configure(websocket::config))
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
