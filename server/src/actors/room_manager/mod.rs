pub mod model;

pub use model::*;

use super::room::{self, Room};
use super::websocket;
use super::websocket::model::ServerMessage;
use actix::prelude::*;
use actix_redis::RedisActor;
use indexmap::IndexMap;
use log::{info};
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::collections::HashMap;
use std::time::Instant;
use uuid::Uuid;

pub struct RoomManager {
    sessions: HashMap<Uuid, Recipient<websocket::Send>>,
    rooms: IndexMap<String, RoomData>,
    redis: Addr<RedisActor>,
}

impl RoomManager {
    pub fn new(redis: Addr<RedisActor>) -> Self {
        Self {
            sessions: HashMap::new(),
            rooms: IndexMap::new(),
            redis,
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
                msg.session
                    .do_send(websocket::JoinedRoom(room.addr.clone()));
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
            addr: Room::new(room_id.clone(), msg.id, self.redis.clone().recipient()).start(),
            created_at: Instant::now(),
        };

        self.rooms.insert(room_id.clone(), room.clone());

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
