use std::collections::BTreeSet;

use ubu_core::projection::approval::ProjectionApproval;

use crate::errors::{AdapterError, Result};
use crate::markers::{is_managed_label, MANAGED_LABELS};
use crate::projection::operations::{
    GitHubProjectionOperationKind, GitHubProjectionPayload, ManagedLabelPreflightPayload,
};
use crate::projection::preview::ProjectionPreviewBatch;

pub fn validate_projection_approval(
    preview: &ProjectionPreviewBatch,
    approval: &ProjectionApproval,
) -> Result<()> {
    validate_projection_approval_with_existing_labels(preview, approval, &managed_labels())
}

pub fn validate_projection_approval_with_existing_labels(
    preview: &ProjectionPreviewBatch,
    approval: &ProjectionApproval,
    existing_labels: &[String],
) -> Result<()> {
    if approval.preview_id != preview.preview.id {
        return Err(AdapterError::ApprovalPreviewMismatch {
            preview_id: preview.preview.id.to_string(),
            approval_preview_id: approval.preview_id.to_string(),
        });
    }

    if !approval.approved {
        return Err(AdapterError::PreviewNotApproved {
            preview_id: preview.preview.id.to_string(),
        });
    }

    validate_operations(preview)?;
    validate_managed_label_preflight(preview, existing_labels)?;

    Ok(())
}

fn validate_operations(preview: &ProjectionPreviewBatch) -> Result<()> {
    for operation in &preview.github_operations {
        match (&operation.kind, &operation.payload) {
            (
                GitHubProjectionOperationKind::ManagedLabelPreflight,
                GitHubProjectionPayload::ManagedLabelPreflight(payload),
            ) => validate_preflight_payload(payload)?,
            (
                GitHubProjectionOperationKind::ApplyLabel
                | GitHubProjectionOperationKind::RemoveLabel,
                GitHubProjectionPayload::Label { label },
            ) if is_managed_label(label) => {}
            (
                GitHubProjectionOperationKind::CreateComment,
                GitHubProjectionPayload::Comment { body, .. },
            ) if crate::markers::has_managed_markers(body) => {}
            (
                GitHubProjectionOperationKind::CreateManagedIssue,
                GitHubProjectionPayload::ManagedIssue { body, .. },
            ) if crate::markers::has_managed_markers(body) => {}
            _ => {
                return Err(AdapterError::ForbiddenProjectionOperation {
                    reason: format!(
                        "operation {} is not allowed by the Phase 1 contract",
                        operation.operation_id
                    ),
                });
            }
        }
    }

    Ok(())
}

fn validate_preflight_payload(payload: &ManagedLabelPreflightPayload) -> Result<()> {
    for label in &payload.missing_labels {
        if !is_managed_label(label) {
            return Err(AdapterError::ForbiddenProjectionOperation {
                reason: format!("label creation is limited to UbU managed labels, got {label}"),
            });
        }
    }
    Ok(())
}

fn validate_managed_label_preflight(
    preview: &ProjectionPreviewBatch,
    existing_labels: &[String],
) -> Result<()> {
    let existing = existing_labels
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let covered_by_preflight = preview
        .github_operations
        .iter()
        .filter_map(|operation| match &operation.payload {
            GitHubProjectionPayload::ManagedLabelPreflight(payload) => Some(payload),
            _ => None,
        })
        .flat_map(|payload| payload.missing_labels.iter().map(String::as_str))
        .collect::<BTreeSet<_>>();

    for label in MANAGED_LABELS {
        if !existing.contains(label) && !covered_by_preflight.contains(label) {
            return Err(AdapterError::MissingManagedLabel {
                label: label.to_owned(),
            });
        }
    }

    Ok(())
}

fn managed_labels() -> Vec<String> {
    MANAGED_LABELS
        .iter()
        .map(|label| (*label).to_owned())
        .collect()
}
