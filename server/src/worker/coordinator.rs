use std::sync::Arc;

use crate::clients::{kube_client::KubeClient, rabbitmq_client::RabbitMQClient};
use anyhow::Result;
use tokio::task::JoinHandle;

use super::worker::Worker;

pub fn start(
    num_workers: u16,
    kube_client: Arc<KubeClient>,
    rabbitmq_client: Arc<RabbitMQClient>,
) -> Vec<JoinHandle<Result<()>>> {
    let mut handles = Vec::new();

    for i in 0..num_workers {
        let kube_client = kube_client.clone();
        let rabbitmq_client = rabbitmq_client.clone();
        let handle = tokio::spawn(async move {
            let worker = Worker::new(i, kube_client, rabbitmq_client);
            worker.start().await
        });

        handles.push(handle);
    }

    handles
}
