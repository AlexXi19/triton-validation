use std::sync::Arc;

use crate::clients::{
    kube_client::KubeClient,
    rabbitmq_client::{RabbitMQClient, ValidationMessage},
};
use crate::unwrap_with_nack;
use anyhow::Result;
use futures::StreamExt;
use tokio::task::JoinHandle;
use tracing::info;

use super::worker::Worker;
use tracing::error;

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
            let mut consumer = rabbitmq_client.create_consumer().await?;
            let worker = Worker::new(i, kube_client);
            while let Some(delivery) = consumer.next().await {
                let delivery = match delivery {
                    Ok(delivery) => delivery,
                    Err(e) => {
                        error!("Error while receiving message: {}", e);
                        continue;
                    }
                };
                let tag = delivery.delivery_tag;
                let message = unwrap_with_nack!(
                    parse_message(delivery),
                    rabbitmq_client,
                    tag,
                    "Failed to parse message"
                );
                let id = message.id.clone();
                info!("Worker {} received message", id);
                unwrap_with_nack!(
                    worker.process_job_start(message).await,
                    rabbitmq_client,
                    tag,
                    "Worker failed to process message"
                );
                info!("Worker {} processed message successfully!", id);
            }

            Ok(())
        });

        handles.push(handle);
    }

    handles
}

fn parse_message(delivery: lapin::message::Delivery) -> Result<ValidationMessage> {
    let message = String::from_utf8(delivery.data.to_vec())?;
    let message: ValidationMessage = serde_json::from_str(&message)?;
    Ok(message)
}
