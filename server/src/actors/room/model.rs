use super::websocket::WebsocketSession;

use actix::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;
use std::ops::{Index, IndexMut};
use chess::{Game, Color};

// Types

pub enum UserType {
    Spectator,
    Player(PlayerColor),
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PlayerColor {
    White,
    Black,
    All,
}

// User storage data structures

pub struct Players {
    pub w: Player,
    pub b: Player,
}

#[derive(Clone, Eq)]
pub struct Spectator {
    pub id: Uuid,
    pub session: Addr<WebsocketSession>,
}

impl PartialEq for Spectator {
    fn eq(&self, other: &Spectator) -> bool {
        self.id == other.id
    }
}

impl Hash for Spectator {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Borrow<Uuid> for Spectator {
    fn borrow(&self) -> &Uuid {
        &self.id
    }
}

#[derive(Clone)]
pub struct Player {
    pub id: Uuid,
    pub session: Option<Addr<WebsocketSession>>,
}

// Actor messages

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub id: Uuid,
    pub session: Addr<WebsocketSession>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Leave {
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Move {
    pub id: Uuid,
    pub from: String,
    pub to: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Start;
