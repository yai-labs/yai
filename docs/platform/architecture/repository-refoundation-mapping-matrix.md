# Repository Refoundation Mapping Matrix (Initial)

Action vocabulary (canonical):
- `keep`
- `move`
- `rename`
- `merge`
- `split`
- `remove`
- `move-to-tests`

This matrix is intentionally high-signal and not yet exhaustive at single-file granularity.

| Legacy path / domain | Target path / domain | Action | Rationale | Notes / follow-up |
|---|---|---|---|---|
| `boot/` | `lib/core/lifecycle/` | `merge` | Boot concerns become runtime lifecycle concerns under sovereign core | file-level split in wave 5 |
| `root/` | `lib/core/authority/`, `lib/core/dispatch/` | `split` | preserve authority and control dispatch semantics without historical package name | validate command routing ownership |
| `kernel/` | `lib/core/`, `lib/protocol/`, `lib/support/` | `split` | kernel folder mixes sovereign runtime and cross-cutting concerns | classify by semantic owner |
| `engine/` | `lib/exec/` | `move` | execution plane converges under exec model | rename legacy names during move |
| `mind/` | `lib/brain/` + `tests/*` | `split` | cognitive runtime stays internal module; mocks/helpers move to tests | preserve runtime-only code path |
| `runtime-protocol/` | `lib/protocol/` | `move` | protocol is foundation layer, not a top-level legacy package | consolidate rpc/codec/binary APIs |
| scattered mains under legacy domains | `cmd/yai/`, `cmd/yai-core/` | `merge` | enforce 2-binary minimal topology | no extra binaries by default |
| legacy include trees | `include/yai/{core,exec,brain,protocol,platform,support,api}/` | `merge` | single include grammar avoids duplicated public surface | enforce internal header discipline |
| `mind/tests_c/*` | `tests/unit/brain/*` and `tests/integration/*` | `move` | tests should follow domain-first testing topology | keep smoke behavior coverage |
| `tests/domains/*` | `tests/unit/*`, `tests/integration/*` | `move` | remove legacy-domain test packaging drift | classify by test intent |
| `build/*` (artifacts) | `build/*` (artifacts only) | `keep` | build remains artifact tree, not source domain | enforce ignore and no-source rule |
| `docs/platform/architecture/components/{boot,root,kernel,engine,mind}.md` | architecture + migration docs | `keep` | historical and bridge value | mark non-canonical for packaging decisions where needed |
| top-level legacy domains after migration completion | removed | `remove` | final state must expose new topology only | scheduled wave 10 |
