use serde::{Deserialize, Serialize};

use crate::fixture::GitHubFixture;
use crate::sources::{
    GitHubCiEventSource, GitHubCommentSource, GitHubIssueSource, GitHubLabelSource,
    GitHubMilestoneSource, GitHubPullRequestSource, GitHubRepositorySource, GitHubReviewSource,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalizedRepositoryState {
    pub repository: GitHubRepositorySource,
    pub issues: Vec<GitHubIssueSource>,
    pub pull_requests: Vec<GitHubPullRequestSource>,
    pub reviews: Vec<GitHubReviewSource>,
    pub ci_events: Vec<GitHubCiEventSource>,
    pub labels: Vec<GitHubLabelSource>,
    pub milestones: Vec<GitHubMilestoneSource>,
    pub comments: Vec<GitHubCommentSource>,
}

pub fn normalize_fixture(fixture: GitHubFixture) -> NormalizedRepositoryState {
    NormalizedRepositoryState {
        repository: fixture.repository,
        issues: fixture.issues,
        pull_requests: fixture.pull_requests,
        reviews: fixture.reviews,
        ci_events: fixture.ci_events,
        labels: fixture.labels,
        milestones: fixture.milestones,
        comments: fixture.comments,
    }
}
