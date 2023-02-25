use crate::k8s::scheduler::create_validation_environment;
use actix_web::{get, post, HttpResponse, Responder};
use tracing::info;

#[post("scheduler")]
pub async fn create() -> impl Responder {
    info!("Scheduler service called");
    let name = "test".to_string();
    create_validation_environment(name).await;
    HttpResponse::Ok()
}
