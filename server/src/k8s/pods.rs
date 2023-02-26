use anyhow::Result;
use futures::io::empty;
use k8s_openapi::api::core::v1::Pod;
use kube::{api::PostParams, core::ObjectMeta, Api, Client};
use serde_json::json;

pub async fn test_pod(name: String) -> Result<Pod> {
    let empty_pod = serde_json::from_value(json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": {
            "name": name,
        },
        "hi": "there",
        "spec": {
            "containers": [{
                "name": "empty",
                "image": "alpine:latest",
                "command": ["tail", "-f", "/dev/null"]
            }],
        }
    }))?;

    Ok(empty_pod)
}

pub async fn test_incorrect_pod(name: String) -> Result<Pod> {
    let empty_pod = serde_json::from_value(json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": {
            "name1": name,
        },
        "spec": {
            "containers": [{
                "name": "empty",
                "image": "alpine:latest",
                "command": ["tail", "-f", "/dev/null"]
            }],
        }
    }))?;

    Ok(empty_pod)
}
