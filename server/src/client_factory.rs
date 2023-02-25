use crate::{clients, env::Variables, KubeClient};
use anyhow::Result;
use once_cell::sync::OnceCell;
use tracing::info;

static KUBE_CLIENT: OnceCell<KubeClient> = OnceCell::new();

pub async fn init(env: Variables) -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Initializing clients");

    let (_, kube_client_init) = tokio::join!(
        clients::http_client::start(env.http_port),
        KubeClient::new(env.kubeconfig)
    );

    KUBE_CLIENT.set(kube_client_init?).map_err(|_| anyhow::anyhow!("Failed to set kube client"))?;

    Ok(())
}
