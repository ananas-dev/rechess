use crate::room::{self, Room};
use crate::websocket;
use crate::websocket::model::ServerMessage;
use crate::websocket::WebsocketSession;
use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use color_eyre::owo_colors::OwoColorize;
use color_eyre::{Report, Result};
use futures::{FutureExt, TryFutureExt};
use log::{debug, info};
use rand::distributions::Alphanumeric;
use rand::Rng;
use redis_async::{resp::RespValue, resp_array};
use std::collections::HashMap;
use indexmap::IndexMap;
use std::time::Instant;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: Uuid,
    pub session: Recipient<websocket::Send>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub id: Uuid,
    pub room_id: String,
    pub session: Addr<WebsocketSession>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
    pub session: Recipient<websocket::Send>,
}

#[derive(Message)]
#[rtype(result = "String")]
pub struct Create {
    pub id: Uuid,
    pub session: Addr<WebsocketSession>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct List {
    pub id: Uuid,
    pub items: usize,
    pub session: Recipient<websocket::Send>,
}

#[derive(Clone)]
pub struct RoomData {
    created_at: Instant,
    addr: Addr<Room>,
}

pub struct RoomManager {
    sessions: HashMap<Uuid, Recipient<websocket::Send>>,
    rooms: IndexMap<String, RoomData>,
}

impl RoomManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            rooms: IndexMap::new(),
        }
    }
}

impl Actor for RoomManager {
    type Context = Context<Self>;
}

impl Handler<Connect> for RoomManager {
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        self.sessions.insert(msg.id, msg.session);
    }
}

impl Handler<Join> for RoomManager {
    type Result = ();

    fn handle(&mut self, msg: Join, ctx: &mut Self::Context) -> Self::Result {
        match self.rooms.get(&msg.room_id) {
            Some(room) => {
                room.addr.do_send(room::Join {
                    session: msg.session.clone(),
                    id: msg.id,
                });
                msg.session.do_send(websocket::JoinedRoom(room.addr.clone()));
            }
            None => (),
        };
    }
}

impl Handler<Create> for RoomManager {
    type Result = String;

    fn handle(&mut self, msg: Create, ctx: &mut Self::Context) -> Self::Result {
        let room_id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();

        info!("Creating new room with id: {}", room_id);

        let room = RoomData {
                addr: Room::new(room_id.clone(), msg.id).start(),
                created_at: Instant::now(),
            };

        self.rooms.insert(
            room_id.clone(),
            room.clone(),
        );

        msg.session.do_send(websocket::Send(ServerMessage::Create {
            room_id: room_id.clone(),
        }));

        room_id
    }
}

impl Handler<Disconnect> for RoomManager {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {}
}

impl Handler<List> for RoomManager {
    type Result = ();

    fn handle(&mut self, msg: List, ctx: &mut Self::Context) -> Self::Result {
        msg.session.do_send(websocket::Send(ServerMessage::List {
            rooms: self.rooms.keys().cloned().rev().take(msg.items).collect(),
        }));
    }
}
