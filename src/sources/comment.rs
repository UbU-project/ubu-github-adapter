use serde::{Deserialize, Serialize};
use ubu_core::UbuTimestamp;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubCommentSource {
    pub repository: String,
    pub id: u64,
    pub issue_number: u64,
    pub body: String,
    pub html_url: String,
    pub updated_at: UbuTimestamp,
}
