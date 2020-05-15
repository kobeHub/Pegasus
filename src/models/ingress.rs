use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

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

#[derive(Default, Serialize)]
pub struct IngressResponse {
    pub name: String,
    pub namespace: String,
    pub host: String,
    pub svc_name: String,
    pub svc_port: IntOrString,
}
