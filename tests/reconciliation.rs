use ubu_core::projection::result::{
    OperationResult, OperationResultStatus, ProjectionResult, ProjectionResultStatus,
};
use ubu_core::{AuthoritySource, UbuTimestamp};
use ubu_github_adapter::projection::labels::apply_managed_label;
use ubu_github_adapter::projection::operations::GitHubProjectionTarget;
use ubu_github_adapter::projection::preview::preview_for_operations;
use ubu_github_adapter::projection::result::{GitHubOperationResult, GitHubProjectionResult};
use ubu_github_adapter::reconcile::reconcile_projection_result;

#[test]
fn reconciles_matching_projection_result() {
    let operation = apply_managed_label(
        "op-1",
        GitHubProjectionTarget::issue("UbU-project", "ubu-github-adapter", 7),
        "ubu",
    );
    let preview = preview_for_operations(vec![operation]).unwrap();
    let result = GitHubProjectionResult {
        core_result: ProjectionResult {
            preview_id: preview.preview.id.clone(),
            applied_at: UbuTimestamp::now_utc(),
            status: ProjectionResultStatus::Applied,
            operation_results: vec![OperationResult {
                operation_id: "op-1".to_owned(),
                status: OperationResultStatus::Applied,
                message: None,
            }],
        },
        authority_source: AuthoritySource::User,
        github_results: vec![GitHubOperationResult {
            operation_id: "op-1".to_owned(),
            applied: true,
            message: None,
        }],
    };

    reconcile_projection_result(&preview, &result).unwrap();
}
