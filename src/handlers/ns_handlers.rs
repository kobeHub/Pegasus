use actix_web::{post, delete, web, HttpResponse, Scope};
use serde_json::json;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::models::namespace::{Namespace, NamespaceInfo};
use crate::services::kube_service;

#[post("/create")]
async fn create_ns(info: web::Json<NamespaceInfo>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();
    kube_service::create_ns(&info.ns).await?;
    let ns = Namespace::create(info)?;

    Ok(HttpResponse::Ok().json(ns))
}

#[derive(Deserialize)]
struct DeleteInfo {
    pub uid: Uuid,
    pub namespace: String,
}

#[delete("/delete")]
async fn delete_ns(info: web::Json<DeleteInfo>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();

    let res = kube_service::delete_ns(&info.namespace).await?;
    Namespace::delete(&info.uid, &info.namespace)?;

    Ok(HttpResponse::Ok().json(json!({
        "msg": res,
    })))
}

#[derive(Deserialize)]
struct NSInfo {
    pub id: Uuid,
}

pub fn ns_scope() -> Scope {
    web::scope("/ns")
        .service(create_ns)
        .service(delete_ns)
}
