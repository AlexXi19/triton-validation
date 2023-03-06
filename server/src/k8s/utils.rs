use anyhow::{anyhow, Result};
use std::sync::Arc;
use tracing::info;

use crate::{clients::kube_client::KubeClient, k8s::constants::JOB_ID_LABEL};

pub async fn delete_pod_name_from_job_id(kube_client: Arc<KubeClient>, job_id: &str) -> Result<()> {
    info!("Getting pod name from job id...");

    let pods = kube_client
        .get_pods_from_label(JOB_ID_LABEL, job_id)
        .await?;

    if pods.is_empty() {
        return Err(anyhow!("Could not find job for id: {}", job_id));
    } else if pods.len() > 1 {
        return Err(anyhow!("More than one pod found with the job_id"));
    }

    let name_option = pods[0].metadata.name.clone();
    let pod_name = match name_option {
        Some(name) => name,
        None => return Err(anyhow!("Could not find pod name")),
    };

    kube_client.delete_pod_blocking(pod_name.as_str()).await?;

    Ok(())
}
