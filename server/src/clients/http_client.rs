use crate::service;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;

pub async fn start(port: String) -> Result<()> {
    let port = port.parse::<u16>()?;

    HttpServer::new(|| {
        App::new()
            .service(welcome)
            .service(health)
            .service(service::scheduler::create)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await?;

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
