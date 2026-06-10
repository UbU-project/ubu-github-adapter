# Security

## Token Handling

The adapter accepts GitHub personal access tokens from `GITHUB_TOKEN` or an
orchestrator-supplied session token. Tokens are held in memory only.

The adapter must never:

- Write a token to disk.
- Log `Authorization` headers.
- Log `GITHUB_TOKEN`.
- Log pasted session tokens.
- Log octocrab authentication configuration.

Report suspected token exposure privately through the repository security
contact path once configured.
