use serde_json::json;
use tracing::info;

use crate::clients::triton_client;

pub mod clients;
pub mod env;
pub mod goose;
pub mod utils;
use validaton_server::RequestBody;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // TODO: Fetch load test configs from db
    let env = env::Variables::get_env_vars().unwrap();
    let client = reqwest::Client::new();
    let job_id = env.job_id;
    let triton_client = triton_client::TritonClient::new(&env.triton_url).unwrap();

    // TODO: Wait for triton server to start up
    info!("Waiting for Triton server...");
    triton_client.wait_for_server_ready().await.unwrap();

    // Download models

    // Load models

    // TODO: Start tests

    // TODO: Publish results

    // Notify the server that the job is complete
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let job_status = RequestBody::JobStatus::Completed;
    info!("Notifying server that job is complete...");
    let body = RequestBody::JobStatusBody {
        name: None,
        id: job_id.clone(),
        status: job_status.to_string(),
    };

    client
        .post(format!("{}/job/status", env.server_url).as_str())
        .json::<RequestBody::JobStatusBody>(&body)
        .send()
        .await
        .unwrap();
}
