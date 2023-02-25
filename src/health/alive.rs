use std::sync::Mutex;

use actix_web::{get, post, web, HttpResponse, Responder};

pub struct Alive(bool);

impl Alive {
    pub fn new(value: bool) -> Alive {
        Alive(value)
    }
}

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
pub async fn dead_or_alive(is_alive: web::Data<Mutex<Alive>>) -> impl Responder {
    let is_alive = is_alive.lock().unwrap();
    if is_alive.0 {
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
pub async fn kill_or_res(is_alive: web::Data<Mutex<Alive>>) -> impl Responder {
    let mut is_alive = is_alive.lock().unwrap();

    is_alive.0 = !is_alive.0;

    HttpResponse::Ok()
}
