use anyhow::Result;
use serde_json::Value;

pub struct Variables {
    pub kubeconfig: String,
    pub http_port: String,
    pub num_workers: u16,
    pub amqp_url: String,
    pub queue_name: String,
}

impl Variables {
    pub fn get_env_vars() -> Result<Variables> {
        Ok(Self {
            kubeconfig: get_env_var("KUBECONFIG", Some("~/.kube/config"))?,
            http_port: get_env_var("HTTP_PORT", Some("8080"))?,
            num_workers: get_env_var("NUM_WORKERS", Some("1"))?.parse()?,
            amqp_url: get_env_var("AMQP_URL", Some("amqp://localhost:5672/%2f"))?,
            queue_name: get_env_var("QUEUE_NAME", Some("validation"))?,
        })
    }
}

fn get_env_var(name: &str, default: Option<&str>) -> Result<String> {
    match std::env::var(name) {
        Ok(val) => Ok(val),
        Err(_) => match default {
            Some(val) => Ok(val.to_string()),
            None => Err(anyhow::anyhow!("{} is not set", name)),
        },
    }
}
