use anyhow::Result;
use k8s_openapi::api::core::v1::Pod;
use kube::{api::PostParams, core::ObjectMeta, Api, Client};

pub async fn validation_server_pod(name: String) -> Result<Pod> {
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);
    let pp = PostParams::default();
    let p = Pod {
        metadata: ObjectMeta {
            name: Some(name),
            ..ObjectMeta::default()
        },
        spec: Some(Default::default()),
        ..Pod::default()
    };
    Ok(p)
}
