use reqwest::{Client, RequestBuilder};
use serde_json::json;
use serde::Serialize;

use crate::errors::ApiError;
use crate::utils::ENGINE_API;
use crate::models::registry::{RepoResponse, RepoCreateInfo,
                              CreateResponse, RepoBuildRule, RulesResponse};
use crate::models::repository::{Repository};

lazy_static::lazy_static!{
    pub static ref CLIENT: Client = Client::new();
}

/// Get Repo info from pagesus-engine API call
pub async fn get_repo(name: &str) -> Result<RepoResponse, ApiError> {
    let data: RepoResponse = build_request(
        format!("{}/repo/getRepo", ENGINE_API.clone()).as_str(),
        &json!({
            "name": name,
        }))
        .send()
        .await?
        .json::<RepoResponse>()
        .await?;
    Ok(data)
}

/// Create a new reppository
pub async fn create_repo(info: RepoCreateInfo) -> Result<(), ApiError> {
    build_request(
        format!("{}/repo/createRepo", ENGINE_API.clone()).as_str(),
        &json!({
            "name": info.name,
            "summary": info.summary,
            "isOverSea": info.is_over_sea,
            "disableCache": info.disable_cache,
        }))
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
        }))
        .send()
        .await?
        .error_for_status()
        .map_err(|err| ApiError::new(500, format!("Pegasus-engine error: {}", err.to_string())))?;
    Ok(())
}

/// Create build rules
pub async fn create_build_rule(info: RepoBuildRule) -> Result<(), ApiError> {
    build_request(
        format!("{}/repo/createRepoRule", ENGINE_API.clone()).as_str(),
        &json!({
            "repoName": info.repo_name,
            "location": info.location,
            "tag": info.tag,
        }))
        .send()
        .await?
        .error_for_status()
        .map_err(|err| ApiError::new(500, format!("Pegasus-engine error: {}", err.to_string())))?;
    Ok(())
}

// Get `repo_name`'s `page_num` build rules
pub async fn get_build_rules(repo_name: &str) -> Result<RulesResponse, ApiError> {
    let data: RulesResponse = build_request(
        format!("{}/repo/getRepo", ENGINE_API.clone()).as_str(),
        &json!({
            "name": repo_name,
        }))
        .send()
        .await?
        .json::<RulesResponse>()
        .await?;
    Ok(data)
}

pub async fn start_build_rule(name: &str, rule_id: &str) -> Result<(), ApiError> {
    build_request(
        format!("{}/repo/startRepoBuild", ENGINE_API.clone()).as_str(),
        &json!({
            "repoName": name,
            "buildRuleId": rule_id,
        }))
        .send()
        .await?
        .error_for_status()
        .map_err(|err| ApiError::new(500, format!("Pegasus-engine error: {}", err.to_string())))?;
    Ok(())
}

fn build_request<T: Serialize + ?Sized>(path: &str, json_param: &T) -> RequestBuilder {
    CLIENT.clone()
        .post(path)
        .header("User-Agent", "Pegasus Axtic-web client")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(json_param)
}
