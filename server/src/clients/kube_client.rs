use std::{path::Path, sync::Arc};

use anyhow::Result;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::PostParams,
    core::ObjectMeta,
    runtime::wait::{await_condition, conditions::is_pod_running},
    Api, Client, ResourceExt,
};
use kube::{client::ConfigExt, Config};
use serde_json::json;
use tower::ServiceBuilder;
use tracing::info;

use crate::k8s::pods::validation_server_pod;

pub struct KubeClient {
    client: Client,
    config: Config,
    namespace: String,
}

impl KubeClient {
    pub async fn new(config_path: String) -> Result<Self> {
        let kubeconfig_path = Path::new(config_path.as_str());
        // let config = Config::from_kubeconfig(&kubeconfig_path)?;
        let config = Config::infer().await?;
        let namespace = config.default_namespace.clone();
        let service = ServiceBuilder::new()
            .layer(config.base_uri_layer())
            .option_layer(config.auth_layer()?)
            .service(hyper::Client::new());
        let client = Client::new(service, namespace.clone());

        info!("Kubernetes client initialized");
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

    pub async fn test_create_pod(&self) -> Result<()> {
        let client = self.client.clone();
        let pods: Api<Pod> = Api::default_namespaced(client);

        // Create Pod blog
        info!("Creating Pod instance blog");
        let p = validation_server_pod("hi".to_ascii_lowercase()).await?;
        let pp = PostParams::default();
        match pods.create(&pp, &p).await {
            Ok(o) => {
                let name = o.name_any();
                assert_eq!(p.name_any(), name);
                info!("Created {}", name);
            }
            Err(kube::Error::Api(ae)) => assert_eq!(ae.code, 409), // if you skipped delete, for instance
            Err(e) => return Err(e.into()),                        // any other case is probably bad
        }

        // Watch it phase for a few seconds
        let establish = await_condition(pods.clone(), "blog", is_pod_running());
        let _ = tokio::time::timeout(std::time::Duration::from_secs(15), establish).await?;

        info!("Pod test is running");
        Ok(())
    }
}
