use std::sync::Mutex;

use actix_web::{get, post, web, HttpResponse, Responder};

pub fn service() -> actix_web::Scope {
    let is_alive = web::Data::new(Mutex::new(true));

    web::scope("/alive")
        .app_data(is_alive.clone())
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
    if *is_alive.lock().unwrap() {
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
    *is_alive = !*is_alive;
    HttpResponse::Ok()
}

#[cfg(test)]
mod tests {
    use actix_web::body::to_bytes;
    use actix_web::dev::Service;
    use actix_web::{http, test, App, Error};

    use super::*;

    #[actix_web::test]
    async fn dead_or_alive() -> Result<(), Error> {
        let app = App::new().service(service());
        let app = test::init_service(app).await;

        let get_request = test::TestRequest::get().uri("/alive").to_request();

        // Initialy we are Alive
        let resp = app.call(get_request).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await.unwrap(), r##"I'm alive"##);

        // Kill it!
        let post_request = test::TestRequest::post().uri("/alive").to_request();
        let resp = app.call(post_request).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);

        let get_request = test::TestRequest::get().uri("/alive").to_request();
        let resp = app.call(get_request).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await.unwrap(), r##"I'm dead"##);

        // Resurrect !
        let post_request = test::TestRequest::post().uri("/alive").to_request();
        let resp = app.call(post_request).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);

        let get_request = test::TestRequest::get().uri("/alive").to_request();
        let resp = app.call(get_request).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);
        Ok(())
    }
}
