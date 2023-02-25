use std::sync::Mutex;

use actix_web::{get, post, web, HttpResponse, Responder};
use tracing::info;

pub fn service() -> actix_web::Scope {
    web::scope("/alive")
        .service(dead_or_alive)
        .service(kill_or_res)
}

#[utoipa::path(
    context_path = "/health/alive",
    responses(
        (status = 200, description = "Alive"),
        (status = 500, description = "Dead"),
    )
)]
#[get("")]
pub async fn dead_or_alive(is_alive: web::Data<Mutex<bool>>) -> impl Responder {
    let is_alive = is_alive.lock().unwrap();
    info!("is_alive: {is_alive}");
    if *is_alive {
        HttpResponse::Ok().body("I'm alive")
    } else {
        HttpResponse::InternalServerError().body("I'm dead")
    }
}

#[utoipa::path(
    context_path = "/health/alive",
    responses(
        (status = 200, description = "Updated aliveness"),
    )
)]
#[post("")]
pub async fn kill_or_res(is_alive: web::Data<Mutex<bool>>) -> impl Responder {
    let mut is_alive = is_alive.lock().unwrap();
    info!("mut is_alive: {is_alive}");
    *is_alive = !*is_alive;
    info!("mut is_alive: {is_alive}");
    HttpResponse::Ok()
}
