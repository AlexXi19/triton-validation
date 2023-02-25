use anyhow::Result;
use serde_json::Value;

pub struct Variables {
    pub kubeconfig: String,
    pub http_port: String,
}

impl Variables {
    pub fn get_env_vars() -> Result<Variables> {
        Ok(Self {
            kubeconfig: get_env_var("KUBECONFIG", Some("~/.kube/config"))?,
            http_port: get_env_var("HTTP_PORT", Some("8080"))?,
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
