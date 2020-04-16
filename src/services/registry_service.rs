use reqwest::Client;
use serde_json::json;

use crate::errors::ApiError;
use crate::utils::ENGINE_API;
use crate::models::registry::{RepoResponse, RepoCreateInfo};
use crate::models::repository::{Repository};

lazy_static::lazy_static!{
    pub static ref CLIENT: Client = Client::new();
}

/// Get Repo info from pagesus-engine API call
pub async fn get_repo(name: &str) -> Result<RepoResponse, ApiError> {
    let data: RepoResponse = CLIENT.clone()
        .post(format!("{}/repo/getRepo", ENGINE_API.clone()).as_str())
        .header("User-Agent", "Pegasus Axtic-web client")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&json!({
            "name": name,
        }))
        .send()
        .await?
        .json::<RepoResponse>()
        .await?;
    Ok(data)
}
/*
pub async fn create_repo(info: RepoCreateInfo) -> Result<i64, ApiError> {
    let data = CLIENT.clone()
        .post(format!("{}/repo/createRepo", ENGINE_API.clone()).as_str())
        .header("User-Agent", "Pegasus Axtic-web client")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&json!({
            "name": info.name,
            "summary": info.summary,
            "isOverSea": info.is_over_sea,
            "disabelCache": info.disable_cache,
        }))
        .send()
        .await?
        .json()
        .await?;

}
*/
