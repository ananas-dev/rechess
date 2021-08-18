use super::model::{NewUser, User};

use actix_web::{delete, get, post, web, web::ServiceConfig, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

// TODO: Proper error handling

pub fn config(config: &mut ServiceConfig) {
    config
        .service(get_user)
        .service(get_users)
        .service(create_user)
        .service(delete_user);
}

#[get("/")]
pub async fn get_users(db_pool: web::Data<PgPool>) -> impl Responder {
    match User::find_all(&db_pool).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::BadRequest().body(format!("got error {:?}", e)),
    }
}

#[post("/")]
pub async fn create_user(data: web::Json<NewUser>, db_pool: web::Data<PgPool>) -> impl Responder {
    match data.validate() {
        Ok(_) => (),
        Err(e) => return HttpResponse::BadRequest().body(format!("{}", e)),
    };

    match User::create(&db_pool, data.into_inner()).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::BadRequest().body(format!("{}", e)),
    }
}

#[get("/{id}")]
pub async fn get_user(id: web::Path<Uuid>, db_pool: web::Data<PgPool>) -> impl Responder {
    match User::find_by_id(&db_pool, *id).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::BadRequest().body(format!("got error {:?}", e)),
    }
}

#[delete("/{id}")]
pub async fn delete_user(id: web::Path<Uuid>, db_pool: web::Data<PgPool>) -> impl Responder {
    match User::delete(&db_pool, *id).await {
        Ok(_data) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::BadRequest().body(format!("{}", e)),
    }
}
