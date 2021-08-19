use serde::{Serialize, Deserialize};
use color_eyre::Result;

#[derive(Deserialize)]
pub enum ClientMessage {
    Move {
        from: String,
        to: String,
        fen: String,
    },
}

#[derive(Serialize)]
pub enum ServerMessage {
    Move {
        from: String,
        to: String,
        fen: String,
    },
    Err(String),
}
