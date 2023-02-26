use anyhow::Result;
use lapin::{
    options::{
        BasicConsumeOptions, BasicNackOptions, BasicPublishOptions, BasicQosOptions,
        ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions,
    },
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties, Consumer,
};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ValidationMessage {
    pub id: String,
    pub name: String,
}
pub struct RabbitMQClient {
    pub connection: Connection,
    pub channel: Channel,
    pub validation_queue_name: String,
    pub dl_queue_name: String,
}

impl RabbitMQClient {
    pub async fn new(url: &str, queue_name: &str) -> Result<Self> {
        let connection = Connection::connect(url, ConnectionProperties::default()).await?;

        let channel = connection.create_channel().await?;

        // set queue options to create queue if it doesn't exist
        let queue_options = QueueDeclareOptions::default();

        // declare main queue
        channel
            .queue_declare(queue_name, queue_options, FieldTable::default())
            .await?;

        // declare dead letter queue
        let dl_queue_name = format!("{}-dl", queue_name);
        channel
            .queue_declare(&dl_queue_name, queue_options, FieldTable::default())
            .await?;

        // Exchange
        let exchange_options = ExchangeDeclareOptions::default();
        channel
            .exchange_declare(
                "validation-dl",
                lapin::ExchangeKind::Fanout,
                exchange_options,
                FieldTable::default(),
            )
            .await?;

        // Bind the DLQ to the main queue with a routing key
        channel
            .queue_bind(
                queue_name,
                dl_queue_name.as_str(),
                "my-routing-key",
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(Self {
            connection,
            channel,
            validation_queue_name: queue_name.to_string(),
            dl_queue_name,
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

    pub async fn nack_to_dlq(&self, delivery_tag: u64) -> Result<()> {
        let nack_options = BasicNackOptions {
            multiple: false,
            requeue: false,
        };
        let properties = BasicProperties::default();
        let publish_options = BasicPublishOptions::default();
        self.channel.basic_nack(delivery_tag, nack_options).await?;
        self.channel
            .basic_publish("", &self.dl_queue_name, publish_options, b"", properties)
            .await?;

        info!("Nacked message with delivery tag: {}", delivery_tag);
        Ok(())
    }
}
