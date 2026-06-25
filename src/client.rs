use octocrab::Octocrab;
use serde_json::json;
use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use crate::auth::GitHubAuth;
use crate::errors::Result;

pub type GitHubApiFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>;

pub trait GitHubApi: Send + Sync {
    fn create_label<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        label: &'a str,
        color: &'a str,
        description: &'a str,
    ) -> GitHubApiFuture<'a, ()>;

    fn add_labels_to_issue<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        issue_number: u64,
        labels: &'a [String],
    ) -> GitHubApiFuture<'a, ()>;

    fn remove_label_from_issue<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        issue_number: u64,
        label: &'a str,
    ) -> GitHubApiFuture<'a, ()>;

    fn create_comment<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        issue_number: u64,
        body: &'a str,
    ) -> GitHubApiFuture<'a, ()>;

    fn create_issue<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        title: &'a str,
        body: &'a str,
        labels: &'a [String],
    ) -> GitHubApiFuture<'a, ()>;

    fn issue_labels<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        issue_number: u64,
    ) -> GitHubApiFuture<'a, Vec<String>>;
}

#[derive(Clone)]
pub struct GitHubClient {
    api: Arc<dyn GitHubApi>,
}

impl GitHubClient {
    pub fn from_auth(auth: GitHubAuth) -> Result<Self> {
        let octocrab = Octocrab::builder()
            .personal_token(auth.token().expose().to_owned())
            .build()?;
        Ok(Self {
            api: Arc::new(OctocrabGitHubApi { octocrab }),
        })
    }

    pub fn from_api(api: Arc<dyn GitHubApi>) -> Self {
        Self { api }
    }

    pub fn recording(api: Arc<RecordingGitHubApi>) -> Self {
        Self::from_api(api)
    }

    pub fn api(&self) -> &dyn GitHubApi {
        self.api.as_ref()
    }
}

type IssueKey = (String, String, u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordedGitHubOperation {
    CreateLabel {
        owner: String,
        repo: String,
        label: String,
        color: String,
        description: String,
    },
    AddLabelsToIssue {
        owner: String,
        repo: String,
        issue_number: u64,
        labels: Vec<String>,
    },
    RemoveLabelFromIssue {
        owner: String,
        repo: String,
        issue_number: u64,
        label: String,
    },
    CreateComment {
        owner: String,
        repo: String,
        issue_number: u64,
        body: String,
    },
    CreateIssue {
        owner: String,
        repo: String,
        title: String,
        body: String,
        labels: Vec<String>,
    },
    ReadIssueLabels {
        owner: String,
        repo: String,
        issue_number: u64,
    },
}

#[derive(Debug, Default)]
pub struct RecordingGitHubApi {
    operations: Mutex<Vec<RecordedGitHubOperation>>,
    issue_labels: Mutex<BTreeMap<IssueKey, Vec<String>>>,
}

impl RecordingGitHubApi {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_issue_labels<I, S>(owner: &str, repo: &str, issue_number: u64, labels: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let api = Self::new();
        api.seed_issue_labels(owner, repo, issue_number, labels);
        api
    }

    pub fn seed_issue_labels<I, S>(&self, owner: &str, repo: &str, issue_number: u64, labels: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.issue_labels.lock().unwrap().insert(
            issue_key(owner, repo, issue_number),
            labels.into_iter().map(Into::into).collect(),
        );
    }

    pub fn issue_labels(&self, owner: &str, repo: &str, issue_number: u64) -> Vec<String> {
        self.issue_labels
            .lock()
            .unwrap()
            .get(&issue_key(owner, repo, issue_number))
            .cloned()
            .unwrap_or_default()
    }

    pub fn recorded_operations(&self) -> Vec<RecordedGitHubOperation> {
        self.operations.lock().unwrap().clone()
    }

    pub fn clear_recorded_operations(&self) {
        self.operations.lock().unwrap().clear();
    }

    fn record(&self, operation: RecordedGitHubOperation) {
        self.operations.lock().unwrap().push(operation);
    }
}

impl GitHubApi for RecordingGitHubApi {
    fn create_label<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        label: &'a str,
        color: &'a str,
        description: &'a str,
    ) -> GitHubApiFuture<'a, ()> {
        let operation = RecordedGitHubOperation::CreateLabel {
            owner: owner.to_owned(),
            repo: repo.to_owned(),
            label: label.to_owned(),
            color: color.to_owned(),
            description: description.to_owned(),
        };
        Box::pin(async move {
            self.record(operation);
            Ok(())
        })
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
            self.record(RecordedGitHubOperation::AddLabelsToIssue {
                owner: owner.clone(),
                repo: repo.clone(),
                issue_number,
                labels: labels.clone(),
            });

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
            self.record(RecordedGitHubOperation::RemoveLabelFromIssue {
                owner: owner.clone(),
                repo: repo.clone(),
                issue_number,
                label: label.clone(),
            });

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
        owner: &'a str,
        repo: &'a str,
        issue_number: u64,
        body: &'a str,
    ) -> GitHubApiFuture<'a, ()> {
        let operation = RecordedGitHubOperation::CreateComment {
            owner: owner.to_owned(),
            repo: repo.to_owned(),
            issue_number,
            body: body.to_owned(),
        };
        Box::pin(async move {
            self.record(operation);
            Ok(())
        })
    }

    fn create_issue<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        title: &'a str,
        body: &'a str,
        labels: &'a [String],
    ) -> GitHubApiFuture<'a, ()> {
        let operation = RecordedGitHubOperation::CreateIssue {
            owner: owner.to_owned(),
            repo: repo.to_owned(),
            title: title.to_owned(),
            body: body.to_owned(),
            labels: labels.to_vec(),
        };
        Box::pin(async move {
            self.record(operation);
            Ok(())
        })
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
            self.record(RecordedGitHubOperation::ReadIssueLabels {
                owner: owner.clone(),
                repo: repo.clone(),
                issue_number,
            });

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

