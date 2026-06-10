use crate::markers::{is_managed_label, MANAGED_LABELS};
use crate::projection::operations::{
    GitHubProjectionOperation, GitHubProjectionOperationKind, GitHubProjectionPayload,
    GitHubProjectionTarget,
};

pub fn managed_label_preflight(
    owner: impl Into<String>,
    repo: impl Into<String>,
    existing_labels: &[String],
) -> Option<GitHubProjectionOperation> {
    let existing: Vec<&str> = existing_labels.iter().map(String::as_str).collect();
    let missing = MANAGED_LABELS
        .iter()
        .filter(|label| !existing.contains(label))
        .map(|label| (*label).to_owned())
        .collect::<Vec<_>>();

    if missing.is_empty() {
        return None;
    }

    Some(GitHubProjectionOperation::managed_label_preflight(
        "managed-label-preflight",
        GitHubProjectionTarget::repository(owner, repo),
        missing,
    ))
}

pub fn apply_managed_label(
    operation_id: impl Into<String>,
    target: GitHubProjectionTarget,
    label: impl Into<String>,
) -> GitHubProjectionOperation {
    let label = label.into();
    assert!(
        is_managed_label(&label),
        "Phase 1 labels must be UbU-managed"
    );
    GitHubProjectionOperation {
        operation_id: operation_id.into(),
        kind: GitHubProjectionOperationKind::ApplyLabel,
        target,
        summary: format!("Apply managed label {label}"),
        payload: GitHubProjectionPayload::Label { label },
    }
}

pub fn remove_managed_label(
    operation_id: impl Into<String>,
    target: GitHubProjectionTarget,
    label: impl Into<String>,
) -> GitHubProjectionOperation {
    let label = label.into();
    assert!(
        is_managed_label(&label),
        "Phase 1 labels must be UbU-managed"
    );
    GitHubProjectionOperation {
        operation_id: operation_id.into(),
        kind: GitHubProjectionOperationKind::RemoveLabel,
        target,
        summary: format!("Remove managed label {label}"),
        payload: GitHubProjectionPayload::Label { label },
    }
}
