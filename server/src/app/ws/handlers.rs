use crate::actors::websocket::{Connection, WebsocketSession};
use crate::actors::room_manager;

use actix::prelude::*;
use actix_redis::RedisActor;
use actix_web::{get, web, web::ServiceConfig, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use actix_session::{Session};

use log::info;
use std::time::Instant;
use uuid::Uuid;

pub fn config(config: &mut ServiceConfig) {
    config.service(join_room).service(join_lobby);
}

#[get("/play/{room_name}")]
pub async fn join_room(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<room_manager::RoomManager>>,
    room_id: web::Path<String>,
    session: Session,
) -> impl Responder {
    if let Some(id) = session.get::<uuid::Uuid>("rc-id")? {
        ws::start(
            WebsocketSession::new(
                id,
                Connection::Play(room_id.clone()),
                srv.get_ref().clone(),
            ),
            &req,
            stream,
        )
    } else {
        let id = uuid::Uuid::new_v4();
        session.insert("rc-id", id)?;
        ws::start(
            WebsocketSession::new(
                id,
                Connection::Play(room_id.clone()),
                srv.get_ref().clone(),
            ),
            &req,
            stream,
        )
    }
}

#[get("/")]
pub async fn join_lobby(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<room_manager::RoomManager>>,
    session: Session,
) -> impl Responder {
    if let Some(id) = session.get::<uuid::Uuid>("rc-id")? {
        ws::start(
            WebsocketSession::new(
                id,
                Connection::Lobby,
                srv.get_ref().clone(),
            ),
            &req,
            stream,
        )
    } else {
        let id = uuid::Uuid::new_v4();
        session.insert("rc-id", id)?;
        ws::start(
            WebsocketSession::new(
                id,
                Connection::Lobby,
                srv.get_ref().clone(),
            ),
            &req,
            stream,
        )
    }
}
