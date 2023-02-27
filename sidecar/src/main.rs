use tracing::info;

use crate::clients::triton_client;

pub mod clients;
pub mod env;
pub mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // TODO: Fetch load test configs from db
    let env = env::Variables::get_env_vars().unwrap();
    let job_id = env.job_id;
    let triton_client = triton_client::TritonClient::new(&env.triton_url).unwrap();

    // TODO: Wait for triton server to start up
    info!("Waiting for Triton server...");

    triton_client.wait_for_server_ready().await.unwrap();

    // TODO: Start tests

    // TODO: Publish results
}
