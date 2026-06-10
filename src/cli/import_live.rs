use crate::client::GitHubClient;
use crate::errors::Result;
use crate::normalize::NormalizedRepositoryState;

pub async fn import_live_repository(
    _client: &GitHubClient,
    _owner: &str,
    _repo: &str,
) -> Result<NormalizedRepositoryState> {
    todo!("live import will be implemented after fixture import contract stabilizes")
}
