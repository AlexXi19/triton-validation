use crate::service;
use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use tracing::info;

pub async fn start(port: String) -> Result<()> {
    let port = port.parse::<u16>()?;

    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(welcome)
            .service(health)
            .service(service::scheduler::create)
    })
    .bind(("127.0.0.1", port))?
    .run();

    info!("HTTP server started on port {}", port);

    server.await?;

    Ok(())
}

#[get("/")]
async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the validation server!")
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}
