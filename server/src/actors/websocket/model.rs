use crate::actors::room;

use serde::{Deserialize, Serialize};

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
        from: String,
        to: String,
        side: String,
        fen: String,
    },
    Err {
        what: ServerError,
    },
    Create {
        room_id: String,
    },
    Start {
        color: room::PlayerColor,
    },
    Reconnect {
        color: room::PlayerColor,
        turn: room::PlayerColor,
        fen: String,
    },
    List{
        rooms: Vec<String>,
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ServerError {
    InternalError,
    InvalidInput,
    IllegalMove,
    OutOfContext,
}
