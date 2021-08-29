use crate::actors::room;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ClientMessage {
    Move { from: String, to: String, fen: String },
    Create,
    List(usize),
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ServerMessage {
    Move {
        side: String,
        fen: String,
        dests: Option<HashMap<String, String>>,
    },
    GameEnd {
        result: room::GameEndResult,
    },
    Err {
        what: ServerError,
    },
    Create {
        room_id: String,
    },
    Start {
        color: room::PlayerColor,
        dests: Option<HashMap<String, String>>,
    },
    Reconnect {
        color: room::PlayerColor,
        turn: room::PlayerColor,
        fen: String,
        dests: Option<HashMap<String, String>>,
    },
    List{
        rooms: Vec<String>,
    },
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ServerError {
    InternalError,
    InvalidInput,
    IllegalMove,
    OutOfContext,
}
