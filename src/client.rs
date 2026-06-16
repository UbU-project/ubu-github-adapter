use octocrab::Octocrab;
use serde_json::json;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::auth::GitHubAuth;
use crate::errors::Result;

pub type GitHubApiFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>;

pub trait GitHubApi: Send + Sync {
    fn create_label<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        label: &'a str,
        color: &'a str,
        description: &'a str,
    ) -> GitHubApiFuture<'a, ()>;

    fn add_labels_to_issue<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        issue_number: u64,
        labels: &'a [String],
    ) -> GitHubApiFuture<'a, ()>;

    fn remove_label_from_issue<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        issue_number: u64,
        label: &'a str,
    ) -> GitHubApiFuture<'a, ()>;

    fn create_comment<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        issue_number: u64,
        body: &'a str,
    ) -> GitHubApiFuture<'a, ()>;

    fn create_issue<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        title: &'a str,
        body: &'a str,
        labels: &'a [String],
    ) -> GitHubApiFuture<'a, ()>;

    fn issue_labels<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        issue_number: u64,
    ) -> GitHubApiFuture<'a, Vec<String>>;
}

#[derive(Clone)]
pub struct GitHubClient {
    api: Arc<dyn GitHubApi>,
}

impl GitHubClient {
    pub fn from_auth(auth: GitHubAuth) -> Result<Self> {
        let octocrab = Octocrab::builder()
            .personal_token(auth.token().expose().to_owned())
            .build()?;
        Ok(Self {
            api: Arc::new(OctocrabGitHubApi { octocrab }),
        })
    }

    pub fn from_api(api: Arc<dyn GitHubApi>) -> Self {
        Self { api }
    }

    pub fn api(&self) -> &dyn GitHubApi {
        self.api.as_ref()
    }
}

struct OctocrabGitHubApi {
    octocrab: Octocrab,
}

impl GitHubApi for OctocrabGitHubApi {
    fn create_label<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        label: &'a str,
        color: &'a str,
        description: &'a str,
    ) -> GitHubApiFuture<'a, ()> {
        Box::pin(async move {
            let route = format!("/repos/{owner}/{repo}/labels");
            let body = json!({
                "name": label,
                "color": color,
                "description": description,
            });
            let _: serde_json::Value = self.octocrab.post(route, Some(&body)).await?;
            Ok(())
        })
    }

    fn add_labels_to_issue<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        issue_number: u64,
        labels: &'a [String],
    ) -> GitHubApiFuture<'a, ()> {
        Box::pin(async move {
            if labels.is_empty() {
                return Ok(());
            }

            let route = format!("/repos/{owner}/{repo}/issues/{issue_number}/labels");
            let body = json!({ "labels": labels });
            let _: serde_json::Value = self.octocrab.post(route, Some(&body)).await?;
            Ok(())
        })
    }

    fn remove_label_from_issue<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        issue_number: u64,
        label: &'a str,
    ) -> GitHubApiFuture<'a, ()> {
        Box::pin(async move {
            let route = format!("/repos/{owner}/{repo}/issues/{issue_number}/labels/{label}");
            let _: serde_json::Value = self.octocrab.delete(route, None::<&()>).await?;
            Ok(())
        })
    }

    fn create_comment<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        issue_number: u64,
        body: &'a str,
    ) -> GitHubApiFuture<'a, ()> {
        Box::pin(async move {
            let route = format!("/repos/{owner}/{repo}/issues/{issue_number}/comments");
            let request = json!({ "body": body });
            let _: serde_json::Value = self.octocrab.post(route, Some(&request)).await?;
            Ok(())
        })
    }

    fn create_issue<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        title: &'a str,
        body: &'a str,
        labels: &'a [String],
    ) -> GitHubApiFuture<'a, ()> {
        Box::pin(async move {
            let route = format!("/repos/{owner}/{repo}/issues");
            let request = json!({
                "title": title,
                "body": body,
                "labels": labels,
            });
            let _: serde_json::Value = self.octocrab.post(route, Some(&request)).await?;
            Ok(())
        })
    }

    fn issue_labels<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
        issue_number: u64,
    ) -> GitHubApiFuture<'a, Vec<String>> {
        Box::pin(async move {
            let route = format!("/repos/{owner}/{repo}/issues/{issue_number}/labels");
            let response: Vec<GitHubLabelResponse> = self.octocrab.get(route, None::<&()>).await?;
            Ok(response.into_iter().map(|label| label.name).collect())
        })
    }
}

#[derive(serde::Deserialize)]
struct GitHubLabelResponse {
    name: String,
}
