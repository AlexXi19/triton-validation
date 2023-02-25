use crate::{clients::http_client::AppData, unwrap_or_return_error};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use tracing::info;

#[post("scheduler")]
pub async fn create(_req: HttpRequest, app_data: web::Data<AppData>) -> impl Responder {
    info!("Scheduler service called");
    let rabbitmq_client = app_data.validation_queue_client.clone();

    let message = "Hello world!".to_string();

    unwrap_or_return_error!(
        rabbitmq_client.publish_message(&message).await,
        HttpResponse::InternalServerError(),
        "Failed to publish message"
    );

    HttpResponse::Ok().into()
}
