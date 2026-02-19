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
- Test plans (evidence expectations): `docs/test-plans/`
- Templates (single source): `docs/templates/`
- Developer guides (workflow/tooling/release): `docs/dev-guide/`
  - Agent execution guide: `docs/dev-guide/agent-playbook.md`
- Getting started (onboarding): `docs/getting-started/`
- User-guide pointers: `docs/user-guide/`

## Canonical Navigation

- Design spine: `docs/design/spine.md`
- Specs bridge: `docs/architecture/specs-bridge.md`
- Traceability map: `docs/design/traceability.md`

These three files define the high-level navigation model:
- spine: lifecycle and artifact ordering,
- bridge: normative boundary with `yai-specs`,
- traceability: cross-link map from decision to proof.

## Recommended Reading Order

1. `docs/design/spine.md`
2. `docs/architecture/specs-bridge.md`
3. `docs/design/adr/README.md`
4. `docs/runbooks/README.md`
5. `docs/milestone-packs/README.md`
6. `docs/test-plans/README.md`

## Writing Rule

If you are creating a new artifact, pick the template from:
- `docs/templates/README.md`

Do not create ad-hoc template copies in local subfolders.
