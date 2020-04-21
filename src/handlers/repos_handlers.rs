use actix_web::{delete, get, post, web, HttpResponse, Scope};
use serde_json::json;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::models::repository::{Repository, DeleteInfo, PageInfo};
use crate::models::registry::{RepoCreateInfo, RepoBuildRule, RuleStartInfo};
use crate::services::{git_service, registry_service};

#[post("/create")]
async fn create_repo(info: web::Json<RepoCreateInfo>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();

    registry_service::create_repo(info.clone()).await?;
    let res = Repository::create(info.belong_to.as_ref(), &info.name, info.is_over_sea)?;
    Ok(HttpResponse::Ok().json(res))
}

type GetInfo = DeleteInfo;

#[get("/repo")]
async fn get_repo(info: web::Query<GetInfo>) -> Result<HttpResponse, ApiError> {
    let res = registry_service::get_repo(&info.repo_name).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[delete("/repo")]
async fn delete_repo(info: web::Json<DeleteInfo>) -> Result<HttpResponse, ApiError> {
    registry_service::delete_repo(&info.repo_name).await?;
    Repository::delete(&info.repo_name)?;
    Ok(HttpResponse::Ok().json(json!({
        "status": true,
        "msg": format!("Repository {} deleted", &info.repo_name),
    })))
}

// build rules handlers
#[post("/rule")]
async fn create_build_rule(info: web::Json<RepoBuildRule>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();
    registry_service::create_build_rule(info).await?;
    Ok(HttpResponse::Ok().json(json!({
        "status": true,
        "msg": "Build rule create successfully",
    })))
}

#[get("/rules")]
async fn get_build_rules(info: web::Query<GetInfo>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();
    let res = registry_service::get_build_rules(&info.repo_name).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[post("/startbuild")]
async fn start_build_rule(info: web::Json<RuleStartInfo>) -> Result<HttpResponse, ApiError> {
    registry_service::start_build_rule(&info.repo_name, &info.build_rule_id).await?;
    Ok(HttpResponse::Ok().json(json!({
        "status": true,
        "msg": "Start build rule successfully",
    })))
}

pub fn repos_scope() -> Scope {
    web::scope("/repos")
        .service(create_repo)
        .service(delete_repo)
        .service(get_repo)
        .service(delete_repo)
        .service(create_build_rule)
        .service(get_build_rules)
        .service(start_build_rule)
}
