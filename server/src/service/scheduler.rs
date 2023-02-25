use crate::clients::http_client::AppData;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use tracing::info;

#[post("scheduler")]
pub async fn create(_req: HttpRequest, app_data: web::Data<AppData>) -> impl Responder {
    info!("Scheduler service called");
    app_data.kube_client.health().await.unwrap();

    HttpResponse::Ok()
}
