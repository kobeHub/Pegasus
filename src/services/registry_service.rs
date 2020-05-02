use reqwest::{Client, RequestBuilder};
use serde::Serialize;
use serde_json::json;

use super::git_service;
use crate::errors::ApiError;
use crate::models::registry::{
    RepoBuildRule, RepoCreateInfo, RepoResponse, RulesResponse, TagsResponse,
};
use crate::models::tag::Tag;
use crate::utils::ENGINE_API;

lazy_static::lazy_static! {
    pub static ref CLIENT: Client = Client::new();
}

/// Get Repo info from pagesus-engine API call
pub async fn get_repo(name: &str) -> Result<RepoResponse, ApiError> {
    let data: RepoResponse = build_request(
        format!("{}/repo/getRepo", ENGINE_API.clone()).as_str(),
        &json!({
            "name": name,
        }),
    )
    .send()
    .await?
    .json::<RepoResponse>()
    .await?;
    Ok(data)
}

/// Create a new reppository
pub async fn create_repo(info: RepoCreateInfo, no_record: bool) -> Result<(), ApiError> {
    git_service::create_directory(&info.name, no_record).await?;
    build_request(
        format!("{}/repo/createRepo", ENGINE_API.clone()).as_str(),
        &json!({
            "name": info.name,
            "summary": info.summary,
            "isOverSea": info.is_over_sea,
            "disableCache": info.disable_cache,
        }),
    )
    .send()
    .await?
    .error_for_status()
    .map_err(|err| ApiError::new(500, format!("Pegasus-engine error: {}", err)))?;

    Ok(())
}

/// Delete a repository
pub async fn delete_repo(repo_name: &str) -> Result<(), ApiError> {
    build_request(
        format!("{}/repo/deleteRepo", ENGINE_API.clone()).as_str(),
        &json!({
            "name": repo_name,
        }),
    )
    .send()
    .await?
    .error_for_status()
    .map_err(|err| ApiError::new(500, format!("Pegasus-engine error: {}", err.to_string())))?;
    Ok(())
}

/// Delete a image
pub async fn delete_image(repo_name: &str, repo_tag: &str) -> Result<(), ApiError> {
    build_request(
        format!("{}/repo/deleteImage", ENGINE_API.clone()).as_str(),
        &json!({
            "repoName": repo_name,
            "tag": repo_tag,
        }),
    )
    .send()
    .await?
    .error_for_status()
    .map_err(|err| ApiError::new(500, format!("Pegasus-engine error: {}", err.to_string())))?;

    Ok(())
}

/// Create build rules
pub async fn create_build_rule(info: RepoBuildRule, no_record: bool) -> Result<(), ApiError> {
    let contents = base64::encode(info.dockerfile.as_bytes());
    git_service::create_file(&info.repo_name, &info.tag, &contents, no_record).await?;
    build_request(
        format!("{}/repo/createRepoRule", ENGINE_API.clone()).as_str(),
        &json!({
            "repoName": info.repo_name,
            "location": format!("/{}/{}", &info.repo_name, &info.tag),
            "tag": info.tag,
        }),
    )
    .send()
    .await?
    .error_for_status()
    .map_err(|err| ApiError::new(500, format!("Pegasus-engine error: {}", err.to_string())))?;

    // After GitHub API, Aliyun API successfully, write db
    if no_record {
        Tag::create(&info.repo_name, &info.tag)?;
    } else {
        Tag::recreate(&info.repo_name, &info.tag)?;
    }

    Ok(())
}

/// Delete build tule
pub async fn delete_build_rule(
    repo_name: &str,
    tag_name: &str,
    build_rule_id: &str,
) -> Result<(), ApiError> {
    build_request(
        format!("{}/repo/deleteRepoBuildRule", ENGINE_API.clone()).as_str(),
        &json!({
            "repoName": repo_name,
            "buildRuleId": build_rule_id,
        }),
    )
    .send()
    .await?
    .error_for_status()
    .map_err(|err| ApiError::new(500, format!("Pegasus-engine error: {}", err.to_string())))?;
    Tag::delete(repo_name, tag_name)?;

    Ok(())
}

// Get `repo_name`'s `page_num` build rules
pub async fn get_build_rules(repo_name: &str) -> Result<RulesResponse, ApiError> {
    let data: RulesResponse = build_request(
        format!("{}/repo/getRepoBuildRule", ENGINE_API.clone()).as_str(),
        &json!({
            "name": repo_name,
        }),
    )
    .send()
    .await?
    .json::<RulesResponse>()
    .await?;
    Ok(data)
}

// Get image tags
pub async fn get_repo_tags(repo_name: &str, page: &str) -> Result<TagsResponse, ApiError> {
    let data: TagsResponse = build_request(
        format!("{}/repo/getRepoTags", ENGINE_API.clone()).as_str(),
        &json!({
            "name": repo_name,
            "page": page,
        }),
    )
    .send()
    .await?
    .json::<TagsResponse>()
    .await?;
    Ok(data)
}

pub async fn start_build_rule(name: &str, rule_id: &str) -> Result<(), ApiError> {
    build_request(
        format!("{}/repo/startRepoBuild", ENGINE_API.clone()).as_str(),
        &json!({
            "repoName": name,
            "buildRuleId": rule_id,
        }),
    )
    .send()
    .await?
    .error_for_status()
    .map_err(|err| ApiError::new(500, format!("Pegasus-engine error: {}", err.to_string())))?;
    Ok(())
}

fn build_request<T: Serialize + ?Sized>(path: &str, json_param: &T) -> RequestBuilder {
    CLIENT
        .clone()
        .post(path)
        .header("User-Agent", "Pegasus Axtic-web client")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(json_param)
}
