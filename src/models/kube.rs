use k8s_openapi::api::core::v1::{Container, Pod, Service, Namespace};
use k8s_openapi::api::apps::v1::Deployment;
use kube::api::Meta;

const AVAILABLE: &'static str = "Available";
const TRUE: &'static str = "True";
const RUNNING: &'static str = "Running";
const ACTIVE: &'static str = "Active";

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

pub type GetInfo = DeleteInfo;

/// Resources state the object item send to web client
#[derive(Serialize, Deserialize)]
pub struct ResourceState {
    pub name: String,
    pub state: bool,
}

/// The service info serialize in the service creatation
#[derive(Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub namespace: String,
    pub app_label: String,
    pub port: i32,
}

/// ResourceState `From` traits
impl From<&Deployment> for ResourceState {
    fn from(info: &Deployment) -> Self {
        let mut state = false;
        if let Some(status) = &info.status {
            if let Some(cons) = &status.conditions {
                state = cons[0].type_.as_str() == AVAILABLE &&
                    cons[0].status.as_str() == TRUE;
            }
        }
        ResourceState {
            name: Meta::name(info),
            state: state
        }
    }
}

impl From<&Pod> for ResourceState {
    fn from(info: &Pod) -> Self {
        let mut state = false;
        if let Some(status) = &info.status {
            if let Some(phase) = &status.phase {
                state = phase.as_str() == RUNNING;
            }
        }
        ResourceState {
            name: Meta::name(info),
            state: state,
        }
    }
}

impl From<&Namespace> for ResourceState {
    fn from(info: &Namespace) -> Self {
        let mut state = false;
        if let Some(status) = &info.status {
            if let Some(phase) = &status.phase {
                state = phase.as_str() == ACTIVE;
            }
        }
        ResourceState {
            name: Meta::name(info),
            state: state,
        }
    }
}

impl From<&Service> for ResourceState {
    fn from(info: &Service) -> Self {
        ResourceState {
            name: Meta::name(info),
            state: true,
        }
    }
}
