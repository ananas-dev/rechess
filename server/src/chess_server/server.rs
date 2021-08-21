use super::model::{self, ServerMessage, ServerError};
use super::ws;

use actix::prelude::*;
use chess::{ChessMove, Game};
use log::info;
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::{HashMap, HashSet};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

pub struct Room {
    users: HashSet<usize>,
    game: Game,
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<ws::Send>,
    pub room_name: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub id: usize,
    /// Room name
    pub name: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Move {
    pub id: usize,
    pub game_id: String,
    pub san: String,
}

pub struct ChessServer {
    sessions: HashMap<usize, Recipient<ws::Send>>,
    rooms: HashMap<String, Room>,
    rng: ThreadRng,
    visitor_count: Arc<AtomicUsize>,
}

impl ChessServer {
    pub fn new(visitor_count: Arc<AtomicUsize>) -> ChessServer {
        ChessServer {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            rng: rand::thread_rng(),
            visitor_count,
        }
    }
}

impl ChessServer {
    fn send_message(&self, room: &str, message: ServerMessage, skip_id: usize) {
        if let Some(room) = self.rooms.get(room) {
            for id in room.users.clone() {
                if id != skip_id {
                    if let Some(addr) = self.sessions.get(&id) {
                        let _ = addr.do_send(ws::Send(message.clone()));
                    }
                }
            }
        }
    }
}

impl Actor for ChessServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChessServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        info!("Someone joined");
        
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        let _room =  self.rooms
            .entry(msg.room_name.clone())
            .or_insert_with(|| Room {
                users: HashSet::new(),
                game: Game::new(),
            })
            .users
            .insert(id);
        
        //let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);

        id
    }
}

impl Handler<Disconnect> for ChessServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        info!("Someone disconnected");

        let mut rooms: Vec<String> = Vec::new();

        if self.sessions.remove(&msg.id).is_some() {
            for (name, room) in &mut self.rooms {
                if room.users.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }
        /*
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }
        */
    }
}

impl Handler<ListRooms> for ChessServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _: &mut Context<Self>) -> Self::Result {
        let mut rooms = Vec::new();

        for key in self.rooms.keys() {
            rooms.push(key.to_owned())
        }

        MessageResult(rooms)
    }
}

impl Handler<Join> for ChessServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        let Join { id, name } = msg;
        let mut rooms = Vec::new();

        for (n, sessions) in &mut self.rooms {
            if sessions.users.remove(&id) {
                rooms.push(n.to_owned());
            }
        }

        /*
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }
        */

        self.rooms
            .entry(name.clone())
            .or_insert_with(|| Room {
                users: HashSet::new(),
                game: Game::new(),
            })
            .users
            .insert(id);

        //self.send_message(&name, "Someone connected", id);
    }
}

impl Handler<Move> for ChessServer {
    type Result = ();

    fn handle(&mut self, msg: Move, _: &mut Context<Self>) {
        let Move { id, game_id, san } = msg;

        if let Some(room) = self.rooms.get_mut(&game_id) {
            let side = match room.game.side_to_move() {
                chess::Color::Black => "b",
                chess::Color::White => "w",
            }.to_string();

            match ChessMove::from_san(&room.game.current_position(), &san) {
                Ok(chess_move) => {
                    room.game.make_move(chess_move);
                    let fen = room.game.current_position().to_string();
                    self.send_message(&game_id, ServerMessage::Move { san: san.to_owned(), side: side.to_owned(), fen }, 0);
                }
                Err(_e) => {
                    self.send_message(&game_id, ServerMessage::Err { what: ServerError::IllegalMove }, 0);
                }
            }
        }
    }
}
