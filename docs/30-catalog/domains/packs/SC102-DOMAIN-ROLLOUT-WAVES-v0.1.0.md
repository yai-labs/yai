---
id: SC102-DOMAIN-ROLLOUT-WAVES-v0.1.0
status: draft
owner: catalog
effective_date: 2026-02-24
revision: 1
issue:
  - https://github.com/yai-labs/yai/issues/186
related:
  - docs/30-catalog/scenarios/SC-102.md
  - docs/40-qualification/QT-0.1-001-SC102/README.md
  - docs/30-catalog/domains/D-MAJOR.md
---

# SC-102 Domain Rollout Waves (D2..D9)

This document defines the post-D1 execution order for cross-domain SC-102 qualification.

## Preconditions
- D1 (`D1-digital/egress-v1`) has completed 3/3 coherent runs.
- Gate A workflow remains unchanged (`QT-0.1-001-SC102`).
- SKIP on required checks remains FAIL.

## Wave order
1. D5-economic / `transaction-authorization-v1`
2. D2-physical / `actuator-command-v1`
3. D8-scientific / `reproducibility-parameter-lock-v1`
4. D4-social-institutional / `procedural-authorization-v1`
5. D3-biological / `chain-of-custody-v1`
6. D7-cognitive-cultural / `publication-enforcement-v1`
7. D6-operational / `incident-response-action-v1`
8. D9-environmental-climatological / `environmental-telemetry-integrity-v1`

## Why this sequence
- D5 and D2 maximize policy/enforcement stress early.
- D8 validates reproducibility and evidence strictness before high-context domains.
- D4/D3 test procedural and consent/custody semantics.
- D7/D6 cover platform governance and operational containment.
- D9 closes with environmental telemetry integrity constraints.

## Per-pack acceptance (same invariants)
For each pack, all must pass:
- 3 coherent runs for the same pack+baseline
- forbidden effect blocked (fail-closed)
- decision record complete (`outcome`, `reason_code`, `baseline_hash`)
- evidence completeness 100%
- findings file published under `docs/50-validation/audits/findings/`

## Risk controls
- No concurrent multi-pack execution in the same wave.
- If a pack fails, wave does not advance.
- Runtime harness changes are locked per wave; only pack data changes.
