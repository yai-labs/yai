---
id: DOMAIN-PACKS
title: Domain Packs - Specification & Layout
status: draft
owner: catalog
effective_date: 2026-02-23
revision: 1
---

# Domain Packs

Domain Packs are versioned semantic packages that allow YAI to operate coherently across different D-Major domains without changing the runtime.

A Domain Pack defines:
- what a forbidden effect means in a given domain,
- what authority/contract constraints apply,
- which decision outcomes and reason codes must be emitted,
- which evidence fields are required,
- which KPIs must be measured,
- and which safe vectors (stimuli) can be used for qualification runs.

Domain Packs are not runbooks, not audits, and not implementation code.
They are the semantic local law + test vectors used by scenarios (SC) and qualification gates (QT).

## Relationship to other artifacts

- D-Major Domains: semantic taxonomy (`docs/40-catalog/domains/D-MAJOR.md`)
- SC (Scenario Spec): declares the scenario goal and invariants (Catalog)
- QT (Qualification Test): executable harness that runs a scenario with one or more packs
- ER (Evidence Run): a single execution output (evidence artifacts)
- PP (Proof Pack): curated bundle for release qualification
- POC (PoC Pack): public/customer-facing bundle

A Domain Pack is referenced by `domain_pack_id`, for example:
- `D1-digital/egress-v1`
- `D5-economic/transaction-authorization-v1`

## Naming rules

### D-Major folder name

Must follow:
- `D1-digital`
- `D2-physical`
- `D3-biological`
- `D4-social-institutional`
- `D5-economic`
- `D6-operational`
- `D7-cognitive-cultural`
- `D8-scientific`
- `D9-environmental-climatological`

### Pack folder name

Must follow:
- `<pack-name>-v<integer>`

Examples:
- `egress-v1`
- `actuator-command-v1`
- `reproducibility-parameter-lock-v1`

### Pack ID

Pack ID is always:
`<Dk-folder>/<pack-folder>`

Example:
`D1-digital/egress-v1`

## Standard layout (minimum)

Every pack MUST include the following files:

```text
<Dk>/
  <pack-id>/
    pack.meta.json
    reason-codes.v1.json
    evidence.required.v1.json
    kpi.v1.json
    contracts/
      baseline-allow.json
      baseline-deny.json
      baseline-quarantine.json
    vectors/
      attack_profile.safe.json
      expected_outcomes.json
```

### File roles

#### `pack.meta.json` (required)

Defines identity and semantics:
- `id`, `d_major`, `title`, `version`
- `forbidden_effects[]` (domain semantics)
- `primary_runtime_effect_domain` (where enforcement is expected)
- `audit_steps[]` (should remain the 6-step grammar)

#### `reason-codes.v1.json` (required)

Defines stable decision explanations:
- keys are reason code identifiers (stable strings)
- each entry declares default outcome and severity

Reason codes MUST remain stable once a pack is used for qualification.

#### `evidence.required.v1.json` (required)

Defines evidence minimum schema:
- list of required fields emitted by the runtime/evidence pipeline
- used by QT harness to validate evidence completeness

#### `kpi.v1.json` (required)

Defines the KPIs required for qualification:
- which metrics must be measured
- recommended thresholds (optional)

#### `contracts/` (required)

Baselines that drive decisions:
- `baseline-allow.json` - nominal behavior
- `baseline-deny.json` - blocks forbidden effects
- `baseline-quarantine.json` - escalates to quarantine for forbidden effects (or specified patterns)

Baselines MUST be hashable and recorded in evidence as `baseline.hash`.

#### `vectors/` (required)

Safe stimuli and expected results:
- `attack_profile.safe.json` - stimulus description (must be safe/non-destructive)
- `expected_outcomes.json` - expected decision/enforcement outcomes for the stimulus

Vectors are designed to be executed repeatedly and publicly.

## Optional (recommended) files

You MAY add:
- `README.md` - human-readable explanation
- `schema/` - JSON schema definitions for pack files
- `fixtures/` - sample data for local runs
- `adapters/` - notes for domain-specific adapters (physical, biological, etc.)

## Versioning policy

- Packs are versioned at folder level: `*-v1`, `*-v2`, ...
- A new version is required if:
  - reason codes change,
  - required evidence fields change,
  - baseline semantics change in a way that impacts pass/fail,
  - forbidden effects change meaningfully.

Minor edits that do not affect qualification may stay within the same version (but should increment `revision` in `pack.meta.json` if you add it later).

## Qualification expectations (SC-102 alignment)

For SC-102 (core-only) the following invariants must hold for every pack:
- decisions are stable (`outcome + reason_code`)
- fail-closed enforcement on forbidden effects
- evidence completeness per `evidence.required.v1.json`
- repeatability (3 consecutive runs consistent under same baseline)

The pack defines semantics; the runtime must preserve the grammar.

## Security and safety

- Vectors MUST be safe and non-destructive.
- No real secrets in pack files.
- If a pack models a harmful real-world action (e.g., physical actuation), the attack profile must be a simulated adapter event, not a real actuator command to hardware.

## Next: Creating packs

To add a new pack:
1. Choose `Dk` and define the forbidden effect.
2. Define 2-3 stable reason codes.
3. Define required evidence fields.
4. Define KPIs.
5. Provide baselines (allow/deny/quarantine).
6. Add a safe vector and expected outcome.
7. Reference the pack in:
   - the relevant `SC-xxx` scenario spec, and
   - the relevant `QT-...` qualification gate.
