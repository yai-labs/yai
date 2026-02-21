# Documentation Map

This page is the canonical entry point for the documentation tree.
If you are unsure where to write or where to read first, start here.

The goal of this map is to reduce ambiguity:
- what each docs area is for,
- which artifacts are normative vs explanatory,
- which path to follow from design intent to delivery evidence.

## Documentation Areas

- Policy: `docs/_policy/`
- Architecture: `docs/architecture/`
- Design (ADRs, proposals, traceability): `docs/design/`
- Runbooks (execution): `docs/runbooks/`
- Milestone Packs (delivery closure): `docs/milestone-packs/`
- Audits and claims: `docs/audits/`
- Program delivery convergence: `docs/program-delivery/`
- Test plans (evidence expectations): `docs/test-plans/`
- Templates (single source): `docs/templates/`
- Developer guides (workflow/tooling/release): `docs/dev-guide/`
  - Dev guide index: `docs/dev-guide/README.md`
  - Agent execution guide: `docs/dev-guide/agent-playbook.md`
  - Agent normative contract: `docs/dev-guide/agent-contract.md`
  - Program PMO governance model: `docs/dev-guide/github-program-governance.md`
- Getting started (onboarding): `docs/getting-started/`
- User-guide pointers: `docs/user-guide/`

## Canonical Navigation

- Design spine: `docs/design/spine.md`
- Specs bridge: `docs/architecture/specs-bridge.md`
- Traceability map: `docs/design/traceability.md`
- Audit convergence plan: `docs/program-delivery/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- Audit convergence matrix: `docs/program-delivery/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`
- Infra Grammar claims registry: `docs/audits/claims/infra-grammar.v0.1.json`

These files define the high-level navigation model:
- spine: lifecycle and artifact ordering,
- bridge: normative boundary with `yai-specs`,
- traceability: cross-link map from decision to proof,
- convergence plan/matrix: execution order to audit-green closure.

## Recommended Reading Order

1. `docs/design/spine.md`
2. `docs/architecture/specs-bridge.md`
3. `docs/design/adr/README.md`
4. `docs/program-delivery/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
5. `docs/program-delivery/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`
6. `docs/runbooks/README.md`
7. `docs/milestone-packs/README.md`
8. `docs/test-plans/README.md`

## Writing Rule

If you are creating a new artifact, pick the template from:
- `docs/templates/README.md`

Do not create ad-hoc template copies in local subfolders.
