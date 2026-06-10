use ubu_core::UbuId;

use crate::markers::managed_body;
use crate::projection::operations::{
    GitHubProjectionOperation, GitHubProjectionOperationKind, GitHubProjectionPayload,
    GitHubProjectionTarget,
};

pub fn create_managed_issue(
    operation_id: impl Into<String>,
    target: GitHubProjectionTarget,
    task_id: &UbuId,
    title: impl Into<String>,
    body: impl AsRef<str>,
) -> GitHubProjectionOperation {
    let title = title.into();
    GitHubProjectionOperation {
        operation_id: operation_id.into(),
        kind: GitHubProjectionOperationKind::CreateManagedIssue,
        target,
        summary: format!("Create UbU-managed issue for {task_id}"),
        payload: GitHubProjectionPayload::ManagedIssue {
            title,
            body: managed_body(task_id, body.as_ref()),
        },
    }
}
