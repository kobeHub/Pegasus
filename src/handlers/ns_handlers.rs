use actix_web::{get, post, web, HttpResponse, Scope};
use serde_json::json;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::models::namespace::{Namespace, NamespaceInfo};
use crate::models::user::User;
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
    pub id: i32,
}

#[post("/delete")]
async fn delete_ns(info: web::Json<DeleteInfo>) -> Result<HttpResponse, ApiError> {
    let id = info.into_inner().id;
    let ns_name = Namespace::delete(id)?;

    // TODO
    let res = kube_service::delete_ns(&ns_name).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": res,
        "msg": format!("Delete namespace {} successfully", ns_name),
    })))
}

#[derive(Deserialize)]
struct NSInfo {
    pub id: Uuid,
}

pub fn ns_scope() -> Scope {
    web::scope("/ns").service(create_ns).service(delete_ns)
}
