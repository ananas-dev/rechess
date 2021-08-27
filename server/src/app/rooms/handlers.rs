use super::model::{self, Response};
use crate::util::redis::get_hashmap;
use actix::prelude::*;
use actix_redis::{Command, Error, RedisActor, RespValue};
use actix_web::{delete, get, post, web, web::ServiceConfig, HttpResponse, Responder};
use log::info;
use redis_async::resp_array;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::str;

pub fn config(config: &mut ServiceConfig) {
    config.service(get_room);
}

#[get("/{id}")]
pub async fn get_room(id: web::Path<String>, redis: web::Data<Addr<RedisActor>>) -> impl Responder {
    match redis
        .send(Command(resp_array!["HGETALL", format!("rc:room:{}", id)]))
        .await
    {
        Ok(resp) => match resp {
            Ok(data) => match get_hashmap(data) {
                Some(hashmap) => HttpResponse::Ok().json(&hashmap),
                None => HttpResponse::BadRequest().json(Response::NotStarted),
            },
            Err(_) => HttpResponse::BadRequest().json(Response::Error(model::Error::RedisError)),
        },
        Err(_) => HttpResponse::BadRequest().json(Response::Error(model::Error::InternalError)),
    }
}
