use actix_web::{web, get, post, delete, HttpResponse, Scope};
use serde_json::json;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::services::kube_service;
use crate::models::ingress::IngressInfo;
use crate::models::kube::DeleteInfo;

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
    let results = kube_service::get_ing_belong(&info.uid).await?;
    Ok(HttpResponse::Ok().json(results))
}

#[post("/create")]
async fn create_ing(info: web::Json<IngressInfo>) -> Result<HttpResponse, ApiError> {
    let result = kube_service::create_ing(&info.into_inner()).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": true,
        "msg": "Ingress create successfully",
        "data": result,
    })))
}

#[get("/svcmap")]
async fn get_svc_map(info: web::Query<GetInfo>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();
    let result = kube_service::get_svc_map(&info.uid).await?;

    Ok(HttpResponse::Ok().json(result))
}

#[delete("/item")]
async fn delete_ing(info: web::Json<DeleteInfo>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();

    let msg = kube_service::delete_ing(&info.namespace, &info.name).await?;
    Ok(HttpResponse::Ok().json(json!({
        "msg": msg,
    })))
}

pub fn ing_scope() -> Scope {
    web::scope("/ing")
        .service(get_ings_belong_to)
        .service(get_svc_map)
        .service(create_ing)
        .service(delete_ing)
}
