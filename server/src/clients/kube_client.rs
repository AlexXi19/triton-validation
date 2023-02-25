use std::path::Path;

use anyhow::Result;
use kube::{client::ConfigExt, Client, Config};
use tower::ServiceBuilder;
use tracing::info;

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
}
