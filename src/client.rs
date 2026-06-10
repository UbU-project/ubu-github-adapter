use octocrab::Octocrab;

use crate::auth::GitHubAuth;
use crate::errors::Result;

#[derive(Clone)]
pub struct GitHubClient {
    octocrab: Octocrab,
}

impl GitHubClient {
    pub fn from_auth(auth: GitHubAuth) -> Result<Self> {
        let octocrab = Octocrab::builder()
            .personal_token(auth.token().expose().to_owned())
            .build()?;
        Ok(Self { octocrab })
    }

    pub fn octocrab(&self) -> &Octocrab {
        &self.octocrab
    }
}
