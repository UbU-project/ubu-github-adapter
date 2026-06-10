# ubu-github-adapter

Rust GitHub adapter for UbU Phase 1.

This crate imports GitHub repository state, normalizes issues, pull requests,
reviews, CI events, labels, milestones, and comments, maps imported state into
UbU candidate objects, creates projection previews, and applies explicitly
approved UbU-managed projection writes through `octocrab`.

## Phase 1 Boundaries

- Authentication is token based. Developer mode reads `GITHUB_TOKEN`; desktop
  session mode accepts an in-memory token supplied by the orchestrator.
- Tokens are never persisted and must never be logged.
- Live writes require a `ProjectionApproval` for the exact preview batch.
- Allowed writes are limited to applying/removing UbU-managed labels, creating
  UbU-managed comments, creating UbU-managed issues, and creating the two managed
  labels `ubu` and `ubu-managed` after explicit batch approval.
- The adapter never mutates the canonical store directly.

## Local Checks

```sh
cargo fmt --check
cargo clippy --all-targets
cargo test
```

TODO: Restore `-D warnings` once the public contract and generated schemas
stabilize.
