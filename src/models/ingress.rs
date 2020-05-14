use  k8s_openapi::api::extensions::v1beta1::{Ingress, HTTPIngressPath};

/// Basic Ingress information to operates ingress
/// object in kubernetes
#[derive(Serialize, Deserialize)]
pub struct IngressInfo {
    pub name: String,
    pub ns: String,
    pub host: String,
    pub paths: Vec<IngressPath>,
}

#[derive(Serialize, Deserialize)]
pub struct IngressPath {
    pub path: Option<String>,
    pub svc_name: String,
    pub svc_port: i32,
}
