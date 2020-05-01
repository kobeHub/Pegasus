#[derive(Deserialize)]
pub struct MasterRefObject {
    pub sha: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct MasterRefResponse {
    #[serde(rename = "ref")]
    pub _ref: String,
    pub node_id: String,
    pub url: String,
    pub object: MasterRefObject,
}

#[derive(Deserialize)]
pub struct Tree {
    pub path: String,
    pub mode: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub sha: String,
    pub size: Option<i32>,
    pub url: String,
}

#[derive(Deserialize)]
pub struct DirectoryTreeResponse {
    pub sha: String,
    pub url: String,
    pub tree: Vec<Tree>,
}
