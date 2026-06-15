use serde::{Deserialize, Serialize};
use serde_json::Value;
use ubu_core::projection::operation::{
    ProjectionOperation, ProjectionOperationKind as CoreProjectionOperationKind,
};
use ubu_core::projection::preview::ProjectionPreview;
use ubu_core::{ObjectType, UbuId, UbuTimestamp};

use crate::errors::Result;
use crate::projection::operations::{GitHubProjectionOperation, GitHubProjectionOperationKind};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProjectionPreviewBatch {
    pub preview: ProjectionPreview,
    pub github_operations: Vec<GitHubProjectionOperation>,
}

pub fn preview_for_operations(
    operations: Vec<GitHubProjectionOperation>,
) -> Result<ProjectionPreviewBatch> {
    let core_operations = operations
        .iter()
        .map(core_operation_from_github)
        .collect::<Result<Vec<_>>>()?;
    let preview = ProjectionPreview {
        id: UbuId::new(ObjectType::ProjectionPreview),
        created_at: UbuTimestamp::now_utc(),
        operations: core_operations,
        policy_summary: None,
    };

    Ok(ProjectionPreviewBatch {
        preview,
        github_operations: operations,
    })
}

fn core_operation_from_github(
    operation: &GitHubProjectionOperation,
) -> Result<ProjectionOperation> {
    let kind = match operation.kind {
        GitHubProjectionOperationKind::ManagedLabelPreflight
        | GitHubProjectionOperationKind::ApplyLabel
        | GitHubProjectionOperationKind::RemoveLabel => CoreProjectionOperationKind::Label,
        GitHubProjectionOperationKind::CreateComment => CoreProjectionOperationKind::Comment,
        GitHubProjectionOperationKind::CreateManagedIssue => CoreProjectionOperationKind::Create,
    };
    Ok(ProjectionOperation {
        operation_id: operation.operation_id.clone(),
        kind,
        target: operation.target.source_ref("github"),
        summary: operation.summary.clone(),
        payload: Some(serde_json::to_value(operation)? as Value),
    })
}
