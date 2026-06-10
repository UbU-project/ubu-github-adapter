use crate::errors::Result;
use crate::projection::operations::GitHubProjectionOperation;
use crate::projection::preview::{preview_for_operations, ProjectionPreviewBatch};

pub fn preview_projection(
    operations: Vec<GitHubProjectionOperation>,
) -> Result<ProjectionPreviewBatch> {
    preview_for_operations(operations)
}
