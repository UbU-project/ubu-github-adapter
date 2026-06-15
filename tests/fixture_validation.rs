use std::fs;

use ubu_github_adapter::fixture::GitHubFixture;
use ubu_github_adapter::projection::operations::GitHubProjectionOperation;

#[test]
fn github_fixtures_validate() {
    for path in [
        "fixtures/github/ci-small.json",
        "fixtures/github/issues-small.json",
        "fixtures/github/prs-small.json",
        "fixtures/github/repo-small.json",
    ] {
        GitHubFixture::from_path(path)
            .unwrap_or_else(|err| panic!("{path} should deserialize: {err}"));
    }
}

#[test]
fn projection_fixtures_validate() {
    for path in [
        "fixtures/projection/comment-preview.json",
        "fixtures/projection/label-preview.json",
        "fixtures/projection/managed-issue-preview.json",
    ] {
        let contents =
            fs::read_to_string(path).unwrap_or_else(|err| panic!("{path} should read: {err}"));
        serde_json::from_str::<GitHubProjectionOperation>(&contents)
            .unwrap_or_else(|err| panic!("{path} should deserialize: {err}"));
    }
}
