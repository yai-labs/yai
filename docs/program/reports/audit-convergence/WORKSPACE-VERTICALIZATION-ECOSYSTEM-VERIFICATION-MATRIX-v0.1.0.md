---
id: WORKSPACE-VERTICALIZATION-ECOSYSTEM-VERIFICATION-MATRIX-v0.1.0
status: active
owner: runtime-governance
updated: 2026-03-10
scope: [yai, yai-law, yai-cli, yai-sdk]
related:
  - docs/program/milestone-packs/runtime-baselines/workspace-command-taxonomy-refoundation.md
  - docs/program/milestone-packs/runtime-baselines/workspace-runtime-command-mapping-and-canonicalization.md
  - docs/program/milestone-packs/workspace-verticalization-closeout/WSV-6-WORKSPACE-SURFACE-CLOSEOUT.md
  - docs/program/reports/audit-convergence/WORKSPACE-VERTICALIZATION-MANUAL-TEST-COMMAND-PACK-v0.1.0.md
  - tests/integration/workspace/workspace_verticalization_closeout.sh
---

# Workspace Verticalization Ecosystem Verification Matrix (v0.1.0)

Legend: `PASS`, `PASS WITH DEBT`, `FAIL`

| Domain | Repo(s) | Canonical truth expected | Automatic check / verification method | Manual check / operator path | Evidence expected | Result | Residual issue / blocker | Owner / follow-up |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| WSV taxonomy alignment | yai, yai-law, yai-cli, yai-sdk | Same `ws` families: lifecycle, graph, db, data, knowledge, policy, domain, recovery, debug, query fallback | `tests/integration/workspace/workspace_verticalization_closeout.sh` | `./dist/bin/yai help ws` in `cli` | Script `ok`; help shows all families | PASS | none | closed |
| Runtime lifecycle/binding substrate | yai | Runtime exposes `create/open/set/switch/current/status/inspect/unset/clear/reset/destroy` IDs | `rg` in `lib/core/session/session.c` (scripted in WSV6 guardrail) | `yai ws create/open/set/switch/current/status/inspect/unset/clear/reset/destroy` | IDs present in dispatch and handlers | PASS | none | closed |
| Runtime graph substrate | yai | Runtime exposes `graph summary/workspace/governance/decision/evidence/authority/artifact/lineage/recent` | `rg "yai.workspace.graph.*"` in `lib/core/session/session.c` | `yai ws graph ...` | IDs present + query-family routing | PASS | none | closed |
| Runtime db/data/knowledge/recovery/debug substrate | yai | Runtime exposes query/events tail + policy/domain/debug + recovery basis | `rg` for `workspace.query/events.tail/policy_*/domain_*/debug_resolution/open/lifecycle.maintain` | `yai ws db ...`, `yai ws data ...`, `yai ws knowledge ...`, `yai ws recovery ...`, `yai ws debug resolution` | IDs present in dispatch | PASS WITH DEBT | Several ws-db/ws-knowledge/ws-recovery commands are composition-backed, not all direct runtime IDs | next hardening wave |
| Law canonical lifecycle/binding entries | yai-law | Registry has `ws general` ops for lifecycle/binding | Python check in `workspace_verticalization_closeout.sh` | `python3` query over `law/registry/commands.v1.json` | topic `general` includes required ops | PASS | none | closed |
| Law canonical graph entries | yai-law | Registry has `ws graph` ops: summary/workspace/governance/decision/evidence/authority/artifact/lineage/recent | Same WSV6 script topic/op check | inspect registry rows for `topic=graph` | all ops present | PASS | none | closed |
| Law canonical db entries | yai-law | Registry has `ws db` ops: status/bindings/stores/classes/count/tail | Same WSV6 script topic/op check | inspect registry rows for `topic=db` | all ops present | PASS WITH DEBT | Some db entries model composition over inspect/query, not direct backend command IDs | follow-up for direct IDs |
| Law canonical data entries | yai-law | Registry has `ws data` ops: events/evidence/governance/authority/artifacts/enforcement | Same WSV6 script topic/op check | inspect registry rows for `topic=data` | all ops present | PASS | none | closed |
| Law canonical knowledge entries | yai-law | Registry has `ws knowledge` ops: status/transient/memory/providers/context | Same WSV6 script topic/op check | inspect registry rows for `topic=knowledge` | all ops present | PASS WITH DEBT | providers/context depend on query substrate behavior; not uniformly direct-backed | next wave |
| Law canonical policy/domain/recovery/debug/query entries | yai-law | `ws policy`, `ws domain`, `ws recovery`, `ws debug`, `ws query` are represented | Same WSV6 script topic/op check | inspect registry rows by topic | all ops present | PASS WITH DEBT | Recovery currently mapped through composed runtime surfaces | next wave |
| CLI taxonomy/help alignment | yai-cli | `help ws` and nested family help expose canonical hierarchy | `make test` in `cli`; `tests/integration/help_guardrail.sh` + WSV6 script | `yai help ws`, `yai help ws graph/db/data/knowledge/policy/domain/recovery/debug/query` | Guardrails green; nested usage strings visible | PASS | none | closed |
| CLI parser/routing alignment | yai-cli | Representative command routes exist for each family | `make test` in `cli`; parse/unit + workspace output guardrails | Run representative `yai ws graph summary`, `yai ws db status`, `yai ws data evidence`, `yai ws knowledge transient`, `yai ws recovery status`, `yai ws debug resolution` | Command parsing succeeds; runtime response or explicit protocol/server error | PASS WITH DEBT | Runtime-up dependency for full behavioral pass; 일부 calls may return protocol/server unavailable without runtime | non-blocking for closeout |
| SDK typed workspace surface presence | yai-sdk | Public headers expose typed helpers for graph/db/data/knowledge/policy/domain/recovery/debug + lifecycle | `make test` in `sdk`; `tests/public_surface_smoke.c`; `tests/workspace_typed_surface_smoke.c`; WSV6 script header checks | Build and inspect `include/yai_sdk/public.h` | New headers exported and helpers compiled | PASS | none | closed |
| SDK family helper behavior | yai-sdk | Representative typed calls map to canonical ws families | `make test` in `sdk`; `workspace_typed_surface_smoke` | Run `example_04_workspace_verticalized` | BAD_ARGS on null client path + compile-safe typed mapping | PASS WITH DEBT | Behavioral e2e depends on live runtime; some helpers composition-backed | non-blocking |
| Workspace lifecycle manual operator readiness | yai-cli, yai | Human can run lifecycle/create/set/status/inspect paths | Manual pack section A/B | Copy-paste command blocks | Observable binding/readiness fields in output | PASS WITH DEBT | Requires runtime up and law compatibility path configured | operator prerequisite |
| Graph manual operator readiness | yai-cli, yai | Human can run graph family commands and inspect graph artifacts | Manual pack section C + filesystem/duckdb checks | `yai ws graph ...`; `tail` graph ndjson files | Graph query responses and persisted graph artifacts | PASS WITH DEBT | Some environments return protocol/server unavailable if runtime not up | non-blocking |
| DB manual operator readiness | yai-cli, yai | Human can run db family and verify data-plane files/tables | Manual pack section D + duckdb queries | `yai ws db ...`; `duckdb -readonly ...` | DB bindings, classes/count, persisted records | PASS WITH DEBT | db family is partially composition-backed | follow-up hardening |
| Data/knowledge/policy/domain/recovery/debug manual readiness | yai-cli, yai-law, yai | Human can run family commands and interpret results | Manual pack sections E/F/G/H | `yai ws data ...`, `knowledge ...`, `policy ...`, `domain ...`, `recovery ...`, `debug ...` | Structured output with effect/authority/evidence/context/recovery | PASS WITH DEBT | policy/debug may return protocol error when runtime endpoint is down | non-blocking |
| Query fallback containment | yai-cli, yai-sdk | `ws query <family>` remains available but non-primary | CLI help note + SDK `yai_sdk_ws_query_family` | `yai ws query evidence` | Fallback path works; dedicated families preferred in docs | PASS | none | closed |
| Cross-repo automated representative coverage | yai, yai-cli, yai-sdk | At least one representative check for lifecycle/graph/db/data/knowledge/policy/recovery-debug/sdk/law alignment | `workspace_verticalization_closeout.sh` + `cli make test` + `sdk make test` | Run three command blocks from manual pack section `Fast sanity` | Script green + suites green | PASS WITH DEBT | yai legacy deep integration scripts may fail in offline/sandbox contexts | next hardening wave |

## Verification disposition

- **Wave result**: `PASS WITH DEBT`
- **Blocking contradictions**: none found in canonical paths.
- **Primary non-blocking debt**:
  - composition-backed db/recovery/knowledge subpaths,
  - runtime-availability dependence for full manual e2e.
