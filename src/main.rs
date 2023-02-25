use actix_web::{middleware, web, App, HttpServer};
use anyhow::Result;

use clap::Parser;
use env_logger::Env;
use tracing::info;
use utoipa::OpenApi;

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

    HttpServer::new(move || {
        App::new() // enable logger
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
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

