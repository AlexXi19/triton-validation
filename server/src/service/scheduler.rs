use crate::{clients::http_client::AppData, unwrap_or_return_error};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use tracing::info;

#[post("scheduler")]
pub async fn create(_req: HttpRequest, app_data: web::Data<AppData>) -> impl Responder {
    info!("Scheduler service called");
    let health = unwrap_or_return_error!(
        app_data.kube_client.health().await,
        HttpResponse::InternalServerError(),
        "Failed to get health"
    );

    HttpResponse::Ok().into()
}
