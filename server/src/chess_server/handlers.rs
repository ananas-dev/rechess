use super::server;
use super::ws::WsChessSession;

use actix::prelude::*;
use actix_web::{get, web, web::ServiceConfig, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use std::time::Instant;

pub fn config(config: &mut ServiceConfig) {
    config.service(chess_room);
}

#[get("/{room_name}")]
pub async fn chess_room(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChessServer>>,
    room_name: web::Path<String>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsChessSession {
            id: 0,
            hb: Instant::now(),
            room_name: room_name.to_string(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}
