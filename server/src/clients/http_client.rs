use std::sync::Arc;

use crate::service;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use tracing::info;

use super::kube_client::KubeClient;

pub struct AppData {
    pub kube_client: Arc<KubeClient>,
}

pub async fn start(port: String, state: AppData) -> Result<()> {
    let port = port.parse::<u16>()?;
    let state = web::Data::new(state);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(state.clone())
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
