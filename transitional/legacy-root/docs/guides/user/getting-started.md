---
role: support
status: active
audience: user
owner_domain: guides
---

# Getting Started

# Purpose
Provide a single onboarding path for users and evaluators.

# Scope
Covers first install, first command, and first verification of the YAI surface.

# Relationships
- `cmd/yai/main.c`
- `include/yai/runtime/`
- `tests/`

# Canonical Role
Primary user onboarding guide.

# Main Body
## Install
- Build CLI/runtime entrypoint from repo root:
  - `make -j4`
- Confirm binary path:
  - `./build/yai --help` or repository equivalent command wrapper.

## Quickstart
- Read architecture entry:
  - `docs/architecture/overview/system-overview.md`
- Run minimal checks:
  - `make test`
- Open command reference:
  - `docs/reference/commands/surface.md`

## FAQ
- If a command differs from docs, command behavior in `cmd/` and reference docs wins.
- Operational procedures are in `docs/runbooks/`, not in user guides.

# Related Docs
- `docs/guides/user/guide/README.md`
- `docs/reference/README.md`
