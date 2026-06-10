use std::collections::BTreeSet;

use crate::errors::{AdapterError, Result};
use crate::projection::preview::ProjectionPreviewBatch;
use crate::projection::result::GitHubProjectionResult;

pub fn reconcile_projection_result(
    preview: &ProjectionPreviewBatch,
    result: &GitHubProjectionResult,
) -> Result<()> {
    if result.core_result.preview_id != preview.preview.id {
        return Err(AdapterError::Reconciliation {
            reason: "result preview id differs from preview".to_owned(),
        });
    }

    let expected = preview
        .github_operations
        .iter()
        .map(|operation| operation.operation_id.as_str())
        .collect::<BTreeSet<_>>();
    let actual = result
        .github_results
        .iter()
        .map(|operation| operation.operation_id.as_str())
        .collect::<BTreeSet<_>>();

    if expected != actual {
        return Err(AdapterError::Reconciliation {
            reason: "operation result set differs from preview".to_owned(),
        });
    }

    Ok(())
}
