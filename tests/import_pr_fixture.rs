use ubu_github_adapter::candidate_mapping::map_repository_state;
use ubu_github_adapter::fixture::GitHubFixture;
use ubu_github_adapter::normalize::normalize_fixture;

#[test]
fn imports_pr_fixture() {
    let fixture = GitHubFixture::from_path("fixtures/github/prs-small.json").unwrap();
    let normalized = normalize_fixture(fixture);
    assert_eq!(normalized.pull_requests.len(), 1);
    assert_eq!(normalized.reviews.len(), 1);

    let mapped = map_repository_state(&normalized).unwrap();
    assert_eq!(mapped.tasks[0].title, "Add projection previews");
}
