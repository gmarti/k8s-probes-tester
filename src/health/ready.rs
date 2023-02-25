use std::sync::Mutex;

use actix_web::{get, post, web, HttpResponse, Responder};

pub fn service() -> actix_web::Scope {
    web::scope("/ready")
        .service(ready_or_not)
        .service(update_readyness)
}

#[utoipa::path(
    context_path = "/health/ready",
    responses(
        (status = 200, description = "Ready"),
        (status = 500, description = "Busy"),
    )
)]
#[get("")]
pub async fn ready_or_not(is_ready: web::Data<Mutex<bool>>) -> impl Responder {
    if *(is_ready.lock().unwrap()) {
        HttpResponse::Ok().body("I'm ready")
    } else {
        HttpResponse::InternalServerError().body("I'm not ready")
    }
}

#[utoipa::path(
    context_path = "/health/ready",
    responses(
        (status = 200, description = "Updated readyness"),
    )
)]
#[post("")]
pub async fn update_readyness(is_ready: web::Data<Mutex<bool>>) -> impl Responder {
    let mut is_ready = is_ready.lock().unwrap();
    *is_ready = !*is_ready;
    HttpResponse::Ok()
}
