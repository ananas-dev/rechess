use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use redis::{Client, aio::MultiplexedConnection};
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

#[derive(Message, Debug)]
#[rtype(result = "Result<Option<String>, redis::RedisError>")]
pub struct InfoCommand;

impl Handler<InfoCommand> for RedisActor {
    type Result = ResponseFuture<Result<Option<String>, redis::RedisError>>;

    fn handle(&mut self, _msg: InfoCommand, _: &mut Self::Context) -> Self::Result {
        let mut con = self.conn.clone();
        let cmd = redis::cmd("INFO");
        let fut = async move {
            cmd
                .query_async(&mut con)
                .await
        };
        Box::pin(fut)
    }
}

impl Actor for RedisActor {
    type Context = Context<Self>;
}

async fn info(redis: web::Data<Addr<RedisActor>>) -> impl Responder {
    let res = redis.send(InfoCommand).await.unwrap().unwrap().unwrap();
    HttpResponse::Ok().body(res)
}