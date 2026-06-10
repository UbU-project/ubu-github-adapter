use std::path::Path;

use crate::candidate_mapping::{map_repository_state, CandidateMapping};
use crate::errors::Result;
use crate::fixture::GitHubFixture;
use crate::normalize::normalize_fixture;

pub fn import_fixture(path: impl AsRef<Path>) -> Result<CandidateMapping> {
    let fixture = GitHubFixture::from_path(path)?;
    let normalized = normalize_fixture(fixture);
    map_repository_state(&normalized)
}
