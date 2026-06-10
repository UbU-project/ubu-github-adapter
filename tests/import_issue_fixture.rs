use ubu_github_adapter::candidate_mapping::map_repository_state;
use ubu_github_adapter::fixture::GitHubFixture;
use ubu_github_adapter::normalize::normalize_fixture;

#[test]
fn imports_issue_fixture() {
    let fixture = GitHubFixture::from_path("fixtures/github/issues-small.json").unwrap();
    let normalized = normalize_fixture(fixture);
    assert_eq!(normalized.issues.len(), 1);
    assert_eq!(normalized.comments.len(), 1);

    let mapped = map_repository_state(&normalized).unwrap();
    assert_eq!(mapped.tasks.len(), 1);
    assert_eq!(
        mapped.external_references[0].source.source_id,
        "UbU-project/ubu-github-adapter#7"
    );
}
