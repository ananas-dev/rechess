use actix::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ClientMessage {
    Move {
        san: String,
        fen: String,
    },
}

#[derive(Serialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ServerMessage {
    Move {
        san: String,
        side: String,
        fen: String,
    },
    Err {
        what: ServerError,
    },
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ServerError {
    InternalError,
    InvalidInput,
    IllegalMove,
}
