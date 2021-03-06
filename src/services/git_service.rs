use reqwest::{Client, RequestBuilder};
use serde::Serialize;
use serde_json::json;
use std::time::Duration;

use crate::errors::ApiError;
use crate::models::gitapis::{DirectoryTreeResponse, MasterRefResponse};
use crate::utils::{GITHUB_AK, GITHUB_API, GITHUB_OWNER, GITHUB_REPO};

pub async fn create_directory(dirname: &str, no_record: bool) -> Result<(), ApiError> {
    if !no_record {
        return Ok(());
    }
    build_put(
        format!(
            "{}/repos/{}/{}/contents/{}/init.txt",
            GITHUB_API.as_str(),
            GITHUB_OWNER.as_str(),
            GITHUB_REPO.as_str(),
            dirname
        )
        .as_str(),
        &json!({
            "message": format!("Add new image repo {}", dirname),
            "content": "aW5pdAo=",
        }),
    )
    .send()
    .await?
    .error_for_status()
    .map_err(|err| ApiError::new(500, format!("Pegasus-engine error: {}", err.to_string())))?;
    Ok(())
}

// Write a Dockerfile with base64 contents
pub async fn create_file(
    repo_name: &str,
    tag_name: &str,
    contents_base64: &str,
    no_record: bool,
) -> Result<(), ApiError> {
    let json_param = if no_record {
        json!({
            "message": format!("Add new image tag {} dockerfile", tag_name),
            "content": contents_base64,
        })
    } else {
        let sha = get_image_sha(repo_name, tag_name).await?;
        json!({
            "message": format!("Add new image tag {} dockerfile", tag_name),
            "content": contents_base64,
            "sha": sha,
        })
    };
    build_put(
        format!(
            "{}/repos/{}/{}/contents/{}/{}/Dockerfile",
            GITHUB_API.as_str(),
            GITHUB_OWNER.as_str(),
            GITHUB_REPO.as_str(),
            repo_name,
            tag_name
        )
        .as_str(),
        &json_param,
    )
    .send()
    .await?
    .error_for_status()
    .map_err(|err| ApiError::new(500, format!("Pegasus-engine error: {}", err.to_string())))?;

    Ok(())
}

/*
// Delete a Dockerfile and repository
#[allow(dead_code)]
pub async fn delete_image(repo_name: &str, tag_name: &str) -> Result<(), ApiError> {
    let sha = get_image_sha(repo_name, tag_name).await?;
    build_delete(
        format!("{}/repos/{}/{}/contents/{}/{}", GITHUB_API.as_str(), GITHUB_OWNER.as_str(), GITHUB_REPO.as_str(), repo_name, tag_name).as_str(),
        &json!({
            "message": format!("delete image {}:{}", repo_name, tag_name),
            "sha": sha,
        })
    )
    .send()
    .await?
    .error_for_status()
    .map_err(|err| ApiError::new(500, format!("Pegasus-engine error: {}", err.to_string())))?;
    Ok(())
}

#[allow(dead_code)]
pub async fn delete_repo(repo_name: &str) -> Result<(), ApiError> {
    let sha = get_repo_sha(repo_name).await?;
    build_delete(
        format!("{}/repos/{}/{}/contents/{}", GITHUB_API.as_str(), GITHUB_OWNER.as_str(), GITHUB_REPO.as_str(), repo_name).as_str(),
        &json!({
            "message": format!("delete image repo {}", repo_name),
            "sha": sha,
        })
    )
        .send()
        .await?
        .error_for_status()
        .map_err(|err| ApiError::new(500, format!("Pegasus-engine error: {}", err.to_string())))?;
    Ok(())
}*/

// Get master sha
async fn get_repo_sha(repo_name: &str) -> Result<String, ApiError> {
    let root_sha: String = build_get(
        format!(
            "{}/repos/{}/{}/git/ref/heads/master",
            GITHUB_API.as_str(),
            GITHUB_OWNER.as_str(),
            GITHUB_REPO.as_str()
        )
        .as_str(),
    )
    .send()
    .await?
    .error_for_status()
    .map_err(|err| ApiError::new(500, format!("Pegasus-engine error: {}", err.to_string())))?
    .json::<MasterRefResponse>()
    .await?
    .object
    .sha;

    let repo_directory_sha: Vec<String> = build_get(
        format!(
            "{}/repos/{}/{}/git/trees/{}",
            GITHUB_API.as_str(),
            GITHUB_OWNER.as_str(),
            GITHUB_REPO.as_str(),
            &root_sha
        )
        .as_str(),
    )
    .send()
    .await?
    .error_for_status()
    .map_err(|err| ApiError::new(500, format!("Pegasus-engine error: {}", err.to_string())))?
    .json::<DirectoryTreeResponse>()
    .await?
    .tree
    .iter()
    .filter(|x| x._type.as_str() == "tree" && x.path.as_str() == repo_name)
    .map(|x| x.sha.clone())
    .collect();

    if repo_directory_sha.len() != 1usize {
        return Ok("".to_string());
    }
    Ok(repo_directory_sha[0].clone())
}

pub async fn get_image_sha(repo_name: &str, tag_name: &str) -> Result<String, ApiError> {
    let repo_sha = get_repo_sha(repo_name).await?;

    let res: Vec<String> = build_get(
        format!(
            "{}/repos/{}/{}/git/trees/{}",
            GITHUB_API.as_str(),
            GITHUB_OWNER.as_str(),
            GITHUB_REPO.as_str(),
            repo_sha
        )
        .as_str(),
    )
    .send()
    .await?
    .error_for_status()
    .map_err(|err| ApiError::new(500, format!("Pegasus-engine error: {}", err.to_string())))?
    .json::<DirectoryTreeResponse>()
    .await?
    .tree
    .iter()
    .filter(|x| x._type.as_str() == "tree" && x.path.as_str() == tag_name)
    .map(|x| x.sha.clone())
    .collect();
    if res.len() != 1usize {
        return Ok("".to_string());
    }
    let sha = build_get(
        format!(
            "{}/repos/{}/{}/git/trees/{}",
            GITHUB_API.as_str(),
            GITHUB_OWNER.as_str(),
            GITHUB_REPO.as_str(),
            res[0]
        )
        .as_str(),
    )
    .send()
    .await?
    .error_for_status()
    .map_err(|err| ApiError::new(500, format!("Pegasus-engine error: {}", err.to_string())))?
    .json::<DirectoryTreeResponse>()
    .await?
    .tree[0]
        .sha
        .clone();

    Ok(sha)
}

// build requests : long time request
fn build_get(path: &str) -> RequestBuilder {
    Client::new()
        .get(path)
        .header("User-Agent", GITHUB_OWNER.as_str())
        .header("Authorization", format!("token {}", GITHUB_AK.as_str()))
        .timeout(Duration::new(15, 0))
}

fn build_put<T: Serialize + ?Sized>(path: &str, json_param: &T) -> RequestBuilder {
    Client::new()
        .put(path)
        .header("User-Agent", GITHUB_OWNER.as_str())
        .header("Authorization", format!("token {}", GITHUB_AK.as_str()))
        .json(json_param)
}

/*
fn build_delete<T: Serialize + ?Sized>(path: &str, json_param: &T) -> RequestBuilder {
    Client::new()
        .delete(path)
        .header("User-Agent", GITHUB_OWNER.as_str())
        .header("Authorization", format!("token {}", GITHUB_AK.as_str()))
        .json(json_param)
}
*/
