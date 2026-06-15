use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("missing GitHub token in {variable}")]
    MissingToken { variable: &'static str },

    #[error("fixture IO failed: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("UbU core validation failed: {0}")]
    Core(#[from] ubu_core::UbuError),

    #[error("GitHub API request failed")]
    GitHub(#[from] octocrab::Error),

    #[error("projection approval does not approve preview {preview_id}")]
    PreviewNotApproved { preview_id: String },

    #[error("projection approval is for {approval_preview_id}, expected {preview_id}")]
    ApprovalPreviewMismatch {
        preview_id: String,
        approval_preview_id: String,
    },

    #[error("missing managed label preflight for {label}")]
    MissingManagedLabel { label: String },

    #[error("forbidden projection operation: {reason}")]
    ForbiddenProjectionOperation { reason: String },

    #[error("unsupported projection target: {source_kind}")]
    UnsupportedProjectionTarget { source_kind: String },

    #[error("projection result did not match preview: {reason}")]
    Reconciliation { reason: String },
}

pub type Result<T> = std::result::Result<T, AdapterError>;
