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
    k8s::pods::{test_incorrect_pod, test_pod, validation_pod},
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
        let p = validation_pod(message.name);
        self.kube_client.create_pod(p).await?;
        Ok(())
    }
}
