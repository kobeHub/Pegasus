use actix_web::{get, post, web, HttpResponse, Scope};
use serde_json::json;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::models::namespace::Namespace;
use crate::models::user::User;
use crate::services::kube_service;

use std::collections::BTreeMap;

#[derive(Deserialize)]
struct UserInfo {
    pub id: Uuid,
}

#[get("/infos")]
async fn get_info(info: web::Query<UserInfo>) -> Result<HttpResponse, ApiError> {
    let uid = info.into_inner().id;

    if !User::exist_id(&uid)? {
        return Ok(HttpResponse::Ok().json(json!({
            "status": false,
            "data": "",
            "msg": "The user does not exist",
        })));
    }

    let namespaces = Namespace::get_ns_of(&uid)?;
    let mut deploys = BTreeMap::new();
    let mut services = BTreeMap::new();
    let mut pods = BTreeMap::new();
    for ns in &namespaces {
        let deploy = kube_service::get_deploy_within(ns).await?;
        let svc = kube_service::get_svc_within(ns).await?;
        let pod = kube_service::get_pod_within(ns).await?;

        deploys.insert(ns, deploy);
        services.insert(ns, svc);
        pods.insert(ns, pod);
    }
    Ok(HttpResponse::Ok().json(json!({
        "status": true,
        "data": {
            "deploy": deploys,
            "service": services,
            "pod": pods,
        },
        "msg": "",
    })))
}

pub fn tasks_scope() -> Scope {
    web::scope("/tasks").service(get_info)
}
