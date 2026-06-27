use crate::client::GitHubClient;
use crate::errors::Result;
use crate::normalize::NormalizedRepositoryState;

pub async fn import_live_repository(
    client: &GitHubClient,
    owner: &str,
    repo: &str,
) -> Result<NormalizedRepositoryState> {
    let repository = client.api().repository(owner, repo).await?;
    let issues = client.api().list_issues(owner, repo).await?;

    Ok(NormalizedRepositoryState {
        repository,
        issues,
        pull_requests: Vec::new(),
        reviews: Vec::new(),
        ci_events: Vec::new(),
        labels: Vec::new(),
        milestones: Vec::new(),
        comments: Vec::new(),
    })
}
