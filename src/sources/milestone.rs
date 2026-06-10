use serde::{Deserialize, Serialize};
use ubu_core::UbuTimestamp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHubMilestoneState {
    Open,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubMilestoneSource {
    pub repository: String,
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub state: GitHubMilestoneState,
    pub html_url: String,
    pub updated_at: UbuTimestamp,
}
