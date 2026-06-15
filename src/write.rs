use serde_json::json;
use ubu_core::projection::approval::ProjectionApproval;
use ubu_core::projection::result::{
    OperationResult, OperationResultStatus, ProjectionResult, ProjectionResultStatus,
};
use ubu_core::UbuTimestamp;

use crate::approval::validate_projection_approval_with_existing_labels;
use crate::client::GitHubClient;
use crate::errors::{AdapterError, Result};
use crate::markers::is_managed_label;
use crate::projection::operations::{
    GitHubProjectionOperation, GitHubProjectionOperationKind, GitHubProjectionPayload,
};
use crate::projection::preview::ProjectionPreviewBatch;
use crate::projection::result::{GitHubOperationResult, GitHubProjectionResult};

pub struct GitHubProjectionWriter {
    client: GitHubClient,
}

impl GitHubProjectionWriter {
    pub fn new(client: GitHubClient) -> Self {
        Self { client }
    }

    pub async fn apply_approved(
        &self,
        preview: &ProjectionPreviewBatch,
        approval: &ProjectionApproval,
        existing_labels: &[String],
    ) -> Result<GitHubProjectionResult> {
        validate_projection_approval_with_existing_labels(preview, approval, existing_labels)?;

        let mut github_results = Vec::new();
        let mut operation_results = Vec::new();

        for operation in &preview.github_operations {
            let outcome = self.apply_operation(operation).await;
            match outcome {
                Ok(message) => {
                    github_results.push(GitHubOperationResult {
                        operation_id: operation.operation_id.clone(),
                        applied: true,
                        message,
                    });
                    operation_results.push(OperationResult {
                        operation_id: operation.operation_id.clone(),
                        status: OperationResultStatus::Applied,
                        message: None,
                    });
                }
                Err(error) => {
                    github_results.push(GitHubOperationResult {
                        operation_id: operation.operation_id.clone(),
                        applied: false,
                        message: Some(error.to_string()),
                    });
                    operation_results.push(OperationResult {
                        operation_id: operation.operation_id.clone(),
                        status: OperationResultStatus::Failed,
                        message: Some(error.to_string()),
                    });
                }
            }
        }

        let status = if operation_results
            .iter()
            .all(|result| result.status == OperationResultStatus::Applied)
        {
            ProjectionResultStatus::Applied
        } else if operation_results
            .iter()
            .any(|result| result.status == OperationResultStatus::Applied)
        {
            ProjectionResultStatus::Partial
        } else {
            ProjectionResultStatus::Failed
        };

        Ok(GitHubProjectionResult {
            core_result: ProjectionResult {
                preview_id: preview.preview.id.clone(),
                applied_at: UbuTimestamp::now_utc(),
                status,
                operation_results,
            },
            authority_source: approval.authority_source,
            github_results,
        })
    }

    async fn apply_operation(
        &self,
        operation: &GitHubProjectionOperation,
    ) -> Result<Option<String>> {
        match (&operation.kind, &operation.payload) {
            (
                GitHubProjectionOperationKind::ManagedLabelPreflight,
                GitHubProjectionPayload::ManagedLabelPreflight(payload),
            ) => {
                for label in &payload.missing_labels {
                    self.create_managed_label(operation, label).await?;
                }
                Ok(Some("managed labels ensured".to_owned()))
            }
            (
                GitHubProjectionOperationKind::ApplyLabel,
                GitHubProjectionPayload::Label { label },
            ) => {
                let number = issue_number(operation)?;
                let route = format!(
                    "/repos/{}/{}/issues/{number}/labels",
                    operation.target.owner, operation.target.repo
                );
                let body = json!({ "labels": [label] });
                let _: serde_json::Value = self.client.octocrab().post(route, Some(&body)).await?;
                Ok(Some(format!("label {label} applied")))
            }
            (
                GitHubProjectionOperationKind::RemoveLabel,
                GitHubProjectionPayload::Label { label },
            ) => {
                let number = issue_number(operation)?;
                let route = format!(
                    "/repos/{}/{}/issues/{number}/labels/{label}",
                    operation.target.owner, operation.target.repo
                );
                let _: serde_json::Value =
                    self.client.octocrab().delete(route, None::<&()>).await?;
                Ok(Some(format!("label {label} removed")))
            }
            (
                GitHubProjectionOperationKind::CreateComment,
                GitHubProjectionPayload::Comment { body, .. },
            ) => {
                let number = issue_number(operation)?;
                let route = format!(
                    "/repos/{}/{}/issues/{number}/comments",
                    operation.target.owner, operation.target.repo
                );
                let request = json!({ "body": body });
                let _: serde_json::Value =
                    self.client.octocrab().post(route, Some(&request)).await?;
                Ok(Some("managed comment created".to_owned()))
            }
            (
                GitHubProjectionOperationKind::CreateManagedIssue,
                GitHubProjectionPayload::ManagedIssue { title, body },
            ) => {
                let route = format!(
                    "/repos/{}/{}/issues",
                    operation.target.owner, operation.target.repo
                );
                let request = json!({
                    "title": title,
                    "body": body,
                    "labels": ["ubu", "ubu-managed"],
                });
                let _: serde_json::Value =
                    self.client.octocrab().post(route, Some(&request)).await?;
                Ok(Some("managed issue created".to_owned()))
            }
            _ => Err(AdapterError::ForbiddenProjectionOperation {
                reason: format!(
                    "operation {} has mismatched payload",
                    operation.operation_id
                ),
            }),
        }
    }

    async fn create_managed_label(
        &self,
        operation: &GitHubProjectionOperation,
        label: &str,
    ) -> Result<()> {
        if !is_managed_label(label) {
            return Err(AdapterError::ForbiddenProjectionOperation {
                reason: format!("label creation is limited to UbU managed labels, got {label}"),
            });
        }

        let route = format!(
            "/repos/{}/{}/labels",
            operation.target.owner, operation.target.repo
        );
        let color = if label == "ubu" { "5319e7" } else { "0e8a16" };
        let body = json!({
            "name": label,
            "color": color,
            "description": "UbU managed label",
        });
        let _: serde_json::Value = self.client.octocrab().post(route, Some(&body)).await?;
        Ok(())
    }
}

fn issue_number(operation: &GitHubProjectionOperation) -> Result<u64> {
    operation
        .target
        .issue_number
        .ok_or_else(|| AdapterError::UnsupportedProjectionTarget {
            source_kind: "github_repository".to_owned(),
        })
}
