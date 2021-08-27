pub mod model;

pub use model::*;

use super::websocket::model::{ServerError, ServerMessage};
use super::websocket::{self, Send};

use crate::actors::room::PlayerColor::White;
use crate::actors::websocket::WebsocketSession;
use actix::prelude::*;
use actix_redis::Command;
use chess::{ChessMove, Color, Game};
use color_eyre::owo_colors::OwoColorize;
use log::info;
use rand::Rng;
use redis_async::resp_array;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use uuid::Uuid;

/*
   DISCLAIMER: THIS IS A MESS, I WILL FIX IT
*/

pub enum GameState {
    Waiting,
    Started {
        spectators: HashSet<Spectator>,
        players: Players,
        game: Game,
    },
}

pub struct Room {
    room_id: String,
    creator: Player,
    state: GameState,
    redis: Recipient<Command>,
}

impl Room {
    pub fn new(room_id: String, creator: Uuid, redis: Recipient<Command>) -> Self {
        Self {
            room_id,
            creator: Player {
                id: creator,
                session: None,
            },
            redis,
            state: GameState::Waiting,
        }
    }
}

impl Room {
    fn send_message(&self, message: ServerMessage, to: UserType) {
        // TODO: Make more efficient
        match &self.state {
            GameState::Waiting => {
                //TODO
            }
            GameState::Started {
                players,
                spectators,
                ..
            } => match to {
                UserType::Spectator => {
                    for spectator in spectators.iter() {
                        &spectator.session.do_send(Send(message.clone()));
                    }
                }
                UserType::Player(color) => match color {
                    PlayerColor::White => {
                        if let Some(session) = &players.w.session {
                            session.do_send(Send(message))
                        }
                    },
                    PlayerColor::Black => {
                        if let Some(session) = &players.b.session {
                            session.do_send(Send(message))
                        }
                    },
                    PlayerColor::All => {
                        if let Some(session) = &players.w.session {
                            session.do_send(Send(message.clone()))
                        }
                        if let Some(session) = &players.b.session {
                            session.do_send(Send(message))
                        }
                    }
                },
            },
        }
    }
}

impl Actor for Room {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Started room !");
    }
}

impl Handler<Join> for Room {
    type Result = ();

    fn handle(&mut self, msg: Join, ctx: &mut Self::Context) -> Self::Result {
        match &mut self.state {
            GameState::Waiting => {
                if msg.id != self.creator.id {
                    let mut players;
                    // Decide who gets to play which color
                    if rand::thread_rng().gen_bool(0.5) {
                        players = Players {
                            w: Player {
                                id: msg.id,
                                session: Some(msg.session),
                            },
                            b: self.creator.clone(),
                        };
                    } else {
                        players = Players {
                            w: self.creator.clone(),
                            b: Player {
                                id: msg.id,
                                session: Some(msg.session),
                            },
                        };
                    }

                    self.state = GameState::Started {
                        spectators: HashSet::new(),
                        players,
                        game: Game::new(),
                    };

                    // TODO: make shorter
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

                    // TODO: turn into a function
                    self.redis
                        .do_send(Command(resp_array![
                                    "HSET",
                                    format!("rc:room:{}", &self.room_id),
                                    "fen",
                                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
                                ]))
                        .ok();

                } else {
                    self.creator.session = Some(msg.session);
                }
            }
            GameState::Started {
                players,
                game,
                spectators,
            } => {
                let turn = match game.side_to_move() {
                    Color::White => PlayerColor::White,
                    Color::Black => PlayerColor::Black,
                };
                if msg.id == players.w.id {
                    players.w.session = Some(msg.session);
                    let fen = game.current_position().to_string();
                    self.send_message(
                        ServerMessage::Reconnect {
                            color: PlayerColor::White,
                            fen,
                            turn,
                        },
                        UserType::Player(PlayerColor::White),
                    );
                } else if msg.id == players.b.id {
                    players.b.session = Some(msg.session);
                    let fen = game.current_position().to_string();
                    self.send_message(
                        ServerMessage::Reconnect {
                            color: PlayerColor::Black,
                            fen,
                            turn,
                        },
                        UserType::Player(PlayerColor::Black),
                    );
                } else {
                    spectators.insert(Spectator {
                        id: msg.id,
                        session: msg.session,
                    });
                }
            }
        }
    }
}

impl Handler<Leave> for Room {
    type Result = ();

    fn handle(&mut self, msg: Leave, _ctx: &mut Self::Context) -> Self::Result {
        match &mut self.state {
            GameState::Waiting => {
                if msg.id == self.creator.id {
                    self.creator.session = None;
                }
            }
            GameState::Started {
                players,
                spectators,
                ..
            } => {
                if players.w.id == msg.id {
                    players.w.session = None;
                } else if players.b.id == msg.id {
                    players.w.session = None;
                } else {
                    spectators.remove(&msg.id);
                }
            }
        }
    }
}

impl Handler<Move> for Room {
    type Result = ();

    fn handle(&mut self, msg: Move, _ctx: &mut Self::Context) -> Self::Result {
        if let GameState::Started {
            players,
            spectators,
            game,
        } = &mut self.state
        {
            let player_color = match game.side_to_move() {
                chess::Color::Black => PlayerColor::White,
                chess::Color::White => PlayerColor::Black,
            };

            let side = match player_color {
                PlayerColor::White => "white",
                PlayerColor::Black => "black",
                PlayerColor::All => "",
            }
            .to_string();

            match ChessMove::from_str(&format!("{}{}", msg.from, msg.to)) {
                Ok(chess_move) => {
                    if game.make_move(chess_move) {
                        let fen = game.current_position().to_string();

                        self.send_message(
                            ServerMessage::Move {
                                from: msg.from,
                                to: msg.to,
                                side,
                                fen: fen.clone(),
                            },
                            UserType::Player(player_color),
                        );

                        self.redis
                            .do_send(Command(resp_array![
                                    "HSET",
                                    format!("rc:room:{}", &self.room_id),
                                    "fen",
                                    fen
                                ]))
                            .ok();
                    } else {
                        self.send_message(
                            ServerMessage::Err {
                                what: ServerError::IllegalMove,
                            },
                            UserType::Player(player_color),
                        )
                    }
                }
                Err(_e) => self.send_message(
                    ServerMessage::Err {
                        what: ServerError::IllegalMove,
                    },
                    UserType::Player(player_color),
                ),
            }
        }
    }
}
