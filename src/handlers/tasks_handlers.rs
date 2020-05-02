use actix_web::{delete, get, post, web, HttpResponse, Scope};
use serde_json::json;
use uuid::Uuid;

use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::api::Meta;

use crate::errors::ApiError;
use crate::models::kube::{DeleteInfo, DeployInfo, GetInfo, ServiceInfo};
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
            "namespace": namespaces,
            "deploy": deploys,
            "service": services,
            "pod": pods,
       },
        "msg": "",
    })))
}

#[get("/deploy")]
async fn get_deploy(info: web::Query<GetInfo>) -> Result<HttpResponse, ApiError> {
    let res = kube_service::get_deploy_state(&info.namespace, &info.name).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[post("/deploy")]
async fn create_deploy(info: web::Json<DeployInfo>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();
    let res = kube_service::create_deploy(info).await?;
    Ok(HttpResponse::Ok().json(json!({
        "status": true,
        "msg": "Deployment create successfully",
        "data": res,
    })))
}

#[delete("/deploy")]
async fn delete_deploy(info: web::Json<DeleteInfo>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();

    let res = kube_service::delete_deploy(&info.namespace, &info.name).await?;
    Ok(HttpResponse::Ok().json(json!({
        "msg": res,
    })))
}

#[post("/replacedeploy")]
async fn replace_deploy(info: web::Json<Deployment>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();
    let name = &info.name();
    if let Some(ns) = info.meta().namespace.as_ref() {
        let o_patched = kube_service::replace_deploy(ns, name, &info).await?;

        Ok(HttpResponse::Ok().json(json!({
            "status": true,
            "msg": "Deployment edit successfully",
            "data": o_patched,
        })))
    } else {
        Ok(HttpResponse::Ok().json(json!({
            "status": false,
            "msg": "Namespace must be provided!",
        })))
    }
}

#[post("/svc")]
async fn create_svc(info: web::Json<ServiceInfo>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();

    let res = kube_service::create_svc(info).await?;
    Ok(HttpResponse::Ok().json(json!({
        "status": true,
        "msg": "Service create successfully",
        "data": res,
    })))
}

#[get("/svc")]
async fn get_svc(info: web::Query<GetInfo>) -> Result<HttpResponse, ApiError> {
    let res = kube_service::get_svc_state(&info.namespace, &info.name).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[delete("/svc")]
async fn delete_svc(info: web::Json<DeleteInfo>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();

    let msg = kube_service::delete_svc(&info.namespace, &info.name).await?;
    Ok(HttpResponse::Ok().json(json!({
        "msg": msg,
    })))
}

#[post("/replacesvc")]
async fn replace_svc(info: web::Json<Service>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();
    let name = &info.name();
    if let Some(ns) = info.meta().namespace.as_ref() {
        let o_patched = kube_service::replace_svc(ns, name, &info).await?;

        Ok(HttpResponse::Ok().json(json!({
            "status": true,
            "msg": "Service edit successfully",
            "data": o_patched,
        })))
    } else {
        Ok(HttpResponse::Ok().json(json!({
            "status": false,
            "msg": "Namespace must be provided!",
        })))
    }
}

#[delete("/pod")]
async fn delete_pod(info: web::Json<DeleteInfo>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();

    let msg = kube_service::delete_pod(&info.namespace, &info.name).await?;
    Ok(HttpResponse::Ok().json(json!({
        "msg": msg,
    })))
}

pub fn tasks_scope() -> Scope {
    web::scope("/tasks")
        .service(get_info)
        .service(get_deploy)
        .service(create_deploy)
        .service(delete_deploy)
        .service(get_svc)
        .service(create_svc)
        .service(delete_svc)
        .service(delete_pod)
        .service(replace_deploy)
        .service(replace_svc)
}
