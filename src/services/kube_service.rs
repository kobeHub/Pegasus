use futures::executor::block_on;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::{Namespace, Node, Pod, Service};
use kube::{
    api::{Api, DeleteParams, ListParams, LogParams, Meta, PostParams},
    client::Client,
};
use lazy_static::lazy_static;
use serde_json::json;

use std::collections::BTreeMap;
use std::vec::Vec;

use crate::errors::ApiError;
use crate::models::kube::{DeployInfo, ResourceState, ServiceInfo};

lazy_static! {
    pub static ref KUBE_CLIENT: Client =
        { block_on(Client::infer()).expect("Please config your k8s cluster correctly!") };
}

/// Get all nodes names, return a vector of String
pub async fn get_nodes() -> Result<Vec<String>, ApiError> {
    let nodes: Api<Node> = Api::all(KUBE_CLIENT.clone());
    let results = nodes
        .list(&ListParams::default())
        .await?
        .iter()
        .map(Meta::name)
        .collect();
    Ok(results)
}

/// Create a namespace in the cluster
/// All the users' namespaces created with a label `dispense=pegasus`
pub async fn create_ns<T>(ns: T) -> Result<(), ApiError>
where
    T: Into<String>,
{
    let namespace: Api<Namespace> = Api::all(KUBE_CLIENT.clone());
    let ns_obj: Namespace = serde_json::from_value(json!({
        "apiVersion": "v1",
        "kind": "Namespace",
        "metadata": {
            "name": ns.into(),
            "labels": {
                "dispense": "pegasus",
            },
        },
    }))?;
    let _res = namespace.create(&PostParams::default(), &ns_obj).await?;
    Ok(())
}

/// Delete namespace with name `ns`
pub async fn delete_ns(ns: &str) -> Result<String, ApiError> {
    let resource: Api<Namespace> = Api::all(KUBE_CLIENT.clone());
    let res = resource.delete(ns, &DeleteParams::default()).await?;

    // TODO: handle right status
    if res.is_left() {
        Ok(format!("Deleting namepsace {}", ns))
    } else {
        Ok(format!("Deleted successfully"))
    }
}

/// Get all deploy within a namespace
pub async fn get_deploy_within(ns: &str) -> Result<Vec<ResourceState>, ApiError> {
    let deploys: Api<Deployment> = Api::namespaced(KUBE_CLIENT.clone(), ns);
    let results: Vec<ResourceState> = deploys
        .list(&ListParams::default())
        .await?
        .iter()
        .map(ResourceState::from)
        .collect();
    Ok(results)
}

pub async fn get_svc_within(ns: &str) -> Result<Vec<ResourceState>, ApiError> {
    let svc: Api<Service> = Api::namespaced(KUBE_CLIENT.clone(), ns);
    let results = svc
        .list(&ListParams::default())
        .await?
        .iter()
        .map(ResourceState::from)
        .collect();
    Ok(results)
}

