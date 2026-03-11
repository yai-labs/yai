# Enforcement Record Closure Walkthrough (DP-10)

This walkthrough validates that a governed runtime outcome is materialized as a
full persisted record set (not only runtime output).

## Scenario
- Workspace family: `digital`
- Specialization: `remote-publication`
- Action: `digital.publish`
- Expected governed outcome: `deny` or `review_required`

## Steps

1. Bind workspace and run governed action.
2. Check event/evidence sinks exist under `~/.yai/run/<ws>/events/`.
3. Check enforcement sinks exist under `~/.yai/run/<ws>/enforcement/`.
4. Verify index `materialization_status` and `missing_fields`.
5. Verify runtime surfaces expose the same status:
   - `yai.workspace.policy_effective`
   - `yai.workspace.inspect`
   - `yai.workspace.debug_resolution`
   - `yai.workspace.query enforcement`

## What to expect

### Complete path
- `materialization_status: complete`
- `missing_fields: ""` (or `none`)
- typed refs present for event/decision/evidence and enforcement outcome/linkage.

### Forced partial path (DP-10 test mode)
- `materialization_status: incomplete`
- `missing_fields` non-empty
- runtime surfaces report incomplete state explicitly.

## Smoke command

Use:
- `tests/integration/workspace/workspace_enforcement_record_closure.sh`

The smoke executes both:
1. complete baseline
2. forced-partial baseline (`YAI_ENFORCEMENT_RECORD_FORCE_PARTIAL=1`)

## Why this matters

DP-10 makes runtime a disciplined writer of its own data plane reality.
This is the prerequisite for:
- DP-11 graph materialization from trusted refs
- DP-12 DB-first read path over canonical persisted records.
