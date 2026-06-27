use std::sync::Arc;

use ubu_core::UbuTimestamp;
use ubu_github_adapter::candidate_mapping::map_repository_state;
use ubu_github_adapter::cli::import_live::import_live_repository;
use ubu_github_adapter::client::{GitHubClient, RecordingGitHubApi};
use ubu_github_adapter::sources::{GitHubIssueSource, GitHubIssueState, GitHubRepositorySource};

#[tokio::test]
async fn imports_live_repository_issues_from_recording_api() {
    let api = Arc::new(RecordingGitHubApi::with_repository(repository()));
    api.seed_issues("UbU-project", "ubu-github-adapter", [issue()]);
    let client = GitHubClient::recording(api);

    let normalized = import_live_repository(&client, "UbU-project", "ubu-github-adapter")
        .await
        .unwrap();

    assert_eq!(
        normalized.repository.full_name(),
        "UbU-project/ubu-github-adapter"
    );
    assert_eq!(normalized.issues.len(), 1);
    assert!(normalized.pull_requests.is_empty());
    assert!(normalized.reviews.is_empty());
    assert!(normalized.ci_events.is_empty());
    assert!(normalized.labels.is_empty());
    assert!(normalized.milestones.is_empty());
    assert!(normalized.comments.is_empty());

    let mapped = map_repository_state(&normalized).unwrap();
    assert_eq!(mapped.tasks.len(), 1);
    assert_eq!(
        mapped.external_references[0].source.source_id,
        "UbU-project/ubu-github-adapter#7"
    );
}

fn repository() -> GitHubRepositorySource {
    GitHubRepositorySource {
        owner: "UbU-project".to_owned(),
        name: "ubu-github-adapter".to_owned(),
        default_branch: "main".to_owned(),
        html_url: "https://github.com/UbU-project/ubu-github-adapter".to_owned(),
        api_id: 1001,
    }
}

fn issue() -> GitHubIssueSource {
    GitHubIssueSource {
        repository: "UbU-project/ubu-github-adapter".to_owned(),
        id: 2001,
        number: 7,
        title: "Import GitHub issues".to_owned(),
        body: Some("Normalize issue state into UbU candidates.".to_owned()),
        state: GitHubIssueState::Open,
        html_url: "https://github.com/UbU-project/ubu-github-adapter/issues/7".to_owned(),
        updated_at: UbuTimestamp::parse("2026-06-10T14:30:00Z").unwrap(),
        labels: vec!["adapter".to_owned()],
        milestone: None,
    }
}
