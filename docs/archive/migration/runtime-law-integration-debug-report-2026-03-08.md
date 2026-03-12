# Runtime Governance Integration Debug Report

> Historical report: superseded by active boundary/consumption references in:
> - `docs/architecture/governance-consumption-model.md`
> - `docs/architecture/embedded-governance-surface.md`
> - `docs/architecture/governance-runtime-convergence-audit.md`
>
> Use this file as historical analysis only, not as primary runtime reference.
>
> Post-cutover update: `governance` is retired from active runtime and tooling paths.

Date: 2026-03-08
Scope: first governance-native integration pass in `yai`

## Executive verdict

`yai` is now governance-aware in runtime flow, but current implementation is an early operational scaffold.

- Strong: loader/classifier/discovery/resolver modules are present, build passes, tests run.
- Weak: discovery and stack build are still hardcoded around digital/scientific seed; semantic depth is limited.
- High risk: documentation and tooling are still split between new `governance/runtime-package` model and old `governance` references.

## Classification by area

### Root/docs identity

- `README.md`: **Approved as scaffold**
- `GOVERNANCE_COMPATIBILITY.md`: **Approved as skeleton**
- `docs/README.md`: **Approved as scaffold**
- `docs/architecture/governance-consumption-model.md`: **Approved as skeleton**
- `docs/architecture/embedded-governance-surface.md`: **Approved as skeleton**

Finding:
- Narrative is directionally correct, but large doc surfaces still reference `governance` as active normative anchor.

### governance/runtime-package

Status: **Duplicated** + **Needs cleanup**

Evidence:
- `governance/runtime-package` contains 391 files (94 markdown, 296 json).
- `governance` has 163 files, so embedded surface is currently larger than legacy mirror.
- Embedded includes many README/documentation artifacts; boundary is not yet “minimal runtime artifact surface”.

Judgment:
- Current embedded is closer to a structured copy than a compiled runtime package.

### governance

Status: **Legacy-contaminated**

Judgment: **still operationally relevant**

Evidence:
- Build/test/docs still reference `governance` heavily (`Makefile`, `tools/dev`, many docs under `docs/platform`, `docs/program`, `docs/interfaces`).
- Runtime loader prefers `governance/runtime-package` but explicitly falls back to `governance`.

### include/yai/governance and lib/governance

- `include/yai/governance/`: **Approved as scaffold**
- `lib/governance/loader`: **Approved as scaffold**
- `lib/governance/classification`: **Approved as scaffold**
- `lib/governance/discovery`: **Approved as scaffold**
- `lib/governance/resolution`: **Fragile**
- `lib/governance/mapping`: **Approved as skeleton**
- `lib/governance/debug`: **Approved as skeleton**

Key finding:
- Architecture split is clean, but resolver internals are mostly deterministic handcrafted logic for digital/scientific seed (not yet generic policy engine over full manifest/schema semantics).

### Enforcement handoff

Status: **Approved as scaffold**

Evidence:
- Handoff is wired in `lib/core/session/session.c` via `yai_law_resolve_control_call(...)`.
- Effects map into runtime reply paths with rationale/evidence trace.

Gap:
- Effect mapping is coarse and control-call centric; broader runtime execution surfaces are not yet integrated.

### Tests

- `tests/unit/governance/`: **Approved as scaffold**
- `tests/integration/law_resolution/`: **Approved as scaffold**

Coverage judgment:
- Good smoke baseline for digital/scientific seed path.
- Insufficient for ambiguity/fallback/conflict complexity and cross-domain compliance interactions.
- Mostly happy-path and binary decision assertions.

### Docs contamination (major)

Status: **Legacy-contaminated**

Evidence:
- Extensive `governance` references across `docs/archive/historical-architecture/platform/**`, `docs/program/**`, `docs/reference/**`, and generated alignment docs.
- Conflicts with new narrative that runtime consumption is embedded-first.

## Fault lines

1. Embedded surface not yet minimal
- Too much payload duplication, too many markdown artifacts.

2. Dual normative path in docs/tooling
- `governance/runtime-package` declared primary, while large portions still enforce/pin `governance` language.

3. Resolver semantics not yet data-driven enough
- Core behavior still encodes domain logic branches directly in runtime code.

## Bucket classification summary

### Approved and stable
- Module boundaries for governance consumer subsystem.
- Build/test wiring and basic runtime integration.

### Approved as scaffold
- Loader/classifier/discovery/resolution pipeline shape.
- digital/scientific seed integration smoke paths.

### Needs refoundation before verticalization
- Embedded export packaging model (compile/minimize surface).
- Docs/program lineage still tied to legacy governance pathing.

### Needs mass verticalization
- Resolver semantics beyond digital/scientific seed.
- Data-driven policy composition from manifests instead of hardcoded branches.

### Legacy contamination to remove
- `governance` references in docs/program/interfaces and helper tools.

### Narrative ambiguity to close
- One authoritative consumption model only: embedded-first, explicit legacy policy.

## Readiness for next block

Ready now:
- digital/scientific seed runtime verticalization continuation.

Not ready yet:
- broad domain expansion before resolving embedded-minimization and dual-path docs contamination.

Recommended order:
1. Embedded surface minimization + export hardening
2. Docs/tooling decontamination from `governance` in active paths
3. Resolver genericity improvements (domain/compliance composition via data)
4. Add non-happy-path tests (ambiguity/conflict/fallback)
