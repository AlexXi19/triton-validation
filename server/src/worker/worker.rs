use anyhow::Result;
use lapin::{
    options::{BasicAckOptions, BasicPublishOptions},
    BasicProperties,
};
use std::{sync::Arc, time::Duration};
use tracing::info;
use uuid::Uuid;

use crate::{
    clients::{
        kube_client::KubeClient,
        rabbitmq_client::{RabbitMQClient, ValidationMessage},
    },
    k8s::pods::{test_incorrect_pod, test_pod},
};

pub struct Worker {
    id: u16,
    kube_client: Arc<KubeClient>,
}

impl Worker {
    pub fn new(id: u16, kube_client: Arc<KubeClient>) -> Self {
        Self { id, kube_client }
    }

    pub async fn process_job_start(&self, message: ValidationMessage) -> Result<()> {
        info!("Worker {} received message: {}", self.id, message.id);

        let p = test_pod(format!("{}-validation", message.name)).await?;
        self.kube_client.create_pod_blocking(p).await.unwrap();

        info!("Worker {} processed message successfully!", self.id);
        Ok(())
    }
}
