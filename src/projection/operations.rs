use serde::{Deserialize, Serialize};
use ubu_core::SourceRef;

use crate::markers::{is_managed_label, MANAGED_LABELS};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHubProjectionOperationKind {
    ManagedLabelPreflight,
    ApplyLabel,
    RemoveLabel,
    CreateComment,
    CreateManagedIssue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubProjectionTarget {
    pub owner: String,
    pub repo: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_number: Option<u64>,
}

impl GitHubProjectionTarget {
    pub fn repository(owner: impl Into<String>, repo: impl Into<String>) -> Self {
        Self {
            owner: owner.into(),
            repo: repo.into(),
            issue_number: None,
        }
    }

    pub fn issue(owner: impl Into<String>, repo: impl Into<String>, issue_number: u64) -> Self {
        Self {
            owner: owner.into(),
            repo: repo.into(),
            issue_number: Some(issue_number),
        }
    }

    pub fn source_ref(&self, kind: &str) -> SourceRef {
        let source_id = match self.issue_number {
            Some(number) => format!("{}/{}#{}", self.owner, self.repo, number),
            None => format!("{}/{}", self.owner, self.repo),
        };
        SourceRef {
            source_kind: kind.to_owned(),
            source_id,
            url: Some(format!("https://github.com/{}/{}", self.owner, self.repo)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedLabelPreflightPayload {
    pub missing_labels: Vec<String>,
}

impl ManagedLabelPreflightPayload {
    pub fn new(missing_labels: Vec<String>) -> Self {
        let missing_labels = missing_labels
            .into_iter()
            .filter(|label| is_managed_label(label))
            .collect();
        Self { missing_labels }
    }

    pub fn required() -> Self {
        Self {
            missing_labels: MANAGED_LABELS
                .iter()
                .map(|label| (*label).to_owned())
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum GitHubProjectionPayload {
    ManagedLabelPreflight(ManagedLabelPreflightPayload),
    Label { label: String },
    Comment { task_id: String, body: String },
    ManagedIssue { title: String, body: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubProjectionOperation {
    pub operation_id: String,
    pub kind: GitHubProjectionOperationKind,
    pub target: GitHubProjectionTarget,
    pub summary: String,
    pub payload: GitHubProjectionPayload,
}

impl GitHubProjectionOperation {
    pub fn managed_label_preflight(
        operation_id: impl Into<String>,
        target: GitHubProjectionTarget,
        missing_labels: Vec<String>,
    ) -> Self {
        Self {
            operation_id: operation_id.into(),
            kind: GitHubProjectionOperationKind::ManagedLabelPreflight,
            target,
            summary: "Create missing UbU managed labels".to_owned(),
            payload: GitHubProjectionPayload::ManagedLabelPreflight(
                ManagedLabelPreflightPayload::new(missing_labels),
            ),
        }
    }
}
