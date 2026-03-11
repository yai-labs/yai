# Build — YAI Mind (C Runtime)

## Requirements

- C toolchain (`cc`, `make`)

## Commands

- Build:
  - `make -C mind`
- Clean:
  - `make -C mind clean`
- Tests:
  - `make -C mind test`
- Optional run (daemon baseline):
  - `make -C mind run`

## Notes

- `mind/` is C-runtime primary.
- Rust/Cargo build path for Mind is decommissioned.
- Historical migration notes are under `mind/docs/archive/`.
