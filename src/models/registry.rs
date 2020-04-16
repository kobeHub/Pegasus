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

#[derive(Serialize, Deserialize)]
pub struct RepoCreateInfo {
    // For API call
    pub name: String,
    pub summary: String,
    #[serde(rename = "isOverSea")]
    pub is_over_sea: bool,
    #[serde(rename = "disabelCache")]
    pub disable_cache: bool,
    // For database usage
    pub is_public: bool,
    pub belong_to: Option<Uuid>,
}
