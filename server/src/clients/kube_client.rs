use std::{path::Path, sync::Arc};

use anyhow::Result;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::PostParams,
    core::ObjectMeta,
    runtime::{
        wait::{await_condition, conditions::is_pod_running},
        Controller,
    },
    Api, Client, ResourceExt,
};
use kube::{client::ConfigExt, Config};
use serde_json::json;
use tracing::info;

pub struct KubeClient {
    client: Client,
    config: Config,
    namespace: String,
}

impl KubeClient {
    pub async fn new(config_path: String) -> Result<Self> {
        // let config = Config::from_kubeconfig(&)?;
        let config = Config::infer().await?;
        let namespace = config.default_namespace.clone();
        let client = Client::try_default().await?;

        info!(
            "Kubernetes client initialized with namespace: {}, cluster url: {:?}, and context: {:?}",
            namespace, config.cluster_url, "Unknown context"
        );
        Ok(KubeClient {
            client,
            config,
            namespace,
        })
    }

    pub async fn health(&self) -> Result<()> {
        info!("Health check passed for kube client");
        Ok(())
    }

    pub async fn get_pod(&self, name: &str) -> Result<Pod> {
        let client = self.client.clone();
        let pods: Api<Pod> = Api::default_namespaced(client);
        let pod = pods.get(name).await?;

        info!("Pod {} found", name);
        Ok(pod)
    }

    pub async fn create_pod(&self, p: Pod) -> Result<()> {
        let client = self.client.clone();
        let pods: Api<Pod> = Api::default_namespaced(client);
        let pod_name = p.name_any();

        let pp = PostParams::default();

        info!("Creating Pod");
        pods.create(&pp, &p).await?;

        info!("Pod {} created", pod_name);
        Ok(())
    }
    
    pub async fn create_pod_blocking(&self, p: Pod) -> Result<()> {
        let client = self.client.clone();
        let pods: Api<Pod> = Api::default_namespaced(client);
        let pod_name = p.name_any();

        let pp = PostParams::default();

        pods.create(&pp, &p).await?;
        // Create Pod blog
        info!("Creating Pod");
        let establish = await_condition(pods.clone(), pod_name.as_str(), is_pod_running());
        let _ = tokio::time::timeout(std::time::Duration::from_secs(30), establish).await?;

        info!("Pod {} created", pod_name);
        Ok(())
    }
}
