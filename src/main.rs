use std::sync::Mutex;

use actix_web::{middleware, web, App, HttpServer};
use anyhow::Result;

use clap::Parser;
use env_logger::Env;
use tracing::info;
use utoipa::OpenApi;

use crate::health::{alive::Alive, ready::Ready};

mod health;

#[derive(Debug, Parser)]
struct Args {
    /// IP Address to listen on
    #[arg(long, short)]
    address: String,
    /// Port
    #[arg(long, short)]
    port: u16,
}

#[derive(OpenApi)]
#[openapi(
        paths(
            health::alive::dead_or_alive,
            health::alive::kill_or_res,
        ),
        components(
            schemas()
        ),
        tags(
            (name = "k8s-probes-tester", description = "k8s probes tester endpoints")
        )
    )]
struct ApiDoc;

#[actix_web::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_module_path(false)
        .init();

    #[cfg(debug_assertions)]
    {
        use tracing::warn;
        warn!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        warn!("============  RUNNING DEBUG BUILD ================");
        warn!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    }

    info!(
        "starting HTTP server at http://{}:{}",
        args.address, args.port
    );

    let is_ready = web::Data::new(Mutex::new(Alive::new(true)));
    let is_alive = web::Data::new(Mutex::new(Ready::new(true)));

    HttpServer::new(move || {
        App::new() // enable logger
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .app_data(is_ready.clone())
            .app_data(is_alive.clone())
            .service(health::service())
            .route(
                "/api-doc/openapi.json",
                web::get().to(|| async { web::Json(ApiDoc::openapi()) }),
            )
    })
    .bind((args.address, args.port))?
    .run()
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use actix_web::body::to_bytes;
    use actix_web::dev::Service;
    use actix_web::{http, test, App, Error};

    use super::*;

    #[actix_web::test]
    async fn ready_or_not() -> Result<(), Error> {
        let is_ready = web::Data::new(Mutex::new(Alive::new(true)));
        let is_alive = web::Data::new(Mutex::new(Ready::new(true)));
        let app = App::new()
            .app_data(is_ready.clone())
            .app_data(is_alive.clone())
            .service(health::service());
        let app = test::init_service(app).await;

        let get_request = test::TestRequest::get().uri("/health/ready").to_request();

        // Initialy we are ready
        let resp = app.call(get_request).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await.unwrap(), r##"I'm ready"##);

        // Not ready!
        let post_request = test::TestRequest::post().uri("/health/ready").to_request();
        let resp = app.call(post_request).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);

        let get_request = test::TestRequest::get().uri("/health/ready").to_request();
        let resp = app.call(get_request).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await.unwrap(), r##"I'm not ready"##);

        // Ready again !
        let post_request = test::TestRequest::post().uri("/health/ready").to_request();
        let resp = app.call(post_request).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);

        let get_request = test::TestRequest::get().uri("/health/ready").to_request();
        let resp = app.call(get_request).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);
        Ok(())
    }

    #[actix_web::test]
    async fn dead_or_alive() -> Result<(), Error> {
        let is_ready = web::Data::new(Mutex::new(Alive::new(true)));
        let is_alive = web::Data::new(Mutex::new(Ready::new(true)));
        let app = App::new()
            .app_data(is_ready.clone())
            .app_data(is_alive.clone())
            .service(health::service());
        let app = test::init_service(app).await;

        let get_request = test::TestRequest::get().uri("/health/alive").to_request();

        // Initialy we are Alive
        let resp = app.call(get_request).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await.unwrap(), r##"I'm alive"##);

        // Kill it!
        let post_request = test::TestRequest::post().uri("/health/alive").to_request();
        let resp = app.call(post_request).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);

        let get_request = test::TestRequest::get().uri("/health/alive").to_request();
        let resp = app.call(get_request).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await.unwrap(), r##"I'm dead"##);

        // Resurrect !
        let post_request = test::TestRequest::post().uri("/health/alive").to_request();
        let resp = app.call(post_request).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);

        let get_request = test::TestRequest::get().uri("/health/alive").to_request();
        let resp = app.call(get_request).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);
        Ok(())
    }
}
