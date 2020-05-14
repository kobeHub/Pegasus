use actix_web::{web, get, post, Scope, HttpResponse};
use serde_json::json;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::services::kube_service;
use crate::models::Ingress::IngressInfo;

#[derive(Deserialize)]
struct GetInfo {
    pub uid: Uuid,
}

#[derive(Deserialize)]
struct NameInfo {
    pub name: String,
}

#[get("/all")]
async fn get_ings_belong_to(info: web::Query<GetInfo>) -> Result<HttpResponse, ApiError> {

}

pub fn ing_scope() -> Scope {
    web::scope("/ing")

}
