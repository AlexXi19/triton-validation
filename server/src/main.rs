mod clients;
mod env;
mod k8s;
mod service;
use anyhow::Result;

lazy_static::lazy_static! {
    static ref ENV_VARS: env::Variables = env::Variables::get_env_vars().expect("Failed to get env vars");
}

#[tokio::main]
async fn main() -> Result<()> {
    let vars = env::Variables::get_env_vars().expect("Failed to get env vars");
    clients::http_client::start(vars.http_port)
        .await
        .expect("Failed to start http client");

    Ok(())
}
