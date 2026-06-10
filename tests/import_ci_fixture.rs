use ubu_core::core::TaskStatus;
use ubu_github_adapter::candidate_mapping::map_repository_state;
use ubu_github_adapter::fixture::GitHubFixture;
use ubu_github_adapter::normalize::normalize_fixture;

#[test]
fn imports_ci_fixture() {
    let fixture = GitHubFixture::from_path("fixtures/github/ci-small.json").unwrap();
    let normalized = normalize_fixture(fixture);
    let mapped = map_repository_state(&normalized).unwrap();

    assert_eq!(mapped.tasks.len(), 1);
    assert_eq!(mapped.tasks[0].status, TaskStatus::Completed);
}
