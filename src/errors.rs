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

    #[error("label write is limited to UbU managed labels, got {label}")]
    UnmanagedLabelWrite { label: String },

    #[error("forbidden projection operation: {reason}")]
    ForbiddenProjectionOperation { reason: String },

    #[error("unsupported projection target: {source_kind}")]
    UnsupportedProjectionTarget { source_kind: String },

    #[error("projection result did not match preview: {reason}")]
    Reconciliation { reason: String },
}

impl AdapterError {
    pub fn is_rate_limit_or_transport_failure(&self) -> bool {
        is_rate_limit_or_transport_failure(self)
    }
}

pub fn is_rate_limit_or_transport_failure(error: &AdapterError) -> bool {
    match error {
        AdapterError::GitHub(error) => is_octocrab_rate_limit_or_transport_failure(error),
        _ => false,
    }
}

fn is_octocrab_rate_limit_or_transport_failure(error: &octocrab::Error) -> bool {
    match error {
        octocrab::Error::GitHub { source, .. } => {
            github_status_is_rate_limited(source.status_code.as_u16(), &source.message)
        }
        octocrab::Error::Hyper { .. } | octocrab::Error::Service { .. } => true,
        _ => false,
    }
}

fn github_status_is_rate_limited(status_code: u16, message: &str) -> bool {
    if status_code == 429 {
        return true;
    }

    if status_code != 403 {
        return false;
    }

    let message = message.to_ascii_lowercase();
    message.contains("rate limit")
        || message.contains("secondary rate")
        || message.contains("abuse detection")
}

pub type Result<T> = std::result::Result<T, AdapterError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_errors_are_not_rate_limit_or_transport_failures() {
        let error = AdapterError::UnmanagedLabelWrite {
            label: "triage".to_owned(),
        };

        assert!(!error.is_rate_limit_or_transport_failure());
        assert!(!is_rate_limit_or_transport_failure(&error));
    }

    #[test]
    fn github_status_classifier_detects_rate_limit_messages() {
        assert!(github_status_is_rate_limited(
            403,
            "API rate limit exceeded for user ID 1."
        ));
        assert!(github_status_is_rate_limited(
            403,
            "You have exceeded a secondary rate limit."
        ));
        assert!(github_status_is_rate_limited(
            429,
            "any throttling response"
        ));

        assert!(!github_status_is_rate_limited(
            403,
            "Resource not accessible"
        ));
        assert!(!github_status_is_rate_limited(404, "Not Found"));
    }
}
