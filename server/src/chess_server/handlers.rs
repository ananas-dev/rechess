use crate::redis::{RedisActor, InfoCommand};
use super::server;
use super::ws::WsChessSession;

use actix::prelude::*;
use actix_web::{get, web, web::ServiceConfig, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use std::time::Instant;
use uuid::Uuid;

pub fn config(config: &mut ServiceConfig) {
    config.service(chess_room);
}

#[get("/{room_name}")]
pub async fn chess_room(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChessServer>>,
    redis_con: web::Data<Addr<RedisActor>>,
    game_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let test_res = redis_con.send(InfoCommand).await.unwrap().unwrap().unwrap();

    println!("{}", &test_res);

    ws::start(
        WsChessSession {
            id: 0,
            hb: Instant::now(),
            game_id: game_id.to_string(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}
