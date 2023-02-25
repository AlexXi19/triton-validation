use std::sync::Arc;

use crate::{
    clients::{self, http_client::AppData},
    env::Variables,
    KubeClient,
};
use anyhow::Result;
use once_cell::sync::OnceCell;
use tracing::info;

pub async fn init(env: Variables) -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Initializing clients");
    let kube_client = KubeClient::new(env.kubeconfig).await?;

    let app_data = AppData {
        kube_client: Arc::new(kube_client),
    };

    clients::http_client::start(env.http_port, app_data).await?;

    Ok(())
}
