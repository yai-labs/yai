# Stub Gap Analysis

## Stub/Placeholder Components Identified
- `GOVERNANCE_PRECEDENCE.tla`: placeholder only.
- `GOVERNANCE_RESOLUTION.tla`: placeholder only.
- `formal/schema/resolution_trace.v1.schema.json`: unconstrained object.
- Traceability references to non-canonical split-era paths.

## Runtime Gap Side
- Runtime policy/grants/containment were previously declared but weakly represented.
- Formal layer had no explicit invariant mapping to runtime enforcement decisions.

## Closure
- Placeholder TLA modules moved to `formal/legacy/tla/`.
- Canonical replacements added as concrete modules (`yai_resolution`, `yai_policy_application`, `yai_grants`, `yai_containment`).
- Runtime enforcement bridge map added under `formal/traceability/`.
