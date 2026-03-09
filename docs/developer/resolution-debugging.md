# Resolution Debugging

## Debug surfaces

`lib/law/debug/` provides lightweight runtime-inspection helpers:

- `dump_discovery_result.c`
- `dump_effective_stack.c`
- `resolution_trace.c`

## What to inspect first

1. Classification context values.
2. Discovery family candidates + selected family + confidence.
3. Specialization candidates + selected specialization.
4. Applied/skipped rules in trace output.
5. Overlay/compliance attachments in the stack.
6. Final effect mapping and rationale.

Trace payload now exposes `routing_mode`, `family_candidates`, and `specialization_candidates` for faster diagnosis of early routing decisions.
For second-half resolver debugging, inspect:
- `regulatory_overlay_count`, `sector_overlay_count`, `contextual_overlay_count`
- `authority_contributor_count`, `evidence_contributor_count`
- `authority_profile`, `evidence_profile`, `precedence_trace`

For workspace-centric debugging, correlate trace output with workspace inspect state:
- declared context fields
- inferred context fields
- effective summaries (`last_effect_summary`, `last_authority_summary`, `last_evidence_summary`)
- workspace surfaces (`yai.workspace.status`, `yai.workspace.inspect`, `yai.workspace.debug.resolution`)
- workspace execution macro (`yai ws run ...`) to reproduce inspectable post-action state

## Quick validation

Use the law-native suites:

```bash
make test-law
```

This executes unit coverage for loader/discovery/resolution and integration smoke for digital/economic/scientific paths, including overlay-driven cases.
