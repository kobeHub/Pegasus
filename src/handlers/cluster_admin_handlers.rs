use actix_web::{web, get, post, HttpResponse, Scope};
use serde_json::json;

use crate::errors::ApiError;
use crate::services::kube_service;

#[get("/nodes")]
async fn get_nodes_info() -> Result<HttpResponse, ApiError> {
    let res = kube_service::get_nodes().await?;
    Ok(HttpResponse::Ok().json(json!({
        "data": res
    })))
}

#[derive(Deserialize)]
struct NamespaceInfo {
    pub name: String
}

#[post("/namespace")]
async fn create_ns(info: web::Json<NamespaceInfo>) -> Result<HttpResponse, ApiError> {
    let info = &info.into_inner().name;
    let res: String = kube_service::create_ns(info).await?;

    Ok(HttpResponse::Ok().json(json!({
        "data": res
    })))
}

pub fn cluster_admin_scope() -> Scope {
    web::scope("/clusteradmin")
    // TODO: guard identity
        // .guard()
        .service(get_nodes_info)
        .service(create_ns)
}
