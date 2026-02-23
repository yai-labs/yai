# YAI — Sovereign Intelligence Runtime
_Acronym: “YAI Ain’t Intelligence”_

YAI is a **sovereign runtime for intelligence**: a governed execution stack where AI systems operate with **deterministic boundaries, auditable state, and explicit authority**—not opaque side-effects.

Releases · Documentation · Specs & Contracts · Security · License

---

## Why YAI

Modern AI systems deliver capability, but routinely fail enterprise requirements: traceability, reproducibility, policy enforcement, and bounded effects. When you cannot prove **what happened**, **why**, and **under which authority**, you do not have infrastructure—you have risk.

YAI is built to make **intelligence operational**: measurable, accountable, and repeatable, so AI can be deployed with the same rigor as any other critical system.

---

## What YAI Is

YAI brings up a vertical runtime stack designed for **controlled intelligence execution**:

- **Boot** — machine bring-up and environment verification  
- **Root Plane** — global authority surface, coordination, control endpoints  
- **Kernel (L1)** — workspace isolation, session authority, policy enforcement  
- **Engine (L2)** — deterministic execution, gated external effects  
- **Mind (L3)** — orchestration and cognition (integrated module; evolving)

This repository contains the **core runtime** (Boot/Root/Kernel/Engine), the **Mind module**, and the tooling required to **verify**, **gate**, and **package** releases.

---

## Core Guarantees

YAI treats governance as a first-class system property:

- **Determinism over improvisation** — execution paths are bounded and repeatable  
- **Auditability over opacity** — state transitions are observable and attributable  
- **Explicit authority over implicit power** — permissions are declared, verified, enforced  
- **Contracts over drift** — behavior is defined by normative specs, not emergent behavior

---

## Repository Layout

- `boot/` — machine bring-up and environment verification  
- `root/` — root control plane and authority coordination  
- `kernel/` — workspace isolation + policy enforcement (L1)  
- `engine/` — deterministic execution and gated effects (L2)  
- `mind/` — cognition/orchestration module (L3)  
- `docs/` — architecture, runbooks, operations, developer guide  
- `contract/` — contract surface notes (non-normative pointers)  
- `tools/` — CI/release gates, verification, packaging tooling  
- `data/` — datasets for local tests/ops (see `DATA_POLICY.md`)  
- `deps/yai-specs/` — pinned normative specs (source of truth)

---

## Build & Verify

Build core planes:

- `make build` — build Boot/Root/Kernel/Engine  
- `make dist` — stage canonical binaries into `dist/bin`  
- `make bundle` — produce release bundle artifacts

Build Mind (optional plane):

- `make mind` — build Mind and stage `build/bin/yai-mind`  
- `make mind-check` — fmt/clippy/test for Mind

Verification:

- `make verify` — run verification suite (when present)  
- `make release-guards` — enforce pin + proof gates for release readiness

---

## Specs & Contracts

The **source of truth** for runtime behavior lives in `deps/yai-specs`.

Specs are **normative contracts**, not explanatory documentation.  
The runtime implements these contracts and must remain aligned. Any drift is treated as a defect and should block release/merge when required gates are enabled.

---

## Documentation

Start with `docs/`:

- architecture overview and component boundaries  
- operational runbooks and release procedures  
- governance and traceability conventions

---

## Security

See `SECURITY.md` for reporting and disclosure process, and `DATA_POLICY.md` for data handling expectations.

---

## License

Apache-2.0. See `LICENSE`, `NOTICE`, and `THIRD_PARTY_NOTICES.md`.