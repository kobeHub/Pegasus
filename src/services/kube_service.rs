use futures::executor::block_on;

use kube::{
    api::{Api},
    client::APIClient,
    config,
    runtime::Reflector
};
use lazy_static::{lazy_static};

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
    let resource = Api::v1Node(KUBE_CLIENT.clone());
    let rf = Reflector::new(resource)
    // .labels("kubernetes.io/lifecycle=spot")
        .timeout(10)
        .init()
        .await?;

    let results = rf
        .state()
        .await?
        .into_iter()
        .map(|object| {
        format!("Node:{}, labels:{:?}",
                object.metadata.name,
                object.metadata.labels)})
        .collect::<Vec<_>>();

    Ok(results)
}
