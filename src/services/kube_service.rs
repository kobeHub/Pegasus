use futures::executor::block_on;

use kube::{
    api::{Api, ListParams, PostParams},
    client::APIClient,
    config,
    runtime::Reflector
};
use lazy_static::{lazy_static};
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
    let nodes = Api::v1Node(KUBE_CLIENT.clone());
    let results = nodes
        .list(&ListParams::default())
        .await?
        .into_iter()
        .map(|x| format!("{}: {}", x.metadata.name, x.metadata.labels["pegasus-role"]))
        .collect();
    Ok(results)
}

pub async fn create_ns<T>(ns: T) -> Result<String, ApiError>
where
    T: Into<String>
{
    let namespace = Api::v1Namespace(KUBE_CLIENT.clone());
    let res = namespace
        .create(&PostParams::default(), serde_json::to_vec(&json!({
            "metadata": {"name": ns.into()}
        }))?)
        .await?;
        //.map(|x| format!("active_deadline_time: {}", x.spec));
    Ok(format!("{}", res.metadata.name))
}
