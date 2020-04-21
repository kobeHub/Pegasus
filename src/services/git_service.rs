use reqwest::{Client, RequestBuilder};
use serde_json::json;
use serde::Serialize;

use crate::errors::ApiError;
use crate::utils::{GITHUB_API, GITHUB_OWNER, GITHUB_REPO, GITHUB_AK};

pub async fn create_directory(dirname: &str) -> Result<(), ApiError> {
    build_put(
        format!("{}/repos/{}/{}/contents/{}/init.txt", GITHUB_API.as_str(), GITHUB_OWNER.as_str(), GITHUB_REPO.as_str(), dirname).as_str(),
        &json!({
            "message": format!("Add new image repo {}", dirname),
            "content": "aW5pdAo=",
        }))
        .send()
        .await?
        .error_for_status()
        .map_err(|err| ApiError::new(500, format!("Pegasus-engine error: {}", err.to_string())))?;
        Ok(())
}

// Write a Dockerfile with base64 contents
pub async fn create_file(repo_name: &str, tag_name: &str, contents_base64: &str) -> Result<(), ApiError> {
    build_put(
        format!("{}/repos/{}/{}/contents/{}/{}/Dockerfile", GITHUB_API.as_str(), GITHUB_OWNER.as_str(), GITHUB_REPO.as_str(), repo_name, tag_name).as_str(),
        &json!({
            "message": format!("Add new image tag {} dockerfile", tag_name),
            "content": contents_base64,
        }))
        .send()
        .await?
        .error_for_status()
        .map_err(|err| ApiError::new(500, format!("Pegasus-engine error: {}", err.to_string())))?;
    Ok(())
}

// TODO: delete api

// build requests
fn build_put<T: Serialize + ?Sized>(path: &str, json_param: &T) -> RequestBuilder {
    Client::new()
        .put(path)
        .header("User-Agent", GITHUB_OWNER.as_str())
        .header("Authorization", format!("token {}", GITHUB_AK.as_str()))
        .json(json_param)
}

fn build_delete<T: Serialize + ?Sized>(path: &str, json_param: &T) -> RequestBuilder {
    Client::new()
        .delete(path)
        .header("User-Agent", GITHUB_OWNER.as_str())
        .header("Authorization", format!("token {}", GITHUB_AK.as_str()))
        .json(json_param)
}
