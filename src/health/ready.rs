use std::sync::Mutex;

use actix_web::{get, post, web, HttpResponse, Responder};

pub fn service() -> actix_web::Scope {
    let is_ready = web::Data::new(Mutex::new(true));

    web::scope("/ready")
        .app_data(is_ready.clone())
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
    if *is_ready.lock().unwrap() {
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

#[cfg(test)]
mod tests {
    use actix_web::body::to_bytes;
    use actix_web::dev::Service;
    use actix_web::{http, test, App, Error};

    use super::*;

    #[actix_web::test]
    async fn ready_or_not() -> Result<(), Error> {
        let app = App::new().service(service());
        let app = test::init_service(app).await;

        let get_request = test::TestRequest::get().uri("/ready").to_request();

        // Initialy we are ready
        let resp = app.call(get_request).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await.unwrap(), r##"I'm ready"##);

        // Kill it!
        let post_request = test::TestRequest::post().uri("/ready").to_request();
        let resp = app.call(post_request).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);

        let get_request = test::TestRequest::get().uri("/ready").to_request();
        let resp = app.call(get_request).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await.unwrap(), r##"I'm not ready"##);

        // Resurrect !
        let post_request = test::TestRequest::post().uri("/ready").to_request();
        let resp = app.call(post_request).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);

        let get_request = test::TestRequest::get().uri("/ready").to_request();
        let resp = app.call(get_request).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);
        Ok(())
    }
}
