# YAI - Governed Intelligence Runtime

**Make AI accountable: authority-first execution, audit trails, deterministic gates, reproducible evidence.**

YAI is the runtime + program hub for a governed execution stack: deterministic planes, explicit authority, auditable state, and reproducible verification.

## Repository Role

This repo is the product implementation and the program center:
- Runtime planes: Boot / Root / Kernel / Engine / (Mind)
- Program docs: RFCs, ADRs, Runbooks, Milestone Packs, Qualification/Validation, Evidence

Not in this repo:
- Cross-repo governance/tooling standards (that lives in `yai-infra`)
- Canonical contracts/law artifacts (that lives in `yai-law`)

## Repo Map (What To Read Next)

- `yai-law` - canonical contracts and constraints (normative source of truth)
- `yai-cli` - operator interface (client control plane)
- `yai-infra` - open factory window (standards, automation, reusable governance suite)

## Quick Start

Build:

```bash
make build
make dist
```

Verify:

```bash
make verify
```

Docs entrypoint:
- `docs/00-dashboard.md`
- `docs/README.md`

## Law and Contract Pinning

This repo consumes canonical law as a pinned dependency:

- `deps/yai-law/` - pinned normative contracts (do not edit to fix drift)

If behavior drifts from law, the implementation is wrong.

## License

Apache-2.0. See `LICENSE`, `NOTICE`, and `THIRD_PARTY_NOTICES.md`.
