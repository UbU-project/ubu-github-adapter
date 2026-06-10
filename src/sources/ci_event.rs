use serde::{Deserialize, Serialize};
use ubu_core::UbuTimestamp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHubCiStatus {
    Queued,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHubCiConclusion {
    Success,
    Failure,
    Cancelled,
    Skipped,
    Neutral,
    TimedOut,
    ActionRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubCiEventSource {
    pub repository: String,
    pub run_id: u64,
    pub workflow_name: String,
    pub status: GitHubCiStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<GitHubCiConclusion>,
    pub html_url: String,
    pub updated_at: UbuTimestamp,
}
