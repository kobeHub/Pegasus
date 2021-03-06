use actix_web::{get, post, web, HttpResponse, Scope};
use serde_json::json;

use crate::errors::ApiError;
use crate::services::kube_service;

#[get("/nodes")]
async fn get_nodes_info() -> Result<HttpResponse, ApiError> {
    let res = kube_service::get_nodes().await?;
    Ok(HttpResponse::Ok().json(json!({ "data": res })))
}

#[derive(Deserialize)]
struct NamespaceInfo {
    pub name: String,
}

#[post("/createns")]
async fn create_ns(info: web::Json<NamespaceInfo>) -> Result<HttpResponse, ApiError> {
    let info = &info.into_inner().name;
    kube_service::create_ns(info).await?;

    Ok(HttpResponse::Ok().json(json!({ "status": true })))
}

#[post("/deletens")]
async fn delete_ns(info: web::Json<NamespaceInfo>) -> Result<HttpResponse, ApiError> {
    let info = &info.into_inner().name;
    let res = kube_service::delete_ns(info).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": res,
    })))
}

#[get("/deploy")]
async fn get_deploy(info: web::Query<NamespaceInfo>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner().name;
    let results = kube_service::get_deploy_within(&info).await?;
    Ok(HttpResponse::Ok().json(results))
}

#[get("/svc")]
async fn get_svc(info: web::Query<NamespaceInfo>) -> Result<HttpResponse, ApiError> {
    let ns = info.into_inner().name;
    let results = kube_service::get_svc_within(&ns).await?;
    Ok(HttpResponse::Ok().json(results))
}

#[get("/pod")]
async fn get_pod(info: web::Query<NamespaceInfo>) -> Result<HttpResponse, ApiError> {
    let ns = info.into_inner().name;
    let results = kube_service::get_pod_within(&ns).await?;
    Ok(HttpResponse::Ok().json(results))
}

pub fn kube_test_scope() -> Scope {
    web::scope("/kubetest")
        // TODO: guard identity
        // .guard()
        .service(get_nodes_info)
        .service(create_ns)
        .service(delete_ns)
        .service(get_deploy)
        .service(get_svc)
        .service(get_pod)
}
