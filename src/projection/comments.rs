use ubu_core::UbuId;

use crate::markers::managed_body;
use crate::projection::operations::{
    GitHubProjectionOperation, GitHubProjectionOperationKind, GitHubProjectionPayload,
    GitHubProjectionTarget,
};

pub fn create_managed_comment(
    operation_id: impl Into<String>,
    target: GitHubProjectionTarget,
    task_id: &UbuId,
    body: impl AsRef<str>,
) -> GitHubProjectionOperation {
    GitHubProjectionOperation {
        operation_id: operation_id.into(),
        kind: GitHubProjectionOperationKind::CreateComment,
        target,
        summary: format!("Create UbU-managed comment for {task_id}"),
        payload: GitHubProjectionPayload::Comment {
            task_id: task_id.to_string(),
            body: managed_body(task_id, body.as_ref()),
        },
    }
}
