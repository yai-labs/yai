# YAI — Sovereign Intelligence Runtime
_Acronym: “YAI Ain’t Intelligence”_


YAI is a **sovereign runtime for intelligence**: a governed execution stack where AI systems run with **deterministic boundaries, auditable state, and explicit authority**—not opaque side-effects.

[ Releases ](#releases) · [ Documentation ](#documentation) · [ Specs ](#specs--contracts) · [ Security ](#security) · [ License ](#license)

---

## Why YAI

Modern AI delivers capability, but too often without governance: decisions that cannot be traced, state that cannot be proven, and effects that cannot be constrained.

YAI is built to make **intelligence operational**—measurable, accountable, and reproducible—so you can deploy AI as infrastructure, not as a leap of faith.

---

## What YAI Is

YAI brings up a vertical runtime stack designed for **controlled intelligence execution**:

- **Boot** — machine bring-up and environment verification  
- **Root Plane** — global authority surface and control coordination  
- **Kernel (L1)** — workspace isolation, policy enforcement, session authority  
- **Engine (L2)** — deterministic execution and gated external effects  
- **Mind (L3)** — higher orchestration (emerging line)

This repository contains the **foundation runtime** (Boot/Root/Kernel/Engine) and the operational tooling to verify and package it.

---

## Principles

YAI treats governance as a first-class system property:

- **Determinism over improvisation** — execution paths are bounded and repeatable  
- **Auditability over opacity** — state transitions are observable and attributable  
- **Explicit authority over implicit power** — permissions are declared, verified, and enforced  
- **Contracts over drift** — behavior is defined by specs, not by “whatever happens”

---

## Get Started

**Option A — Bundle (recommended):** download a prebuilt bundle from GitHub Releases.  
**Option B — Build from source:** clone the repo and compile the runtime.

For installation, verification, and operational runbooks, start here: `docs/`.

---

## Specs & Contracts

The source of truth for behavior lives in **`deps/yai-specs`**.

Specs are not “documentation”; they are **normative contracts**.  
The runtime implements these contracts—**and must remain aligned**. When alignment breaks, it is treated as a defect.

---

## Documentation

- `docs/` — architecture notes, runbooks, guides, and operational procedures  
- `data/` — datasets used for local tests/ops (see `DATA_POLICY.md`)  
- `scripts/` — verification and gate scripts used in CI and release pipelines

---

## Releases

GitHub Releases are the distribution channel for official bundles and metadata.  
Each bundle ships with a manifest and checksums to support reproducible deployment.

---

## Security

See `SECURITY.md` for disclosure and handling policy.

---

## License

Apache-2.0. See `LICENSE`, `NOTICE`, and `THIRD_PARTY_NOTICES.md`.
