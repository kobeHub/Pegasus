use k8s_openapi::api::core::v1::Container;

/// The Contianer info to be serialize in the deploy
/// creatation string, shiped from `k8s-openapi`
#[derive(Serialize, Deserialize)]
pub struct DeployInfo {
    pub name: String,
    pub namespace: String,
    pub reschedulable: bool,
    pub app_label: String,
    pub replicas: i32,
    pub containers: Vec<Container>,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteInfo {
    pub name: String,
    pub namespace: String,
}

/// The service info serialize in the service creatation
#[derive(Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub namespace: String,
    pub app_label: String,
    pub port: i32,
}
