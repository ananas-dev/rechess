use super::model::Auth;

use actix_web::{post, web, web::ServiceConfig, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

// TODO: Proper error handling

pub fn config(config: &mut ServiceConfig) {
    config.service(login);
}

#[post("/")]
pub async fn login(data: web::Json<Auth>, db_pool: web::Data<PgPool>) -> impl Responder {
    match data.validate() {
        Ok(_) => (),
        Err(e) => return HttpResponse::BadRequest().body(format!("{}", e)),
    };

    HttpResponse::Ok().body("aaabbbccc")
}
