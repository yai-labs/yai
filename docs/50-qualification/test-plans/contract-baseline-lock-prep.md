# Test Plan (Prep) - Contract Baseline Lock

Purpose: prepare a strict but small baseline before the official test plan.

This is a preparation plan, not the final governance plan.

## Scope

- Baseline pin alignment checks (`yai` + `yai-cli`)
- Deterministic docs/60-validation/proof checks
- Minimal positive/negative integration checks for control path

## Preconditions

- `yai`, `yai-cli`, `yai-specs` repos available
- same target baseline commit identified for `deps/yai-specs`
- local build is clean enough to run verify commands

## Commands (minimum)

```bash
# In yai repo
tools/bin/yai-check-pins
tools/bin/yai-docs-trace-check --all
tools/bin/yai-proof-check --all
python3 tests/integration/test_handshake.py
```

## Positive checks

1. Pin check passes and reports aligned refs.
2. Trace/proof checks exit `0`.
3. Handshake test reaches root control socket and receives valid response envelopes.

## Negative checks

1. Simulate pin mismatch -> `check_pins.sh` must fail.
2. Remove/alter a required docs link -> trace check must fail.
3. Run handshake with missing control socket -> test returns explicit `SKIP` reason (deterministic behavior).

## Evidence to collect

- command + exit code
- commit SHA
- CI run link (if executed in pipeline)
- relevant log excerpt (max 20-30 lines per failure/success proof)

## Exit criteria (prep complete)

- checks are reproducible for 2 consecutive runs
- false positives/false negatives are understood
- commands are stable enough to be promoted into the official plan
