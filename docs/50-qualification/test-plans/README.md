# Test Plans

This directory defines evidence expectations for delivery artifacts.

A test plan should make verification reproducible and reviewable.

## Minimum content of a test plan

- positive checks,
- negative checks,
- exact commands,
- expected outputs/outcomes,
- notes on determinism and failure behavior.

## Role in the spine

- upstream: constrained by ADR/runbook intent,
- downstream: referenced by milestone packs and CI evidence.

## Scope boundary

Test plans describe how to validate; they do not replace normative contract/spec definitions.

## Suggested progression

- Start with a preparation plan: `docs/50-qualification/test-plans/contract-baseline-lock-prep.md`
- Promote to an official plan only after commands/evidence are stable in CI.
