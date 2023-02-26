use crate::{
    clients::{http_client::AppData, rabbitmq_client::ValidationMessage},
    unwrap_or_return_error,
};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use tracing::info;
use uuid::Uuid;

#[post("job")]
pub async fn create_job(_req: HttpRequest, app_data: web::Data<AppData>) -> impl Responder {
    info!("Scheduler service called");
    let rabbitmq_client = app_data.validation_queue_client.clone();

    let job = ValidationMessage {
        id: Uuid::new_v4().to_string(),
        name: "test-job".to_string(),
    };

    unwrap_or_return_error!(
        rabbitmq_client.publish_message(&job).await,
        HttpResponse::InternalServerError(),
        "Failed to publish message"
    );

    HttpResponse::Ok().into()
}
