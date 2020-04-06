use futures::executor::block_on;
use kube::{
    api::{Api, Meta, ListParams, PostParams, DeleteParams},
    client::Client,
};
use k8s_openapi::api::core::v1::{
    Namespace,
    Node,
    Service,
    Pod,
};
use k8s_openapi::api::apps::v1::Deployment;
use lazy_static::lazy_static;
use serde_json::json;

use std::vec::Vec;
use std::collections::BTreeMap;

use crate::errors::ApiError;

lazy_static! {
    pub static ref KUBE_CLIENT: Client = {
        block_on(Client::infer())
            .expect("Please config your k8s cluster correctly!")
    };
}

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
    let _res = namespace
        .create(
            &PostParams::default(),
            &ns_obj,
        ).await?;
    Ok(())
}

pub async fn delete_ns(ns: &str) -> Result<bool, ApiError> {
    let resource: Api<Namespace> = Api::all(KUBE_CLIENT.clone());
    let result = resource
        .delete(ns, &DeleteParams::default()).await?;

    // TODO: handle right status
    Ok(result.is_left())
}

// TODO: Add deploy, service, pod list
pub async fn get_deploy_within(ns: &str) -> Result<Vec<String>, ApiError> {
    let deploys: Api<Deployment> = Api::namespaced(KUBE_CLIENT.clone(), ns);
    let results: Vec<String> = deploys
        .list(&ListParams::default())
        .await?
        .iter()
        .map(Meta::name)
        .collect();
    Ok(results)
}

pub async fn get_svc_within(ns: &str) -> Result<Vec<String>, ApiError> {
    let svc: Api<Service> = Api::namespaced(KUBE_CLIENT.clone(), ns);
    let results = svc
        .list(&ListParams::default())
        .await?
        .iter()
        .map(Meta::name)
        .collect();
    Ok(results)
}

/// Deployment and Pod labels:
///
/// All the pods belong to same deploy has same label `app=`
/// `get_pod_within` will return a hashmap with deploy name
/// as keys, pod name as value
pub async fn get_pod_within(ns: &str) -> Result<BTreeMap<String, Vec<String>>, ApiError> {
    let deploys: Api<Deployment> = Api::namespaced(KUBE_CLIENT.clone(), ns);
    let pods: Api<Pod> = Api::namespaced(KUBE_CLIENT.clone(), ns);

    let deploy = deploys.list(&ListParams::default()).await?;
    let pod = pods.list(&ListParams::default()).await?;

    let mut result = BTreeMap::new();
    for d in &deploy {
        let deploy_label = &Meta::meta(d).labels.as_ref().unwrap()["app"];
        let deploy_name = Meta::name(d);
        result.insert(deploy_name.clone(), Vec::new());
        for p in &pod {
            let pod_label = &Meta::meta(p).labels.as_ref().unwrap()["app"];
            if deploy_label == pod_label {
                if let Some(x) = result.get_mut(&deploy_name) {
                    x.push(Meta::name(p))
                }
            }
        }
    }
    Ok(result)
}
