use serde::{Deserialize, Serialize};
use ubu_core::projection::result::ProjectionResult;
use ubu_core::AuthoritySource;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GitHubOperationResult {
    pub operation_id: String,
    pub applied: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GitHubProjectionResult {
    pub core_result: ProjectionResult,
    pub authority_source: AuthoritySource,
    pub github_results: Vec<GitHubOperationResult>,
}
