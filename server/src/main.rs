mod clients;
mod env;
mod handlers;
mod k8s;
mod server_factory;
mod utils;
mod worker;

use crate::clients::kube_client::KubeClient;
use anyhow::Result;

lazy_static::lazy_static! {
    static ref env_vars: env::Variables = env::Variables::get_env_vars().expect("Failed to get env vars");
}

#[tokio::main]
async fn main() -> Result<()> {
    let vars = env::Variables::get_env_vars().expect("Failed to get env vars");
    server_factory::init(vars)
        .await
        .expect("Failed to start client factory");

    Ok(())
}
