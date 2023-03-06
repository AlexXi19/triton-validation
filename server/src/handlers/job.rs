use std::str::FromStr;

use crate::{
    clients::{http_client::AppData, rabbitmq_client::ValidationMessage},
    k8s::utils::delete_pod_name_from_job_id,
    unwrap_or_return_error,
};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use tracing::*;
use uuid::Uuid;
use validaton_server::RequestBody::{CreateJobBody, JobStatus, JobStatusBody};

#[post("job")]
pub async fn create_job(
    _req: HttpRequest,
    app_data: web::Data<AppData>,
    params: web::Json<CreateJobBody>,
) -> impl Responder {
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

#[post("job/status")]
pub async fn update_job_status(
    _req: HttpRequest,
    app_data: web::Data<AppData>,
    params: web::Json<JobStatusBody>,
) -> impl Responder {
    let kube_client = app_data.kube_client.clone();
    let job_status = unwrap_or_return_error!(
        JobStatus::from_str(params.status.as_str()),
        HttpResponse::BadRequest(),
        "Invalid job status"
    );
    let job_id = params.id.clone();

    match job_status {
        JobStatus::Completed => {
            info!("Job {} completed", job_id);
            unwrap_or_return_error!(
                delete_pod_name_from_job_id(kube_client, job_id.as_str()).await,
                HttpResponse::InternalServerError(),
                "Failed to delete pod to complete job"
            );
        }
        JobStatus::Failed => {
            info!("Job failed");
        }
    }

    HttpResponse::Ok().into()
}
