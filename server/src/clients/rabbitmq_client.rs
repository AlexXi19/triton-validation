use anyhow::Result;
use lapin::{
    options::{BasicConsumeOptions, BasicPublishOptions, BasicQosOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties, Consumer,
};
use serde::Serialize;
use tracing::info;

#[derive(Serialize)]
pub struct ValidationMessage {
    pub id: String,
    pub name: String,
}
pub struct RabbitMQClient {
    pub connection: Connection,
    pub channel: Channel,
    pub validation_queue_name: String,
}

impl RabbitMQClient {
    pub async fn new(url: &str, queue_name: &str) -> Result<Self> {
        let connection = Connection::connect(url, ConnectionProperties::default()).await?;

        let channel = connection.create_channel().await?;

        // set queue options to create queue if it doesn't exist
        let queue_options = QueueDeclareOptions::default();

        // create queue with given options
        channel
            .queue_declare(queue_name, queue_options, FieldTable::default())
            .await?;

        Ok(Self {
            connection,
            channel,
            validation_queue_name: queue_name.to_string(),
        })
    }

    pub async fn create_consumer(&self) -> Result<Consumer> {
        let consumer = self
            .channel
            .basic_consume(
                &self.validation_queue_name,
                "Validation worker",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!(
            "Created consumer for queue with name: {}",
            self.validation_queue_name
        );
        return Ok(consumer);
    }

    pub async fn publish_message<T: Serialize>(&self, message: &T) -> Result<()> {
        let payload = serde_json::to_vec(&message)?;
        self.channel
            .basic_publish(
                "",
                &self.validation_queue_name,
                BasicPublishOptions::default(),
                payload.as_slice(),
                BasicProperties::default(),
            )
            .await?;

        info!("Published message to queue {}", self.validation_queue_name);
        Ok(())
    }
}
