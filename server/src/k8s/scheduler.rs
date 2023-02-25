use anyhow::Result;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::PostParams,
    core::ObjectMeta,
    runtime::wait::{await_condition, conditions::is_pod_running},
    Api, Client, ResourceExt,
};
use serde_json::json;
use tracing::info;

use crate::k8s::pods::validation_server_pod;

pub async fn create_validation_environment(name: String) -> Result<()> {
    info!("Creating validation environment...");
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);

    // Create Pod blog
    info!("Creating Pod instance blog");
    let p = validation_server_pod(name).await?;
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
