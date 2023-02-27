use anyhow::Result;

pub struct Variables {
    pub job_id: String,
    pub triton_url: String,
}

impl Variables {
    pub fn get_env_vars() -> Result<Variables> {
        Ok(Self {
            job_id: get_env_var("VALIDATION_JOB_ID", Some(""))?,
            triton_url: get_env_var("TRITON_URL", Some("http://localhost:8001"))?,
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
