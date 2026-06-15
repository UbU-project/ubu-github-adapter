use serde::{Deserialize, Serialize};
use ubu_core::UbuTimestamp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHubReviewState {
    Approved,
    ChangesRequested,
    Commented,
    Dismissed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GitHubReviewSource {
    pub repository: String,
    pub id: u64,
    pub pull_request_number: u64,
    pub state: GitHubReviewState,
    pub html_url: String,
    pub submitted_at: UbuTimestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}
