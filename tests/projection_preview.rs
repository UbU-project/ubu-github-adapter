use ubu_core::{ObjectType, UbuId};
use ubu_github_adapter::fixture::GitHubFixture;
use ubu_github_adapter::projection::labels::apply_managed_label;
use ubu_github_adapter::projection::operations::{
    GitHubProjectionOperationKind, GitHubProjectionPayload, GitHubProjectionTarget,
};
use ubu_github_adapter::projection::preview::{
    preview_for_operations, preview_for_operations_with_existing_labels,
};

#[test]
fn builds_projection_preview_for_label_write() {
    let operation = apply_managed_label(
        "op-1",
        GitHubProjectionTarget::issue("UbU-project", "ubu-github-adapter", 7),
        "ubu",
    );

    let preview = preview_for_operations(vec![operation]).unwrap();
    assert_eq!(preview.github_operations.len(), 1);
    preview
        .preview
        .id
        .require_object_type(ObjectType::ProjectionPreview)
        .unwrap();
    UbuId::parse(preview.preview.id.to_string()).unwrap();
}

#[test]
fn fresh_repo_preview_includes_managed_label_preflight() {
    let fixture = GitHubFixture::from_path("fixtures/github/repo-fresh.json").unwrap();
    let existing_labels = fixture
        .labels
        .iter()
        .map(|label| label.name.clone())
        .collect::<Vec<_>>();
    let operation = apply_managed_label(
        "op-1",
        GitHubProjectionTarget::issue("UbU-project", "ubu-github-adapter", 7),
        "ubu",
    );

    let preview =
        preview_for_operations_with_existing_labels(vec![operation], &existing_labels).unwrap();

    assert_eq!(preview.github_operations.len(), 2);
    assert_eq!(
        preview.github_operations[0].kind,
        GitHubProjectionOperationKind::ManagedLabelPreflight
    );
    let GitHubProjectionPayload::ManagedLabelPreflight(payload) =
        &preview.github_operations[0].payload
    else {
        panic!("first operation should be managed-label preflight");
    };
    assert_eq!(payload.missing_labels, ["ubu", "ubu-managed"]);
    assert_eq!(preview.preview.operations.len(), 2);
}
