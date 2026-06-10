use ubu_core::projection::approval::ProjectionApproval;

use crate::errors::Result;
use crate::projection::preview::ProjectionPreviewBatch;
use crate::projection::result::GitHubProjectionResult;
use crate::write::GitHubProjectionWriter;

pub async fn apply_projection(
    writer: &GitHubProjectionWriter,
    preview: &ProjectionPreviewBatch,
    approval: &ProjectionApproval,
    existing_labels: &[String],
) -> Result<GitHubProjectionResult> {
    writer
        .apply_approved(preview, approval, existing_labels)
        .await
}
