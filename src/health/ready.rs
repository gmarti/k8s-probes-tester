use std::sync::Mutex;

use actix_web::{get, post, web, HttpResponse, Responder};

pub struct Ready(bool);

impl Ready {
    pub fn new(value: bool) -> Ready {
        Ready(value)
    }
}

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
pub async fn ready_or_not(is_ready: web::Data<Mutex<Ready>>) -> impl Responder {
    if is_ready.lock().unwrap().0 {
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
pub async fn update_readyness(is_ready: web::Data<Mutex<Ready>>) -> impl Responder {
    let mut is_ready = is_ready.lock().unwrap();
    is_ready.0 = !is_ready.0;
    HttpResponse::Ok()
}
