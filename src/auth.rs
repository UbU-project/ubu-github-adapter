use std::env;
use std::fmt;

use crate::errors::{AdapterError, Result};

#[derive(Clone)]
pub struct GitHubToken {
    value: String,
}

impl GitHubToken {
    pub fn from_env() -> Result<Self> {
        let value = env::var("GITHUB_TOKEN").map_err(|_| AdapterError::MissingToken {
            variable: "GITHUB_TOKEN",
        })?;
        Self::from_session(value)
    }

    pub fn from_session(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(AdapterError::MissingToken {
                variable: "session_token",
            });
        }
        Ok(Self { value })
    }

    pub(crate) fn expose(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for GitHubToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GitHubToken")
            .field("value", &"<redacted>")
            .finish()
    }
}

#[derive(Debug, Clone)]
pub enum GitHubAuth {
    PersonalAccessToken(GitHubToken),
}

impl GitHubAuth {
    pub fn from_env() -> Result<Self> {
        Ok(Self::PersonalAccessToken(GitHubToken::from_env()?))
    }

    pub fn from_session_token(value: impl Into<String>) -> Result<Self> {
        Ok(Self::PersonalAccessToken(GitHubToken::from_session(value)?))
    }

    pub(crate) fn token(&self) -> &GitHubToken {
        match self {
            Self::PersonalAccessToken(token) => token,
        }
    }
}
