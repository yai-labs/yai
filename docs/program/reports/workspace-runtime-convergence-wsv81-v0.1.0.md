---
id: workspace-runtime-convergence-wsv81-v0.1.0
status: active
owner: runtime-governance
updated: 2026-03-10
scope: [yai, yai-cli]
related:
  - tests/integration/workspace/workspace_runtime_reachability.sh
  - docs/program/reports/workspace-verticalization-wsv8-closure-plan-v0.1.0.md
---

# WSV-8.1 Runtime/Control-Plane Convergence Report (v0.1.0)

## What changed

Runtime host convergence hardening in `cmd/yai/main.c`:
- ingress path resolution now uses canonical runtime ingress resolver (`YAI_RUNTIME_INGRESS` supported consistently),
- runtime pidfile path now supports override (`YAI_RUNTIME_PIDFILE`) and follows ingress directory when overridden,
- ingress bind failure now emits an actionable hint with override env guidance,
- startup no longer prints `service is live` before ingress listener is actually bound.

API contract update:
- added `YAI_RUNTIME_PIDFILE_ENV` in `include/yai/api/runtime.h`.

## Verification

Green representative check:
- `tests/integration/workspace/workspace_runtime_reachability.sh`

Validated path:
1. start runtime host (`yai up`) with explicit ingress override
2. `yai lifecycle up` (CLI)
3. `yai runtime ping`
4. `yai ws status`
5. `yai ws inspect`
6. `yai ws graph summary`

Expected outcome enforced by script:
- ping/status/inspect are live (`rc=0`),
- rich family command is non-dead (`ws graph summary` returns `rc=0` or bounded `BAD_ARGS`, but not `SERVER_UNAVAILABLE`).

## Residual debt (for WSV-8.x)

- This step fixes runtime/control-plane convergence baseline only.
- It does not close:
  - full workspace persistence/materialization guarantees,
  - direct-backing for all composition-backed `ws db/recovery/knowledge` paths,
  - broader family richness closure and post-bind deep behavior.
