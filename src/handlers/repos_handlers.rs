use actix_web::{get, post, web, HttpResponse, Scope};
use serde_json::json;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::models::repository::{Repository, DeleteInfo};
use crate::models::registry::{RepoCreateInfo};
use crate::services::registry_service;

#[post("/create")]
async fn create_repo(info: web::Json<RepoCreateInfo>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();

    let res = Repository::create(info.belong_to.as_ref(), &info.name, info.is_over_sea)?;
    Ok(HttpResponse::Ok().json(res))
}

#[post("/delete")]
async fn delete_repo(info: web::Json<DeleteInfo>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();

    Repository::delete(&info.repo_name)?;
    Ok(HttpResponse::Ok().json(json!({
        "status": true,
        "msg": format!("Delete repo {} successfully", &info.repo_name),
    })))
}

type GetInfo = DeleteInfo;

#[get("/repo")]
async fn get_repo(info: web::Query<GetInfo>) -> Result<HttpResponse, ApiError> {
    let res = registry_service::get_repo(&info.repo_name).await?;
    Ok(HttpResponse::Ok().json(res))
}

pub fn repos_scope() -> Scope {
    web::scope("/repos")
        .service(create_repo)
        .service(delete_repo)
        .service(get_repo)
}
