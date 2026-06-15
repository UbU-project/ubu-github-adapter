use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GitHubRepositorySource {
    pub owner: String,
    pub name: String,
    pub default_branch: String,
    pub html_url: String,
    pub api_id: u64,
}

impl GitHubRepositorySource {
    pub fn full_name(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}
