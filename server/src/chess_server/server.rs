use actix::prelude::*;
use uuid::Uuid;
use chess::{ChessMove, Game};
use log::info;
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::{HashMap, HashSet};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

pub struct Room {
    white: Uuid,
    black: Uuid,
    spectators: HashSet<Uuid>,
    game: Game,
}

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message for chat server communications

/// New chat session is created
#[derive(Message)]
#[rtype(Uuid)]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub room_name: String,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}

/// Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: Uuid,
    pub msg: String,
    pub room: String,
}

pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub id: Uuid,
    /// Room name
    pub name: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Move {
    pub id: Uuid,
    pub game_id: String,
    pub san: String,
}

/// `ChatServer` manages chat rooms and responsible for coordinating chat
/// session. implementation is super primitive
pub struct ChessServer {
    sessions: HashMap<usize, Recipient<Message>>,
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
    /// Send message to all users in the room
    fn send_message(&self, room: &str, message: &str, skip_id: usize) {
        if let Some(room) = self.rooms.get(room) {
            for id in room.users.clone() {
                if id != skip_id {
                    if let Some(addr) = self.sessions.get(&id) {
                        let _ = addr.do_send(Message(message.to_owned()));
                    }
                }
            }
        }
    }
}

/// Make actor from `ChatServer`
impl Actor for ChessServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChessServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        info!("Someone joined");

        // notify all users in same room
        self.send_message(&msg.room_name, "Someone joined", 0);

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        // auto join session to Main room
        self.rooms
            .entry(msg.room_name.clone())
            .or_insert_with(|| Room {
                users: HashSet::new(),
                game: Game::new(),
            })
            .users
            .insert(id);

        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        self.send_message(&msg.room_name, &format!("Total visitors {}", count), 0);

        // send id back
        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for ChessServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        info!("Someone disconnected");

        let mut rooms: Vec<String> = Vec::new();

        // remove address
        if self.sessions.remove(&msg.id).is_some() {
            // remove session from all rooms
            for (name, room) in &mut self.rooms {
                if room.users.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }
        // send message to other users
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }
    }
}

/// Handler for Message message.
impl Handler<ClientMessage> for ChessServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(&msg.room, msg.msg.as_str(), msg.id);
    }
}

/// Handler for `ListRooms` message.
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

/// Join room, send disconnect message to old room
/// send join message to new room
impl Handler<Join> for ChessServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        let Join { id, name } = msg;
        let mut rooms = Vec::new();

        // remove session from all rooms
        for (n, sessions) in &mut self.rooms {
            if sessions.users.remove(&id) {
                rooms.push(n.to_owned());
            }
        }
        // send message to other users
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }

        self.rooms
            .entry(name.clone())
            .or_insert_with(|| Room {
                users: HashSet::new(),
                game: Game::new(),
            })
            .users
            .insert(id);

        self.send_message(&name, "Someone connected", id);
    }
}
/// Join room, send disconnect message to old room
/// send join message to new room
impl Handler<Move> for ChessServer {
    type Result = ();

    fn handle(&mut self, msg: Move, _: &mut Context<Self>) {
        let Move { id, room_name, san } = msg;

        if let Some(room) = self.rooms.get_mut(&room_name) {
            match ChessMove::from_san(&room.game.current_position(), &san) {
                Ok(chess_move) => {
                    room.game.make_move(chess_move);
                    self.send_message(&room_name, &format!("Move made by {}: {}", id, san), 0);
                }
                Err(e) => {
                    self.send_message(&room_name, &format!("Illegal move: {:#?}", e), 0);
                }
            }
        }
    }
}
