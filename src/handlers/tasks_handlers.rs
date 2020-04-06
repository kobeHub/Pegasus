use actix_web::{web, get, post, HttpResponse, Scope};
use serde_json::json;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::service::kube_service;

#{derive(Seralize)}
struct NSInfo {
    id: Uuid,
}

#[get("/ns")]
async fn get_ns(info: web::Query<NSInfo>) -> Result<HttpResponse, ApiError> {
    let uid = info.into_inner().id;

}


pub fn tasks_scope() -> Scope {
    web::scope("/tasks")
}
