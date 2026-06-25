use std::sync::Arc;

use ubu_core::AuthoritySource;
use ubu_github_adapter::client::{GitHubClient, RecordedGitHubOperation, RecordingGitHubApi};
use ubu_github_adapter::errors::AdapterError;
use ubu_github_adapter::projection::label_write::{
    apply_managed_label_write, read_managed_label_observation, GitHubLabelWrite,
};
use ubu_github_adapter::sources::GitHubRepositorySource;

#[tokio::test]
async fn managed_label_write_applies_requested_managed_labels() {
    let api = Arc::new(recording_api_with_issue_labels(std::iter::empty::<&str>()));
    let client = GitHubClient::from_api(api.clone());
    let payload = GitHubLabelWrite {
        repository: repository(),
        issue_number: 7,
        labels: vec!["ubu".to_owned(), "ubu-managed".to_owned()],
    };

    let result = apply_managed_label_write(&client, &payload, AuthoritySource::AutomationWorker)
        .await
        .unwrap();

    assert_eq!(
        api.issue_labels("UbU-project", "ubu-github-adapter", 7),
        ["ubu", "ubu-managed"]
    );
    assert_eq!(
        api.recorded_operations(),
        [RecordedGitHubOperation::AddLabelsToIssue {
            owner: "UbU-project".to_owned(),
            repo: "ubu-github-adapter".to_owned(),
            issue_number: 7,
            labels: vec!["ubu".to_owned(), "ubu-managed".to_owned()],
        }]
    );
    assert_eq!(result.applied_labels, ["ubu", "ubu-managed"]);
    assert_eq!(
        result.provenance.authority_source,
        AuthoritySource::AutomationWorker
    );
    assert_eq!(
        result.provenance.source.as_ref().unwrap().source_kind,
        "github_issue"
    );
    assert_eq!(
        result.provenance.source.as_ref().unwrap().source_id,
        "UbU-project/ubu-github-adapter#7"
    );
}

#[tokio::test]
async fn managed_label_write_rejects_out_of_set_labels() {
    let api = Arc::new(recording_api_with_issue_labels(["adapter"]));
    let client = GitHubClient::from_api(api.clone());
    let payload = GitHubLabelWrite {
        repository: repository(),
        issue_number: 7,
        labels: vec!["adapter".to_owned()],
    };

    let error = apply_managed_label_write(&client, &payload, AuthoritySource::AutomationWorker)
        .await
        .unwrap_err();

    assert!(matches!(
        error,
        AdapterError::UnmanagedLabelWrite { label } if label == "adapter"
    ));
    assert_eq!(
        api.issue_labels("UbU-project", "ubu-github-adapter", 7),
        ["adapter"]
    );
    assert!(api.recorded_operations().is_empty());
}

#[tokio::test]
async fn managed_label_write_preserves_existing_non_managed_labels() {
    let api = Arc::new(recording_api_with_issue_labels(["adapter"]));
    let client = GitHubClient::from_api(api.clone());
    let payload = GitHubLabelWrite {
        repository: repository(),
        issue_number: 7,
        labels: vec!["ubu".to_owned()],
    };

    apply_managed_label_write(&client, &payload, AuthoritySource::AutomationWorker)
        .await
        .unwrap();

    assert_eq!(
        api.issue_labels("UbU-project", "ubu-github-adapter", 7),
        ["adapter", "ubu"]
    );
}

#[tokio::test]
async fn reconciliation_read_returns_observed_managed_label_state() {
    let api = Arc::new(recording_api_with_issue_labels([
        "adapter",
        "ubu-managed",
        "triage",
        "ubu",
    ]));
    let client = GitHubClient::recording(api.clone());

    let observation = read_managed_label_observation(&client, &repository(), 7)
        .await
        .unwrap();

    assert_eq!(observation.labels, ["ubu", "ubu-managed"]);
    assert_eq!(observation.source.source_kind, "github_issue");
    assert_eq!(
        observation.source.source_id,
        "UbU-project/ubu-github-adapter#7"
    );
    assert_eq!(
        observation.provenance.authority_source,
        AuthoritySource::System
    );
    assert_eq!(
        observation.provenance.source.as_ref().unwrap(),
        &observation.source
    );
    assert_eq!(
        api.recorded_operations(),
        [RecordedGitHubOperation::ReadIssueLabels {
            owner: "UbU-project".to_owned(),
            repo: "ubu-github-adapter".to_owned(),
            issue_number: 7,
        }]
    );
}

fn recording_api_with_issue_labels<I, S>(labels: I) -> RecordingGitHubApi
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    RecordingGitHubApi::with_issue_labels("UbU-project", "ubu-github-adapter", 7, labels)
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
