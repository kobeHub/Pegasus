use uuid::Uuid;

/// Models defines here used for API call
/// data models
#[derive(Serialize, Deserialize)]
pub struct Repo {
    pub id: i64,
    pub name: String,
    pub summary: String,
    pub status: String,
    pub downloads: i32,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct RepoResponse {
    pub status: String,
    pub data: Repo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RepoCreateInfo {
    // For API call
    pub name: String,
    pub summary: String,
    #[serde(rename = "isOverSea")]
    pub is_over_sea: bool,
    #[serde(rename = "disableCache")]
    pub disable_cache: bool,
    // For database usage
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    pub belong_to: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct CreateResponse {
    pub status: String,
    pub data: i64,
}

/// RepoBuildRule data model
#[derive(Serialize, Deserialize)]
pub struct RepoBuildRule {
    #[serde(rename = "repoName")]
    pub repo_name: String,
    pub tag: String,
    pub dockerfile: String, // Dockerfile content
}

#[derive(Serialize, Deserialize)]
pub struct RuleItem {
    #[serde(rename = "buildRuleId")]
    pub build_rule_id: String,
    #[serde(rename = "imageTag")]
    pub image_tag: String,
}

#[derive(Serialize, Deserialize)]
pub struct RulesResponse {
    pub status: String,
    pub data: Vec<RuleItem>,
}

#[derive(Deserialize)]
pub struct RuleStartInfo {
    #[serde(rename = "repoName")]
    pub repo_name: String,
    #[serde(rename = "buildRuleId")]
    pub build_rule_id: String,
}

#[derive(Deserialize)]
pub struct RuleDeleteInfo {
    #[serde(rename = "repoName")]
    pub repo_name: String,
    #[serde(rename = "buildRuleId")]
    pub build_rule_id: String,
    pub tag: String,
}

/// Image tags
#[derive(Deserialize, Serialize)]
pub struct TagsResponse {
    pub status: String,
    pub data: TagsObject,
}

#[derive(Deserialize, Serialize)]
pub struct TagsObject {
    pub page: i32,
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    pub total: i32,
    pub tags: Vec<TagItem>,
}

#[derive(Deserialize, Serialize)]
pub struct TagItem {
    #[serde(rename = "imageId")]
    pub image_id: String,
    pub tag: String,
    pub size: i32,
    pub status: String,
    pub digest: String,
}
