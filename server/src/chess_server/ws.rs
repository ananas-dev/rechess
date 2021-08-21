use super::server;
use super::model::{self, ServerMessage, ServerError, ClientMessage};

use actix::prelude::*;
use actix_web_actors::ws;
use log::{error, info, trace};
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Send(pub ServerMessage);

pub struct WsChessSession {
    pub id: usize,
    pub hb: Instant,
    pub game_id: String,
    pub name: Option<String>,
    pub addr: Addr<server::ChessServer>,
}

impl Actor for WsChessSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr
            .send(server::Connect {
                addr: addr.recipient(),
                room_name: self.game_id.clone(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChessSession {
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
            },
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            },
            ws::Message::Text(text) => {
                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(msg) => {
                        match msg {
                            ClientMessage::Move { san, fen } => {
                                self.addr.do_send(server::Move {
                                    id: self.id,
                                    game_id: self.game_id.clone(),
                                    san: san,
                                })
                            }
                        }
                    },
                    Err(_e) => {
                        ctx.text(serde_json::to_string(
                            &ServerMessage::Err {
                                what: ServerError::InvalidInput,
                            }
                        ).unwrap())
                    }
                }
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

impl WsChessSession {
    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // notify chat server
                act.addr.do_send(server::Disconnect { id: act.id });

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Handler<Send> for WsChessSession {
    type Result = ();

    fn handle(&mut self, msg: Send, ctx: &mut Self::Context) -> Self::Result {
        match serde_json::to_string(&msg.0) {
            Ok(msg) => ctx.text(msg),
            Err(_e) => {
                ctx.text(serde_json::to_string(&ServerMessage::Err {
                    what: ServerError::InternalError,
                }).unwrap())
            }
        }
    }
}
