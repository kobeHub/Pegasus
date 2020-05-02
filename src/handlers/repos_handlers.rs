use actix_web::{delete, get, post, web, HttpResponse, Scope};
use serde_json::json;

use crate::errors::ApiError;
use crate::models::registry::{RepoBuildRule, RepoCreateInfo, RuleDeleteInfo, RuleStartInfo};
use crate::models::repository::{DeleteInfo, ImageInfo, PageInfo, RepoRecordState, Repository};
use crate::models::tag::{Tag, TagRecordState};
use crate::services::registry_service;

#[post("/create")]
async fn create_repo(info: web::Json<RepoCreateInfo>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();
    let state = Repository::record_state(&info.name)?;
    if state == RepoRecordState::Active {
        return Ok(HttpResponse::Ok().json(json!({
            "state": false,
            "msg": "The repository exists already",
            "data": "",
        })));
    }

    let no_record = state == RepoRecordState::NotFound;
    registry_service::create_repo(info.clone(), no_record).await?;
    let res = Repository::create(
        info.belong_to.as_ref(),
        &info.name,
        info.is_public,
        no_record,
    )?;
    Ok(HttpResponse::Ok().json(json!({
        "status": true,
        "msg": "Repository created successfully!",
        "data": res,
    })))
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
    //git_service::delete_repo(&info.repo_name).await?;
    Repository::delete(&info.repo_name)?;
    Ok(HttpResponse::Ok().json(json!({
        "status": true,
        "msg": format!("Repository {} deleted", &info.repo_name),
    })))
}

#[delete("/image")]
async fn delete_image(info: web::Json<ImageInfo>) -> Result<HttpResponse, ApiError> {
    registry_service::delete_image(&info.repo_name, &info.tag).await?;
    Ok(HttpResponse::Ok().json(json!({
        "status": true,
        "msg": format!("Image {}:{} deleted successfully", &info.repo_name, &info.tag)
    })))
}

// build rules handlers
#[post("/rule")]
async fn create_build_rule(info: web::Json<RepoBuildRule>) -> Result<HttpResponse, ApiError> {
    let tag_state = Tag::record_state(&info.repo_name, &info.tag)?;
    if tag_state == TagRecordState::Active {
        return Ok(HttpResponse::Ok().json(json!({
            "status": false,
            "msg": "Build rule exists already",
        })));
    }
    let no_record = tag_state == TagRecordState::NotFound;

    let info = info.into_inner();
    registry_service::create_build_rule(info, no_record).await?;
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

#[get("/tags")]
async fn get_tags(info: web::Query<PageInfo>) -> Result<HttpResponse, ApiError> {
    let response =
        registry_service::get_repo_tags(&info.name, format!("{}", info.page).as_str()).await?;
    Ok(HttpResponse::Ok().json(response))
}

// Delete build rule and set tags file invalid
#[delete("/buildrule")]
async fn delete_build_rule(info: web::Json<RuleDeleteInfo>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();
    registry_service::delete_build_rule(&info.repo_name, &info.tag, &info.build_rule_id).await?;
    Ok(HttpResponse::Ok().json(json!({
        "status": true,
        "msg": "Build rule deleted successfully",
    })))
}

pub fn repos_scope() -> Scope {
    web::scope("/repos")
        .service(create_repo)
        .service(delete_repo)
        .service(get_repo)
        .service(get_tags)
        .service(delete_repo)
        .service(delete_image)
        .service(create_build_rule)
        .service(get_build_rules)
        .service(start_build_rule)
        .service(delete_build_rule)
}
