# Build — YAI Mind

## Requirements

- Rust stable toolchain
- rustfmt + clippy components

## Commands

- Format check:
  - `cargo fmt --check`
- Lint:
  - `cargo clippy -- -D warnings`
- Tests:
  - `cargo test --all --locked`

Convenience:
- `make check` runs fmt + clippy + tests.

## Notes

- This repo is intended to integrate with the wider YAI runtime.
- Build scripts and contracts must remain compatible with the canonical specs in `yai-law`.
