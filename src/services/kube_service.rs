use futures::executor::block_on;
use kube::{
    api::{Api, ListParams, PostParams, DeleteParams},
    client::APIClient,
    config,
};
use k8s_openapi::api::core::v1::{
    Namespace,
    Node
};
use lazy_static::lazy_static;
use serde_json::json;

use std::vec::Vec;

use crate::errors::ApiError;

lazy_static! {
    pub static ref KUBE_CLIENT: APIClient = {
        let config = block_on(config::load_kube_config())
            .expect("Please config your k8s cluster correctly!");
        APIClient::new(config)
    };
}

pub async fn get_nodes() -> Result<Vec<String>, ApiError> {
    let nodes: Api<Node> = Api::all(KUBE_CLIENT.clone());
    let results = nodes
        .list(&ListParams::default())
        .await?
        .into_iter()
        .map(|x| format!("{}", x.metadata.unwrap().name.unwrap()))
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
    let res = namespace
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
