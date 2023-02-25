use anyhow::Result;
use futures::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicPublishOptions},
    BasicProperties,
};
use std::{sync::Arc, time::Duration};
use tracing::info;

use crate::clients::{kube_client::KubeClient, rabbitmq_client::RabbitMQClient};

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
        }
        Ok(())
    }
}
