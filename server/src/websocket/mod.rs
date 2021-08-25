pub mod handlers;
pub mod model;

pub use handlers::config;

use crate::room::{self, Room};
use crate::room_manager;
use crate::room_manager_ng;
use model::{ClientMessage, ServerError, ServerMessage};

use actix::prelude::*;
use actix_redis::RedisActor;
use actix_web_actors::ws;
use log::{error, info, trace};
use std::time::{Duration, Instant};
use uuid::Uuid;
use color_eyre::owo_colors::OwoColorize;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub enum Connection {
    Play(String),
    Lobby,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Send(pub ServerMessage);

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinedRoom(pub Addr<Room>);

pub struct WebsocketSession {
    pub id: Uuid,
    pub hb: Instant,
    pub room_manager: Addr<room_manager_ng::RoomManager>,
    pub room: Option<Addr<Room>>,
    pub connection: Connection,
}

impl WebsocketSession {
    pub fn new(
        id: Uuid,
        connection: Connection,
        room_manager: Addr<room_manager_ng::RoomManager>,
    ) -> Self {
        Self {
            id,
            connection,
            room_manager,
            room: None,
            hb: Instant::now(),
        }
    }

    fn create_err(err: ServerError) -> String {
        serde_json::to_string(&ServerMessage::Err { what: err }).unwrap()
    }
}

impl Actor for WebsocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        self.room_manager
            .send(room_manager_ng::Connect {
                id: self.id.clone(),
                session: ctx.address().recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(()) => (),
                    // Something has gone wrong, closing the socket
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);

        match &self.connection {
            Connection::Play(room_id) => self.room_manager.do_send(room_manager_ng::Join {
                id: self.id.clone(),
                room_id: room_id.clone(),
                session: ctx.address(),
            }),
            Connection::Lobby => self.room_manager.do_send(room_manager_ng::List {
                id: self.id.clone(),
                items: 12,
                session: ctx.address().recipient(),
            }),
        }
    }

    /*
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(room_manager::Disconnect { id: self.id });
        Running::Stop
    }
     */
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebsocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        trace!("Message: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => match serde_json::from_str::<ClientMessage>(&text) {
                Ok(msg) => match &self.connection {
                    Connection::Play(string) => match msg {
                        ClientMessage::Move { from, to, fen } => {
                            info!("Got move message");
                            if let Some(room) = &self.room {
                                info!("&self.room is not empty!");
                                room.do_send(room::Move {
                                    id: self.id,
                                    from,
                                    to,
                                })
                            }
                        }
                        _ => ctx.text(WebsocketSession::create_err(ServerError::OutOfContext)),
                    },
                    Connection::Lobby => match msg {
                        ClientMessage::Create => {
                            self.room_manager.do_send(room_manager_ng::Create {
                                id: self.id,
                                session: ctx.address(),
                            })
                        }
                        ClientMessage::List(items) => {}
                        _ => ctx.text(WebsocketSession::create_err(ServerError::OutOfContext)),
                    },
                },
                Err(_e) => ctx.text(WebsocketSession::create_err(ServerError::InvalidInput)),
            },
            ws::Message::Binary(_) => error!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

impl WebsocketSession {
    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // notify chat room_manager
                /*
                act.addr.do_send(room_manager::Disconnect { id: act.id });
                 */

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Handler<Send> for WebsocketSession {
    type Result = ();

    fn handle(&mut self, msg: Send, ctx: &mut Self::Context) -> Self::Result {
        match serde_json::to_string(&msg.0) {
            Ok(msg) => ctx.text(msg),
            Err(_e) => ctx.text(WebsocketSession::create_err(ServerError::InternalError)),
        }
    }
}

impl Handler<JoinedRoom> for WebsocketSession {
    type Result = ();

    fn handle(&mut self, msg: JoinedRoom, ctx: &mut Self::Context) -> Self::Result {
        info!("GOT <JoinedRoom>");
        self.room = Some(msg.0);
    }
}
