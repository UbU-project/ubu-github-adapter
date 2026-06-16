use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use ubu_core::AuthoritySource;
use ubu_github_adapter::client::{GitHubApi, GitHubApiFuture, GitHubClient};
use ubu_github_adapter::errors::AdapterError;
use ubu_github_adapter::projection::label_write::{
    apply_managed_label_write, read_managed_label_observation, GitHubLabelWrite,
};
use ubu_github_adapter::sources::GitHubRepositorySource;

#[tokio::test]
async fn managed_label_write_applies_requested_managed_labels() {
    let api = Arc::new(MockGitHubApi::with_issue_labels(&[]));
    let client = GitHubClient::from_api(api.clone());
    let payload = GitHubLabelWrite {
        repository: repository(),
        issue_number: 7,
        labels: vec!["ubu".to_owned(), "ubu-managed".to_owned()],
    };

    let result = apply_managed_label_write(&client, &payload, AuthoritySource::AutomationWorker)
        .await
        .unwrap();

    assert_eq!(api.labels_for_issue(7), ["ubu", "ubu-managed"]);
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
    let api = Arc::new(MockGitHubApi::with_issue_labels(&["adapter"]));
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
    assert_eq!(api.labels_for_issue(7), ["adapter"]);
}

#[tokio::test]
async fn managed_label_write_preserves_existing_non_managed_labels() {
    let api = Arc::new(MockGitHubApi::with_issue_labels(&["adapter"]));
    let client = GitHubClient::from_api(api.clone());
    let payload = GitHubLabelWrite {
        repository: repository(),
        issue_number: 7,
        labels: vec!["ubu".to_owned()],
    };

    apply_managed_label_write(&client, &payload, AuthoritySource::AutomationWorker)
        .await
        .unwrap();

    assert_eq!(api.labels_for_issue(7), ["adapter", "ubu"]);
}

#[tokio::test]
async fn reconciliation_read_returns_observed_managed_label_state() {
    let api = Arc::new(MockGitHubApi::with_issue_labels(&[
        "adapter",
        "ubu-managed",
        "triage",
        "ubu",
    ]));
    let client = GitHubClient::from_api(api);

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
}

#[derive(Default)]
struct MockGitHubApi {
    issue_labels: Mutex<BTreeMap<(String, String, u64), Vec<String>>>,
}

impl MockGitHubApi {
    fn with_issue_labels(labels: &[&str]) -> Self {
        let api = Self::default();
        api.issue_labels.lock().unwrap().insert(
            ("UbU-project".to_owned(), "ubu-github-adapter".to_owned(), 7),
            labels.iter().map(|label| (*label).to_owned()).collect(),
        );
        api
    }

    fn labels_for_issue(&self, issue_number: u64) -> Vec<String> {
        self.issue_labels
            .lock()
            .unwrap()
            .get(&(
                "UbU-project".to_owned(),
                "ubu-github-adapter".to_owned(),
                issue_number,
            ))
            .cloned()
            .unwrap_or_default()
    }
}

impl GitHubApi for MockGitHubApi {
    fn create_label<'a>(
        &'a self,
        _owner: &'a str,
        _repo: &'a str,
        _label: &'a str,
        _color: &'a str,
        _description: &'a str,
    ) -> GitHubApiFuture<'a, ()> {
        Box::pin(async { Ok(()) })
    }

    fn add_labels_to_issue<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        issue_number: u64,
        labels: &'a [String],
    ) -> GitHubApiFuture<'a, ()> {
        let owner = owner.to_owned();
        let repo = repo.to_owned();
        let labels = labels.to_vec();
        Box::pin(async move {
            let mut issue_labels = self.issue_labels.lock().unwrap();
            let existing = issue_labels.entry((owner, repo, issue_number)).or_default();
            for label in labels {
                if !existing.contains(&label) {
                    existing.push(label);
                }
            }
            Ok(())
        })
    }

    fn remove_label_from_issue<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        issue_number: u64,
        label: &'a str,
    ) -> GitHubApiFuture<'a, ()> {
        let owner = owner.to_owned();
        let repo = repo.to_owned();
        let label = label.to_owned();
        Box::pin(async move {
            if let Some(existing) =
                self.issue_labels
                    .lock()
                    .unwrap()
                    .get_mut(&(owner, repo, issue_number))
            {
                existing.retain(|existing_label| existing_label != &label);
            }
            Ok(())
        })
    }

    fn create_comment<'a>(
        &'a self,
        _owner: &'a str,
        _repo: &'a str,
        _issue_number: u64,
        _body: &'a str,
    ) -> GitHubApiFuture<'a, ()> {
        Box::pin(async { Ok(()) })
    }

    fn create_issue<'a>(
        &'a self,
        _owner: &'a str,
        _repo: &'a str,
        _title: &'a str,
        _body: &'a str,
        _labels: &'a [String],
    ) -> GitHubApiFuture<'a, ()> {
        Box::pin(async { Ok(()) })
    }

    fn issue_labels<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        issue_number: u64,
    ) -> GitHubApiFuture<'a, Vec<String>> {
        let owner = owner.to_owned();
        let repo = repo.to_owned();
        Box::pin(async move {
            Ok(self
                .issue_labels
                .lock()
                .unwrap()
                .get(&(owner, repo, issue_number))
                .cloned()
                .unwrap_or_default())
        })
    }
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
