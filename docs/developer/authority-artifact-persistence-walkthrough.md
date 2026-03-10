# Authority and Artifact Persistence Walkthrough (DP-6)

## Goal
Verify authority state and artifact metadata are persisted as live Data Plane entities during a governed workspace action.

## Scenario
1. Create workspace `ws_authority_artifact_dp6_v1`.
2. Set domain `digital/remote-publication`.
3. Attach and activate approved governance object `enterprise.ecohmedia.digital-outbound.review-gate`.
4. Run `digital.publish sink=external_untrusted contract=missing artifact=bundle-v1`.
5. Inspect persisted authority/artifact sinks and workspace surfaces.

## Persisted sinks (workspace scoped)
- `~/.yai/run/<ws>/authority/authority-state.v1.ndjson`
- `~/.yai/run/<ws>/authority/resolution-state.v1.ndjson`
- `~/.yai/run/<ws>/authority/index.v1.json`
- `~/.yai/run/<ws>/artifacts/metadata.v1.ndjson`
- `~/.yai/run/<ws>/artifacts/linkage.v1.ndjson`
- `~/.yai/run/<ws>/artifacts/metadata.index.v1.json`

## What to verify
- Authority records exist with workspace and governance refs.
- Artifact metadata exists with governance/authority/decision/evidence refs.
- Index records expose latest typed refs (`auth-*`, `ares-*`, `art-*`, `alink-*`).
- `workspace.inspect`, `workspace.policy_effective`, `workspace.debug_resolution`
  expose `authority_artifact_persistence` block with store refs and latest refs.

## Why it matters
This closes the DP gap where authority and artifact context were mostly inferred from runtime output. They are now persisted, typed, and query-ready for DP-8.
