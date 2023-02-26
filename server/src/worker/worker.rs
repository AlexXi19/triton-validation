use anyhow::Result;
use futures::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicPublishOptions},
    BasicProperties,
};
use std::{sync::Arc, time::Duration};
use tracing::info;
use uuid::Uuid;

use crate::{
    clients::{kube_client::KubeClient, rabbitmq_client::RabbitMQClient},
    k8s::pods::{test_incorrect_pod, test_pod},
};

pub struct Worker {
    id: u16,
    kube_client: Arc<KubeClient>,
    rabbitmq_client: Arc<RabbitMQClient>,
}

impl Worker {
    pub fn new(
        id: u16,
        kube_client: Arc<KubeClient>,
        rabbitmq_client: Arc<RabbitMQClient>,
    ) -> Self {
        Self {
            id,
            kube_client,
            rabbitmq_client,
        }
    }

    pub async fn start(&self) -> Result<()> {
        info!("Worker {} started", self.id);
        let mut consumer = self.rabbitmq_client.create_consumer().await?;
        while let Some(delivery) = consumer.next().await {
            let delivery = delivery.expect("error in consumer");
            let message = std::str::from_utf8(&delivery.data)?;
            info!("Worker {} received message: {}", self.id, message);

            let id = Uuid::new_v4().to_string();
            let p = test_incorrect_pod(format!("test-mode-{}", id)).await?;
            self.kube_client.create_pod(p).await?;
            self.kube_client.get_pod("nginx").await.unwrap();

            // Sleep for 7 secs
            tokio::time::sleep(Duration::from_secs(7)).await;
            // self.kube_client.get_pod("hi").await.unwrap();

            // Ack
            delivery.ack(BasicAckOptions::default()).await?
        }
        Ok(())
    }
}
