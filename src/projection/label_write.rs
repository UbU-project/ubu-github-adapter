use serde::{Deserialize, Serialize};
use ubu_core::{AuthoritySource, Provenance, SourceRef, UbuTimestamp};

use crate::client::GitHubClient;
use crate::errors::{AdapterError, Result};
use crate::markers::{is_managed_label, MANAGED_LABELS};
use crate::sources::GitHubRepositorySource;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GitHubLabelWrite {
    pub repository: GitHubRepositorySource,
    pub issue_number: u64,
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ManagedLabelWriteResult {
    pub source: SourceRef,
    pub applied_labels: Vec<String>,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ManagedLabelObservation {
    pub source: SourceRef,
    pub checked_at: UbuTimestamp,
    pub labels: Vec<String>,
    pub provenance: Provenance,
}

pub async fn apply_managed_label_write(
    client: &GitHubClient,
    payload: &GitHubLabelWrite,
    authority_source: AuthoritySource,
) -> Result<ManagedLabelWriteResult> {
    validate_managed_labels(&payload.labels)?;

    client
        .api()
        .add_labels_to_issue(
            &payload.repository.owner,
            &payload.repository.name,
            payload.issue_number,
            &payload.labels,
        )
        .await?;

    let source = github_issue_source(&payload.repository, payload.issue_number);
    Ok(ManagedLabelWriteResult {
        applied_labels: payload.labels.clone(),
        provenance: provenance(authority_source, source.clone()),
        source,
    })
}

pub async fn read_managed_label_observation(
    client: &GitHubClient,
    repository: &GitHubRepositorySource,
    issue_number: u64,
) -> Result<ManagedLabelObservation> {
    let observed = client
        .api()
        .issue_labels(&repository.owner, &repository.name, issue_number)
        .await?;
    let labels = MANAGED_LABELS
        .iter()
        .filter(|managed| observed.iter().any(|label| label == **managed))
        .map(|label| (*label).to_owned())
        .collect();
    let source = github_issue_source(repository, issue_number);
    let checked_at = UbuTimestamp::now_utc();

    Ok(ManagedLabelObservation {
        source: source.clone(),
        checked_at,
        labels,
        provenance: Provenance {
            created_at: checked_at,
            created_by: Some("ubu-github-adapter".to_owned()),
            authority_source: AuthoritySource::System,
            source: Some(source),
            source_refs: None,
        },
    })
}

fn validate_managed_labels(labels: &[String]) -> Result<()> {
    for label in labels {
        if !is_managed_label(label) {
            return Err(AdapterError::UnmanagedLabelWrite {
                label: label.clone(),
            });
        }
    }

    Ok(())
}

fn provenance(authority_source: AuthoritySource, source: SourceRef) -> Provenance {
    Provenance {
        created_at: UbuTimestamp::now_utc(),
        created_by: Some("ubu-github-adapter".to_owned()),
        authority_source,
        source: Some(source),
        source_refs: None,
    }
}

fn github_issue_source(repository: &GitHubRepositorySource, issue_number: u64) -> SourceRef {
    let full_name = repository.full_name();
    SourceRef {
        source_kind: "github_issue".to_owned(),
        source_id: format!("{full_name}#{issue_number}"),
        url: Some(format!("{}/issues/{issue_number}", repository.html_url)),
    }
}
