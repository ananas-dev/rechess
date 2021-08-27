use super::websocket::{self, WebsocketSession};
use super::room::Room;
use actix::prelude::*;
use uuid::Uuid;
use std::time::Instant;

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
    pub created_at: Instant,
    pub addr: Addr<Room>,
}

