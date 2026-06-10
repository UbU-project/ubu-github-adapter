#[path = "projection/comments.rs"]
pub mod comments;
#[path = "projection/labels.rs"]
pub mod labels;
#[path = "projection/managed_issues.rs"]
pub mod managed_issues;
#[path = "projection/operations.rs"]
pub mod operations;
#[path = "projection/preview.rs"]
pub mod preview;
#[path = "projection/reconciliation.rs"]
pub mod reconciliation;
#[path = "projection/result.rs"]
pub mod result;

pub use operations::{
    GitHubProjectionOperation, GitHubProjectionOperationKind, GitHubProjectionTarget,
    ManagedLabelPreflightPayload,
};
pub use preview::{preview_for_operations, ProjectionPreviewBatch};
pub use result::{GitHubOperationResult, GitHubProjectionResult};
