use serde_json::json;
use ubu_core::core::{ExternalReference, Task, TaskStatus};
use ubu_core::store::CandidateObject;
use ubu_core::{AuthoritySource, ObjectType, Provenance, SourceRef, UbuId, UbuTimestamp};

use crate::errors::Result;
use crate::normalize::NormalizedRepositoryState;
use crate::sources::{
    GitHubCiConclusion, GitHubCiEventSource, GitHubCiStatus, GitHubIssueSource, GitHubIssueState,
    GitHubPullRequestSource, GitHubPullRequestState,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CandidateMapping {
    pub external_references: Vec<ExternalReference>,
    pub tasks: Vec<Task>,
    pub candidates: Vec<CandidateObject>,
}

pub fn map_repository_state(state: &NormalizedRepositoryState) -> Result<CandidateMapping> {
    let mut external_references = Vec::new();
    let mut tasks = Vec::new();
    let mut candidates = Vec::new();

    for issue in &state.issues {
        let (reference, task, candidate) = map_issue(issue)?;
        external_references.push(reference);
        tasks.push(task);
        candidates.push(candidate);
    }

    for pull_request in &state.pull_requests {
        let (reference, task, candidate) = map_pull_request(pull_request)?;
        external_references.push(reference);
        tasks.push(task);
        candidates.push(candidate);
    }

    for ci_event in &state.ci_events {
        let (reference, task, candidate) = map_ci_event(ci_event)?;
        external_references.push(reference);
        tasks.push(task);
        candidates.push(candidate);
    }

    Ok(CandidateMapping {
        external_references,
        tasks,
        candidates,
    })
}

pub fn map_issue(issue: &GitHubIssueSource) -> Result<(ExternalReference, Task, CandidateObject)> {
    let source = source_ref(
        "github_issue",
        format!("{}#{}", issue.repository, issue.number),
        &issue.html_url,
    );
    let observed_at = issue.updated_at;
    let reference = external_reference(issue.title.clone(), source.clone(), observed_at);
    let status = match issue.state {
        GitHubIssueState::Open => TaskStatus::Proposed,
        GitHubIssueState::Closed => TaskStatus::Completed,
    };
    let task = task(
        issue.title.clone(),
        issue.body.clone(),
        status,
        source,
        observed_at,
    );
    let candidate = candidate_for_task(&task, observed_at)?;
    Ok((reference, task, candidate))
}

pub fn map_pull_request(
    pull_request: &GitHubPullRequestSource,
) -> Result<(ExternalReference, Task, CandidateObject)> {
    let source = source_ref(
        "github_pull_request",
        format!("{}#{}", pull_request.repository, pull_request.number),
        &pull_request.html_url,
    );
    let observed_at = pull_request.updated_at;
    let reference = external_reference(pull_request.title.clone(), source.clone(), observed_at);
    let status = match pull_request.state {
        GitHubPullRequestState::Open => TaskStatus::InProgress,
        GitHubPullRequestState::Closed | GitHubPullRequestState::Merged => TaskStatus::Completed,
    };
    let task = task(
        pull_request.title.clone(),
        pull_request.body.clone(),
        status,
        source,
        observed_at,
    );
    let candidate = candidate_for_task(&task, observed_at)?;
    Ok((reference, task, candidate))
}

pub fn map_ci_event(
    ci_event: &GitHubCiEventSource,
) -> Result<(ExternalReference, Task, CandidateObject)> {
    let source = source_ref(
        "github_ci_event",
        format!("{}#{}", ci_event.repository, ci_event.run_id),
        &ci_event.html_url,
    );
    let observed_at = ci_event.updated_at;
    let title = format!("CI: {}", ci_event.workflow_name);
    let reference = external_reference(title.clone(), source.clone(), observed_at);
    let status = match (ci_event.status, ci_event.conclusion) {
        (GitHubCiStatus::Completed, Some(GitHubCiConclusion::Success)) => TaskStatus::Completed,
        (
            GitHubCiStatus::Completed,
            Some(GitHubCiConclusion::Failure | GitHubCiConclusion::TimedOut),
        ) => TaskStatus::Blocked,
        (GitHubCiStatus::Completed, _) => TaskStatus::Canceled,
        (GitHubCiStatus::Queued | GitHubCiStatus::InProgress, _) => TaskStatus::InProgress,
    };
    let description = Some(format!(
        "GitHub Actions run {} is {:?}.",
        ci_event.run_id, ci_event.status
    ));
    let task = task(title, description, status, source, observed_at);
    let candidate = candidate_for_task(&task, observed_at)?;
    Ok((reference, task, candidate))
}

fn external_reference(
    title: String,
    source: SourceRef,
    observed_at: UbuTimestamp,
) -> ExternalReference {
    ExternalReference {
        id: UbuId::new(ObjectType::ExternalReference),
        source: source.clone(),
        title,
        observed_at,
        provenance: provenance(source, observed_at),
    }
}

fn task(
    title: String,
    description: Option<String>,
    status: TaskStatus,
    source: SourceRef,
    observed_at: UbuTimestamp,
) -> Task {
    Task {
        id: UbuId::new(ObjectType::Task),
        title,
        description,
        status,
        objective_id: None,
        due_at: None,
        provenance: provenance(source, observed_at),
    }
}

fn candidate_for_task(task: &Task, submitted_at: UbuTimestamp) -> Result<CandidateObject> {
    Ok(CandidateObject {
        candidate_id: task.id.to_string(),
        object_type: "Task".to_owned(),
        payload: serde_json::to_value(task)?,
        submitted_at,
        authority_source: AuthoritySource::Delegated,
    })
}

fn provenance(source: SourceRef, created_at: UbuTimestamp) -> Provenance {
    Provenance {
        created_at,
        created_by: Some("github".to_owned()),
        authority_source: AuthoritySource::Delegated,
        source: Some(source),
    }
}

fn source_ref(source_kind: &str, source_id: String, url: &str) -> SourceRef {
    SourceRef {
        source_kind: source_kind.to_owned(),
        source_id,
        url: Some(url.to_owned()),
    }
}

pub fn candidate_metadata(task: &Task) -> serde_json::Value {
    json!({
        "taskId": task.id,
        "title": task.title,
        "status": task.status,
    })
}
