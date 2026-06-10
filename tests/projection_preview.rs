use ubu_core::{ObjectType, UbuId};
use ubu_github_adapter::projection::labels::apply_managed_label;
use ubu_github_adapter::projection::operations::GitHubProjectionTarget;
use ubu_github_adapter::projection::preview::preview_for_operations;

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
