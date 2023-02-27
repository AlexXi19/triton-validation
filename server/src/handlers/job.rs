use crate::{
    clients::{http_client::AppData, rabbitmq_client::ValidationMessage},
    unwrap_or_return_error,
};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use tracing::info;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct JobBody {
    name: String,
}

#[post("job")]
pub async fn create_job(
    _req: HttpRequest,
    app_data: web::Data<AppData>,
    params: web::Json<JobBody>,
) -> impl Responder {
    info!("Scheduler service called");
    let rabbitmq_client = app_data.validation_queue_client.clone();
    let job_name = params.name.clone();
    let job_id = Uuid::new_v4().to_string();

    // TODO: validate job name

    // TODO: Define job config, get job config, and persist job config for consumption by validation server

    let job = ValidationMessage {
        id: job_id,
        name: job_name,
    };

    unwrap_or_return_error!(
        rabbitmq_client.publish_message(&job).await,
        HttpResponse::InternalServerError(),
        "Failed to publish message"
    );

    HttpResponse::Ok().into()
}