/// Get all the deployment and pods mapping within a namespace
///
/// All the pods belong to same deploy has at least onesame label
/// `get_pod_within` will return a hashmap with deploy name
/// as keys, pod name as value
pub async fn get_pod_within(ns: &str) -> Result<BTreeMap<String, Vec<ResourceState>>, ApiError> {
    let deploys: Api<Deployment> = Api::namespaced(KUBE_CLIENT.clone(), ns);
    let pods: Api<Pod> = Api::namespaced(KUBE_CLIENT.clone(), ns);

    let deploy = deploys.list(&ListParams::default()).await?;
    let pod = pods.list(&ListParams::default()).await?;

    let mut result = BTreeMap::new();
    for d in &deploy {
        if let Some(deploy_spec) = &d.spec {
            if let Some(match_labels) = &deploy_spec.selector.match_labels {
                let deploy_name = Meta::name(d);
                result.insert(deploy_name.clone(), Vec::new());
                for p in &pod {
                    if let Some(pod_meta) = &p.metadata {
                        if let Some(pod_labels) = &pod_meta.labels {
                            for (deploy_key, deploy_value) in match_labels.iter() {
                                if pod_labels.contains_key(deploy_key)
                                    && pod_labels[deploy_key] == *deploy_value
                                {
                                    if let Some(x) = result.get_mut(&deploy_name) {
                                        x.push(ResourceState::from(p))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(result)
}

/// Create a deployment with basic config
pub async fn create_deploy(deploy_info: DeployInfo) -> Result<Deployment, ApiError> {
    let resource: Api<Deployment> = Api::namespaced(KUBE_CLIENT.clone(), &deploy_info.namespace);

    let mut deploy_obj: Deployment = serde_json::from_value(json!({
        "apiVersion": "apps/v1",
        "kind": "Deployment",
        "metadata": {
            "name": deploy_info.name,
            "namespace": deploy_info.namespace,
            "labels": {
                "pegausus.state/reschedulable": deploy_info.reschedulable.to_string(),
                "pegasus.state/dispense": "pegasus",
                "pegasus.name/app": deploy_info.app_label,
            },
        },
        "spec": {
            "replicas": deploy_info.replicas,
            "selector": {
                "matchLabels": {
                    "pegasus.name/app": deploy_info.app_label,
                }
            },
            "strategy": {
                "type": "RollingUpdate",
                "rollingUpdate": {
                    "maxSurge": "25%",
                    "maxUnavailable": "25%",
                }
            },
            "template": {
                "metadata": {
                    "labels": {
                        "pegasus.name/app": deploy_info.app_label,
                        "pegasus.state/reschedulable": deploy_info.reschedulable.to_string(),
                    }
                },
                "spec": {
                    "containers": [],
                },
            },
        },
    }))?;

    if let Some(ref mut spec) = deploy_obj.spec.as_mut() {
        if let Some(ref mut temp) = spec.template.spec.as_mut() {
            temp.containers = deploy_info.containers;
        }
    }
    let res = resource.create(&PostParams::default(), &deploy_obj).await?;

    Ok(res)
}

/// Get deployment current state
pub async fn get_deploy_state(ns: &str, name: &str) -> Result<Deployment, ApiError> {
    let resource: Api<Deployment> = Api::namespaced(KUBE_CLIENT.clone(), ns);
    let deploy = resource.get(name).await?;
    Ok(deploy)
}

/// Replace deployment
pub async fn replace_deploy(
    ns: &str,
    name: &str,
    deploy: &Deployment,
) -> Result<Deployment, ApiError> {
    let resource: Api<Deployment> = Api::namespaced(KUBE_CLIENT.clone(), ns);
    let pp = PostParams::default();
    let deploy = resource.replace(name, &pp, deploy).await?;
    Ok(deploy)
}

/// Delete a deploy in spefic namespace
pub async fn delete_deploy(ns: &str, name: &str) -> Result<String, ApiError> {
    let resource: Api<Deployment> = Api::namespaced(KUBE_CLIENT.clone(), ns);
    let res = resource.delete(name, &DeleteParams::default()).await?;

    if res.is_left() {
        Ok(format!("Deleting deploy {}:{}", ns, name))
    } else {
        Ok(format!("Deleted successfully"))
    }
}

/// Get current Service object
pub async fn get_svc_state(ns: &str, name: &str) -> Result<Service, ApiError> {
    let resource: Api<Service> = Api::namespaced(KUBE_CLIENT.clone(), ns);
    let svc = resource.get(name).await?;
    Ok(svc)
}

/// Create service with baisc config
pub async fn create_svc(info: ServiceInfo) -> Result<Service, ApiError> {
    let resource: Api<Service> = Api::namespaced(KUBE_CLIENT.clone(), &info.namespace);

    let svc_obj: Service = serde_json::from_value(json!({
        "apiVersion": "v1",
        "kind": "Service",
        "metadata": {
            "name": info.name,
            "namespace": info.namespace,
            "labels": {
                "pegasus.state/dispense": "pegasus",
                "pegasus.name/svc": format!("{}-service", info.app_label),
            },
        },
        "spec": {
            "ports": [{
                "name": "default-http",
                "port": info.port,
                "protocol": "TCP",
                "targetPort": "default-http",
            }],
            "selector": {
                "pegasus.name/app": info.app_label,
            },
        },
    }))?;

    let res = resource.create(&PostParams::default(), &svc_obj).await?;
    Ok(res)
}

/// Delete given service
pub async fn delete_svc(ns: &str, name: &str) -> Result<String, ApiError> {
    let resource: Api<Service> = Api::namespaced(KUBE_CLIENT.clone(), ns);
    let res = resource.delete(name, &DeleteParams::default()).await?;

    if res.is_left() {
        Ok(format!("Deleting service {}:{}", ns, name))
    } else {
        Ok("Deleted service successfully".to_string())
    }
}

/// Repalce Service
pub async fn replace_svc(ns: &str, name: &str, svc: &Service) -> Result<Service, ApiError> {
    let resource: Api<Service> = Api::namespaced(KUBE_CLIENT.clone(), ns);
    let svc = resource.replace(name, &PostParams::default(), svc).await?;
    Ok(svc)
}

pub async fn delete_pod(ns: &str, name: &str) -> Result<String, ApiError> {
    let resource: Api<Pod> = Api::namespaced(KUBE_CLIENT.clone(), ns);
    let res = resource.delete(name, &DeleteParams::default()).await?;

    if res.is_left() {
        Ok(format!("Deleting pod {}:{}", ns, name))
    } else {
        Ok("Deleted pod successfully".to_string())
    }
}

/// `app_label` identify one app in a namespace
pub async fn get_label_map(ns: &str) -> Result<Vec<Option<String>>, ApiError> {
    let resource: Api<Deployment> = Api::namespaced(KUBE_CLIENT.clone(), ns);
    let results = resource
        .list(&ListParams::default())
        .await?
        .iter()
        .map(|x| {
            if let Some(labels) = &x.meta().labels {
                if labels.contains_key("pegasus.name/app") {
                    Some(labels["pegasus.name/app"].clone())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .filter(|x| x.is_some())
        .collect();
    Ok(results)
}

/// Get all containers within a pod
pub async fn get_containers_within(ns: &str, pod: &str) -> Result<Option<Vec<String>>, ApiError> {
    let resource: Api<Pod> = Api::namespaced(KUBE_CLIENT.clone(), ns);

    let results: Option<Vec<String>> = resource.get(pod)
        .await?
        .spec
        .map(|x| {
            x.containers.iter()
                .map(|c| c.name.clone())
                .collect()
        });
    Ok(results)
}

/// Get one container log in pod
pub async fn get_pod_log(ns: &str, pod: &str, container: Option<String>) -> Result<String, ApiError> {
    let resource: Api<Pod> = Api::namespaced(KUBE_CLIENT.clone(), ns);
    let mut param = LogParams::default();
    param.container = container;

    let result = resource.logs(pod, &param)
        .await?;
    Ok(result)
}
