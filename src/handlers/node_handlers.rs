use actix_web:: HttpResponse;
use serde_json::json;

use crate::errors::ApiError;
use crate::services::kube_service;

pub async fn get_node_info() -> Result<HttpResponse, ApiError> {
    let res = kube_service::get_nodes().await?;
    Ok(HttpResponse::Ok().json(json!({
        "data": res
    })))
}