fn issue_key(owner: &str, repo: &str, issue_number: u64) -> IssueKey {
    (owner.to_owned(), repo.to_owned(), issue_number)
}

struct OctocrabGitHubApi {
    octocrab: Octocrab,
}

impl GitHubApi for OctocrabGitHubApi {
    fn create_label<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        label: &'a str,
        color: &'a str,
        description: &'a str,
    ) -> GitHubApiFuture<'a, ()> {
        Box::pin(async move {
            let route = format!("/repos/{owner}/{repo}/labels");
            let body = json!({
                "name": label,
                "color": color,
                "description": description,
            });
            let _: serde_json::Value = self.octocrab.post(route, Some(&body)).await?;
            Ok(())
        })
    }

    fn add_labels_to_issue<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        issue_number: u64,
        labels: &'a [String],
    ) -> GitHubApiFuture<'a, ()> {
        Box::pin(async move {
            if labels.is_empty() {
                return Ok(());
            }

            let route = format!("/repos/{owner}/{repo}/issues/{issue_number}/labels");
            let body = json!({ "labels": labels });
            let _: serde_json::Value = self.octocrab.post(route, Some(&body)).await?;
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
        Box::pin(async move {
            let route = format!("/repos/{owner}/{repo}/issues/{issue_number}/labels/{label}");
            let _: serde_json::Value = self.octocrab.delete(route, None::<&()>).await?;
            Ok(())
        })
    }

    fn create_comment<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        issue_number: u64,
        body: &'a str,
    ) -> GitHubApiFuture<'a, ()> {
        Box::pin(async move {
            let route = format!("/repos/{owner}/{repo}/issues/{issue_number}/comments");
            let request = json!({ "body": body });
            let _: serde_json::Value = self.octocrab.post(route, Some(&request)).await?;
            Ok(())
        })
    }

    fn create_issue<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        title: &'a str,
        body: &'a str,
        labels: &'a [String],
    ) -> GitHubApiFuture<'a, ()> {
        Box::pin(async move {
            let route = format!("/repos/{owner}/{repo}/issues");
            let request = json!({
                "title": title,
                "body": body,
                "labels": labels,
            });
            let _: serde_json::Value = self.octocrab.post(route, Some(&request)).await?;
            Ok(())
        })
    }

    fn issue_labels<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        issue_number: u64,
    ) -> GitHubApiFuture<'a, Vec<String>> {
        Box::pin(async move {
            let route = format!("/repos/{owner}/{repo}/issues/{issue_number}/labels");
            let response: Vec<GitHubLabelResponse> = self.octocrab.get(route, None::<&()>).await?;
            Ok(response.into_iter().map(|label| label.name).collect())
        })
    }
}

#[derive(serde::Deserialize)]
struct GitHubLabelResponse {
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn recording_api_records_all_trait_methods_and_updates_issue_labels() {
        let api = RecordingGitHubApi::with_issue_labels("UbU-project", "repo", 7, ["old"]);

        api.create_label("UbU-project", "repo", "ubu", "5319e7", "UbU managed label")
            .await
            .unwrap();
        api.add_labels_to_issue("UbU-project", "repo", 7, &["ubu".to_owned()])
            .await
            .unwrap();
        api.remove_label_from_issue("UbU-project", "repo", 7, "old")
            .await
            .unwrap();
        api.create_comment("UbU-project", "repo", 7, "body")
            .await
            .unwrap();
        api.create_issue(
            "UbU-project",
            "repo",
            "title",
            "body",
            &["ubu".to_owned(), "ubu-managed".to_owned()],
        )
        .await
        .unwrap();

        assert_eq!(
            api.issue_labels("UbU-project", "repo", 7),
            ["ubu".to_owned()]
        );
        assert_eq!(
            GitHubApi::issue_labels(&api, "UbU-project", "repo", 7)
                .await
                .unwrap(),
            ["ubu".to_owned()]
        );
        assert_eq!(
            api.recorded_operations(),
            [
                RecordedGitHubOperation::CreateLabel {
                    owner: "UbU-project".to_owned(),
                    repo: "repo".to_owned(),
                    label: "ubu".to_owned(),
                    color: "5319e7".to_owned(),
                    description: "UbU managed label".to_owned(),
                },
                RecordedGitHubOperation::AddLabelsToIssue {
                    owner: "UbU-project".to_owned(),
                    repo: "repo".to_owned(),
                    issue_number: 7,
                    labels: vec!["ubu".to_owned()],
                },
                RecordedGitHubOperation::RemoveLabelFromIssue {
                    owner: "UbU-project".to_owned(),
                    repo: "repo".to_owned(),
                    issue_number: 7,
                    label: "old".to_owned(),
                },
                RecordedGitHubOperation::CreateComment {
                    owner: "UbU-project".to_owned(),
                    repo: "repo".to_owned(),
                    issue_number: 7,
                    body: "body".to_owned(),
                },
                RecordedGitHubOperation::CreateIssue {
                    owner: "UbU-project".to_owned(),
                    repo: "repo".to_owned(),
                    title: "title".to_owned(),
                    body: "body".to_owned(),
                    labels: vec!["ubu".to_owned(), "ubu-managed".to_owned()],
                },
                RecordedGitHubOperation::ReadIssueLabels {
                    owner: "UbU-project".to_owned(),
                    repo: "repo".to_owned(),
                    issue_number: 7,
                },
            ]
        );

        api.clear_recorded_operations();
        assert!(api.recorded_operations().is_empty());
    }
}
