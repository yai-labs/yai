# Enforcement Flow

## Governance-native handoff

Runtime command handling now performs a governance resolution step before enforcement behavior is finalized.

Current handoff point:

- `lib/core/session/session.c`
- `yai_session_handle_control_call(...)`

## Flow

1. Build classification context from control payload.
2. Run domain discovery.
3. Build effective normative stack.
4. Resolve final effect + rationale.
5. Map effect to runtime response contract.
6. Return decision/evidence/trace payload fields.

## Effect mapping

- `deny` and `quarantine` map to policy-blocked status.
- `review_required` maps to explicit review-needed status.
- non-blocking effects remain `ok` with rationale attached.

This keeps enforcement decisions driven by resolved governance output instead of ad-hoc local policy branches.

## Event surface preservation

Enforcement is no longer the only semantic shown to operators.
Workspace surfaces preserve three parallel fields:

- declared scenario specialization
- business specialization
- enforcement specialization

When enforcement falls back to `network-egress`, event surfaces still preserve business scenario identity for inspect/debug/effective views.

Reference: `docs/architecture/workspace-event-surface-model.md`.
