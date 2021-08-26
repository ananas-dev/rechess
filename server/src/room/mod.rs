use crate::room_manager_ng::Disconnect;
use crate::websocket::model::{ServerError, ServerMessage};
use crate::websocket::{self, Send, WebsocketSession};
use actix::prelude::*;
use serde::Serialize;
use chess::{ChessMove, Game, Square};
use color_eyre::owo_colors::OwoColorize;
use log::info;
use rand::Rng;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use uuid::Uuid;
use std::str::FromStr;

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
    pub w: Option<Player>,
    pub b: Option<Player>,
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

// Main actor

pub struct Room {
    room_id: String,
    players: Players,
    spectators: HashSet<Spectator>,
    creator: Uuid,
    started: bool,
    game: Option<Game>,
}

impl Room {
    pub fn new(room_id: String, creator: Uuid) -> Self {
        Self {
            room_id,
            players: Players {
                w: None,
                b: None,
            },
            spectators: HashSet::new(),
            creator,
            started: false,
            game: None,
        }
    }
}

impl Room {
    fn send_message(&self, message: ServerMessage, to: UserType) {
        // TODO: Make more efficient
        match to {
            UserType::Spectator => {
                for spectator in self.spectators.iter() {
                    &spectator.session.do_send(Send(message.clone()));
                }
            }
            UserType::Player(color) => {
                match color {
                    PlayerColor::White => {
                        if let Some(player) = &self.players.w {
                            if let Some(session) = &player.session {
                                session.do_send(Send(message.clone()));
                            }
                        }
                    },
                    PlayerColor::Black => {
                        if let Some(player) = &self.players.b {
                            if let Some(session) = &player.session {
                                session.do_send(Send(message.clone()));
                            }
                        }
                    }
                    PlayerColor::All => {
                        if let Some(player) = &self.players.b {
                            if let Some(session) = &player.session {
                                session.do_send(Send(message.clone()));
                            }
                        }
                        if let Some(player) = &self.players.w {
                            if let Some(session) = &player.session {
                                session.do_send(Send(message.clone()));
                            }
                        }
                    }
                };
            }
            /*
            UserType::Creator => {
                if let Some(session) = &self.creator.session {
                    session.do_send(Send(message.clone()));
                }
            }
             */
        }
    }
}

impl Actor for Room {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Started room !");
    }
}

impl Handler<Join> for Room {
    type Result = ();

    fn handle(&mut self, msg: Join, _ctx: &mut Self::Context) -> Self::Result {
        if self.started {
            self.spectators.insert(Spectator {
                id: msg.id,
                session: msg.session,
            });
            info!("New spectator in room {}", &self.room_id);
        } else if msg.id == self.creator {
            // Randomly decide which player will play which side
            let player = Some(Player {
                id: msg.id,
                session: Some(msg.session),
            });

            if rand::thread_rng().gen_bool(0.5) {
                self.players.w = player;
            } else {
                self.players.b = player;
            }
        } else {
            let player = Some(Player {
                id: msg.id,
                session: Some(msg.session),
            });

            if let Some(_) = self.players.w {
                self.players.b = player;
            } else if let Some(_) = self.players.b {
                self.players.w = player;
            }

            self.game = Some(Game::new());
            self.started = true;

            // Send messages

            self.send_message(
                ServerMessage::Start {
                    color: PlayerColor::Black,
                },
                UserType::Player(PlayerColor::Black),
            );

            self.send_message(
                ServerMessage::Start {
                    color: PlayerColor::White,
                },
                UserType::Player(PlayerColor::White),
            );
        }
    }
}

impl Handler<Leave> for Room {
    type Result = ();

    fn handle(&mut self, msg: Leave, ctx: &mut Self::Context) -> Self::Result {
        if let Some(white_player) = &mut self.players.w {
            if white_player.id == msg.id {
                white_player.session = None;
            }
        } else if let Some(black_player) = &mut self.players.b {
            if black_player.id == msg.id {
                black_player.session = None;
            }
        } else {
            self.spectators.remove(&msg.id);
        }
    }
}

impl Handler<Move> for Room {
    type Result = ();

    fn handle(&mut self, msg: Move, ctx: &mut Self::Context) -> Self::Result {
        if self.started {
            if let Some(game) = &mut self.game {

                let player_color = match game.side_to_move() {
                    chess::Color::Black => PlayerColor::White,
                    chess::Color::White => PlayerColor::Black,
                };

                let side = match player_color {
                    PlayerColor::White => "white",
                    PlayerColor::Black => "black",
                    PlayerColor::All => "",
                }.to_string();

                match ChessMove::from_str(
                    &format!("{}{}", msg.from, msg.to),
                ) {
                    Ok(chess_move) => {
                        if game.make_move(chess_move) {
                            let fen = game.current_position().to_string();

                            self.send_message(ServerMessage::Move {
                                from: msg.from,
                                to: msg.to,
                                side,
                                fen
                            }, UserType::Player(player_color));
                        } else {
                            self.send_message(ServerMessage::Err {
                                what: ServerError::IllegalMove,
                            }, UserType::Player(player_color))
                        }
                    }
                    Err(_e) => {
                        self.send_message(ServerMessage::Err {
                            what: ServerError::IllegalMove,
                        }, UserType::Player(player_color))
                    }
                }
            }
        }
    }
}
