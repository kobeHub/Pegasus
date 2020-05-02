use actix_web::{delete, get, post, web, HttpResponse, Scope};
use serde_json::json;
use uuid::Uuid;

use std::collections::BTreeMap;

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
struct GetInfo {
    pub id: Uuid,
}

#[get("belong")]
async fn get_ns_belong(info: web::Query<GetInfo>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();

    let res = Namespace::get_ns_of(&info.id)?;
    Ok(HttpResponse::Ok().json(res))
}

#[get("/labels")]
async fn get_app_labels(info: web::Query<GetInfo>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();
    let nss = Namespace::get_ns_of(&info.id)?;
    let mut results: BTreeMap<String, Vec<Option<String>>> = BTreeMap::new();
    for ns in nss.iter() {
        let labels = kube_service::get_label_map(ns).await?;
        results.insert(ns.to_string(), labels);
    }
    Ok(HttpResponse::Ok().json(results))
}

pub fn ns_scope() -> Scope {
    web::scope("/ns")
        .service(create_ns)
        .service(delete_ns)
        .service(get_ns_belong)
        .service(get_app_labels)
}
