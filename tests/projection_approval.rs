use ubu_core::projection::approval::ProjectionApproval;
use ubu_core::{AuthoritySource, UbuTimestamp};
use ubu_github_adapter::approval::validate_projection_approval_with_existing_labels;
use ubu_github_adapter::errors::AdapterError;
use ubu_github_adapter::projection::labels::apply_managed_label;
use ubu_github_adapter::projection::operations::GitHubProjectionTarget;
use ubu_github_adapter::projection::preview::preview_for_operations;

#[test]
fn approval_is_per_preview_batch() {
    let operation = apply_managed_label(
        "op-1",
        GitHubProjectionTarget::issue("UbU-project", "ubu-github-adapter", 7),
        "ubu",
    );
    let preview = preview_for_operations(vec![operation]).unwrap();
    let approval = ProjectionApproval {
        preview_id: preview.preview.id.clone(),
        approved: true,
        approved_at: UbuTimestamp::now_utc(),
        authority_source: AuthoritySource::User,
    };
    let labels = vec!["ubu".to_owned(), "ubu-managed".to_owned()];

    validate_projection_approval_with_existing_labels(&preview, &approval, &labels).unwrap();
}

#[test]
fn approval_fails_when_managed_label_preflight_is_missing() {
    let operation = apply_managed_label(
        "op-1",
        GitHubProjectionTarget::issue("UbU-project", "ubu-github-adapter", 7),
        "ubu",
    );
    let preview = preview_for_operations(vec![operation]).unwrap();
    let approval = ProjectionApproval {
        preview_id: preview.preview.id.clone(),
        approved: true,
        approved_at: UbuTimestamp::now_utc(),
        authority_source: AuthoritySource::User,
    };
    let labels = vec!["ubu".to_owned()];

    let error = validate_projection_approval_with_existing_labels(&preview, &approval, &labels)
        .unwrap_err();
    assert!(matches!(error, AdapterError::MissingManagedLabel { label } if label == "ubu-managed"));
}
