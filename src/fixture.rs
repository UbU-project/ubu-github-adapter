use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::errors::Result;
use crate::sources::{
    GitHubCiEventSource, GitHubCommentSource, GitHubIssueSource, GitHubLabelSource,
    GitHubMilestoneSource, GitHubPullRequestSource, GitHubRepositorySource, GitHubReviewSource,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubFixture {
    pub repository: GitHubRepositorySource,
    #[serde(default)]
    pub issues: Vec<GitHubIssueSource>,
    #[serde(default)]
    pub pull_requests: Vec<GitHubPullRequestSource>,
    #[serde(default)]
    pub reviews: Vec<GitHubReviewSource>,
    #[serde(default)]
    pub ci_events: Vec<GitHubCiEventSource>,
    #[serde(default)]
    pub labels: Vec<GitHubLabelSource>,
    #[serde(default)]
    pub milestones: Vec<GitHubMilestoneSource>,
    #[serde(default)]
    pub comments: Vec<GitHubCommentSource>,
}

impl GitHubFixture {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&contents)?)
    }
}
