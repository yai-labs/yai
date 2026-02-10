# yai-law

YAI law is the authority repo for the YAI system. It defines the axioms,
invariants, boundaries, formal models, and protocol specs that all downstream
repos must obey.

## What Lives Here

- `axioms/` — non-negotiable truths
- `invariants/` — structural rules that must never break
- `boundaries/` — L0–L3 authority levels
- `specs/protocol/` — wire IDs and ABI specs (source of truth)
- `formal/YAI_KERNEL.tla`, `formal/YAI_KERNEL.cfg` — formal kernel model

## Canonical Boundary Levels

- L0 Vault: `boundaries/L0-vault.md`
- L1 Kernel: `boundaries/L1-kernel.md`
- L2 Engine: `boundaries/L2-engine.md`
- L3 Mind: `boundaries/L3-mind.md`
- Lx Docs: `boundaries/Lx-docs.md`

## No Tooling Here

This repo contains no build system, no site generator, and no runtime code.
It is a law and spec authority repo only.

## Quick Links

- Protocol authority: `specs/protocol/README.md`
- Kernel model: `formal/YAI_KERNEL.tla`, `formal/YAI_KERNEL.cfg`
- Boundaries index: `boundaries/README.md`
