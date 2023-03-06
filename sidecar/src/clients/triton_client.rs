use std::{f32::consts::E, thread::sleep, time::Duration};

use crate::utils::repeat_until_success;
use anyhow::Result;
use reqwest::Url;
use tracing::info;

pub struct TritonClient {
    http_client: reqwest::Client,
    triton_base_url: Url,
}

impl TritonClient {
    pub fn new(url: &str) -> Result<Self> {
        Ok(TritonClient {
            triton_base_url: Url::parse(url)?,
            http_client: reqwest::Client::builder()
                .timeout(Duration::from_secs(20))
                .build()?,
        })
    }

    fn get_url_from_base(&self, relative: &str) -> Result<String> {
        match self.triton_base_url.join(relative) {
            Ok(url) => Ok(url.to_string()),
            Err(e) => Err(anyhow::anyhow!("Failed to join url: {}", e)),
        }
    }

    pub async fn live(&self) -> Result<()> {
        let live_endpoint = self.get_url_from_base("/v2/health/live")?;
        let response = self.http_client.get(live_endpoint).send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Server is not live"))
        }
    }

    pub async fn ready(&self) -> Result<()> {
        let ready_endpoint = self.get_url_from_base("/v2/health/ready")?;
        let response = self.http_client.get(ready_endpoint).send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Server is not live"))
        }
    }

    pub async fn wait_for_server_ready(&self) -> Result<()> {
        repeat_until_success(|| self.ready(), Duration::from_secs(5)).await?;
        info!("The Triton server is ready, waiting for it to be live...");
        repeat_until_success(|| self.live(), Duration::from_secs(5)).await?;
        info!("The Triton server is live and ready to receive requests!");
        Ok(())
    }
}
