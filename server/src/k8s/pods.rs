use anyhow::Result;
use futures::io::empty;
use k8s_openapi::{
    api::core::v1::{
        Container, ContainerPort, EnvVar, Pod, PodSpec, Service, ServicePort, ServiceSpec, Volume,
        VolumeMount,
    },
    apimachinery::pkg::util::intstr::IntOrString,
};
use kube::{api::PostParams, core::ObjectMeta, Api, Client};
use serde_json::json;

use super::constants::TRITON_IMAGE;

pub fn validation_service(name: String) -> Service {
    let validation_service = Service {
        metadata: ObjectMeta {
            name: Some(name),
            ..Default::default()
        },
        spec: Some(ServiceSpec {
            ports: Some(vec![
                ServicePort {
                    port: 8000,
                    target_port: Some(IntOrString::Int(8000)),
                    ..Default::default()
                },
                ServicePort {
                    port: 8001,
                    target_port: Some(IntOrString::Int(8001)),
                    ..Default::default()
                },
                ServicePort {
                    port: 8002,
                    target_port: Some(IntOrString::Int(8002)),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        }),
        ..Default::default()
    };

    validation_service
}

pub fn validation_pod(name: String, job_id: String) -> Pod {
    let triton_container = Container {
        name: "triton".to_string(),
        image: Some(TRITON_IMAGE.to_string()),
        command: Some(vec!["tritonserver".to_string()]),
        args: Some(vec![
            "--model-repository=/models".to_string(),
            "--grpc-port=8000".to_string(),
            "--http-port=8001".to_string(),
            "--metrics-port=8002".to_string(),
            "--model-control-mode=explicit".to_string(),
            "--log-verbose=3".to_string(),
        ]),
        ports: Some(vec![
            ContainerPort {
                container_port: 8000,
                ..Default::default()
            },
            ContainerPort {
                container_port: 8001,
                ..Default::default()
            },
            ContainerPort {
                container_port: 8002,
                ..Default::default()
            },
        ]),
        volume_mounts: Some(vec![VolumeMount {
            name: "model-repository".to_string(),
            mount_path: "/models".to_string(),
            ..Default::default()
        }]),
        ..Default::default()
    };

    // TODO: Complete this pod
    let validation_container = Container {
        name: "validation".to_string(),
        env: Some(vec![EnvVar {
            name: "VALIDATION_JOB_ID".to_string(),
            value: Some(job_id),
            ..Default::default()
        }]),
        ..Default::default()
    };

    let triton_volume = Volume {
        name: "model-repository".to_string(),
        ..Default::default()
    };

    let pod = Pod {
        metadata: ObjectMeta {
            name: Some(name),
            ..Default::default()
        },
        spec: Some(PodSpec {
            containers: vec![triton_container],
            volumes: Some(vec![triton_volume]),
            ..Default::default()
        }),
        ..Default::default()
    };

    pod
}

pub fn test_pod(name: String) -> Result<Pod> {
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

pub fn test_incorrect_pod(name: String) -> Result<Pod> {
    let empty_pod = serde_json::from_value(json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": {
            "name1": name, // Incorrect field name
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
