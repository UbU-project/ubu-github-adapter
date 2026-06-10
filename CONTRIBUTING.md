# Contributing

Run the local checks before opening a pull request:

```sh
cargo fmt --check
cargo clippy --all-targets
cargo test
```

Keep fixture tests deterministic. Do not add tests that require a live GitHub
token by default.

Never commit `.cargo/config.toml`; local sibling patches belong only in the
ignored developer environment.
