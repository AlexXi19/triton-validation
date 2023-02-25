use std::sync::Arc;

use crate::{
    clients::{self, http_client::AppData},
    env::Variables,
    worker, KubeClient,
};
use anyhow::Result;
use once_cell::sync::OnceCell;
use tracing::info;

pub async fn init(env: Variables) -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Initializing clients...");

    let kube_client = Arc::new(KubeClient::new(env.kubeconfig).await?);
    let validation_queue_client = Arc::new(
        clients::rabbitmq_client::RabbitMQClient::new(&env.amqp_url, &env.queue_name).await?,
    );
    let worker_handles = worker::coordinator::start(env.num_workers, kube_client.clone(), validation_queue_client.clone());

    clients::http_client::start(
        env.http_port,
        AppData {
            kube_client: kube_client.clone(),
            validation_queue_client: validation_queue_client.clone(),
        },
    )
    .await?;

    for handle in worker_handles {
        handle.await??;
    }

    Ok(())
}
