use ubu_core::{AuthoritySource, UbuId};
use ubu_github_adapter::candidate_mapping::map_repository_state;
use ubu_github_adapter::fixture::GitHubFixture;
use ubu_github_adapter::normalize::normalize_fixture;

#[test]
fn maps_candidates_with_canonical_task_ids_and_source_urls() {
    let fixture = GitHubFixture::from_path("fixtures/github/issues-small.json").unwrap();
    let normalized = normalize_fixture(fixture);
    let mapped = map_repository_state(&normalized).unwrap();

    let candidate = &mapped.candidates[0];
    assert!(candidate.candidate_id.starts_with("task_"));
    UbuId::parse(&candidate.candidate_id).unwrap();
    assert_eq!(candidate.authority_source, AuthoritySource::Delegated);
    assert_eq!(
        mapped.external_references[0].source.url.as_deref(),
        Some("https://github.com/UbU-project/ubu-github-adapter/issues/7")
    );
}
