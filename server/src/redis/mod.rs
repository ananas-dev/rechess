use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use redis::{Client, Cmd, RedisResult, aio::MultiplexedConnection, cmd};
use actix::prelude::*;

pub struct RedisActor {
    conn: MultiplexedConnection,
}

impl RedisActor {
    pub async fn new(redis_url: &str) -> Self {
        let client = Client::open(redis_url).unwrap();// not recommended
        let conn = client.get_multiplexed_async_connection().await.unwrap();
        RedisActor { conn }
    }
}

#[derive(Message)]
#[rtype(result = "RedisResult<Option<String>>")]
pub struct PingCommand;

#[derive(Message)]
#[rtype(result = "RedisResult<Option<String>>")]
pub struct SetCommand;

#[derive(Message)]
#[rtype(result = "RedisResult<Option<String>>")]
pub struct Command(Cmd);

impl Handler<PingCommand> for RedisActor {
    type Result = ResponseFuture<RedisResult<Option<String>>>;

    fn handle(&mut self, _msg: PingCommand, _: &mut Self::Context) -> Self::Result {
        let mut con = self.conn.clone();
        let cmd = redis::cmd("PING");
        let fut = async move {
            cmd
                .query_async(&mut con)
                .await
        };
        Box::pin(fut)
    }
}

impl Handler<Command> for RedisActor {
    type Result = ResponseFuture<RedisResult<Option<String>>>;

    fn handle(&mut self, msg: Command, ctx: &mut Self::Context) -> Self::Result {
        let mut con = self.conn.clone();
        let fut = async move {
            msg.0.query_async(&mut con).await
        };
        Box::pin(fut)
    }
}

impl Actor for RedisActor {
    type Context = Context<Self>;
}
