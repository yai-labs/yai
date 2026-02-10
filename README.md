# YAI Core

`yai-core` is the canonical repository that binds **YAI Law (L0/Lx)** to the **Kernel (L1)** and the **Engine (L2)**.
It is a single repo with a single build + verification pipeline and a single binary distribution, while keeping L1/L2 boundaries intact.

Law is authoritative and lives under `law/`.
Kernel is subordinate and lives under `kernel/`.
Engine is subordinate and lives under `engine/`.

---

## Repository Layout

yai-core/
├── law/        # Axioms, invariants, boundaries, specs, formal models (authority)
├── kernel/     # C runtime implementation (L1 enforcement)
├── engine/     # C runtime execution (L2)
└── scripts/    # Verification scripts (TLC + builds)

---

## Canonical Authority (Law)

The following are normative and authoritative:

- `law/axioms/`
- `law/invariants/`
- `law/boundaries/`
- `law/specs/`
- `law/formal/`

If runtime behavior conflicts with these, the runtime is wrong.

---

## Kernel Runtime (L1)

The kernel enforces:

- the canonical state machine (L1)
- authority gating
- transition trace evidence
- vault layout constraints (L0)

Kernel code lives in `kernel/` and is bound to Law via:

- `law/formal/KERNEL_LAW_BINDING.md`
- `law/formal/spec_map.md`

---

## Engine Runtime (L2)

The engine executes external effects and I/O under kernel authority.
It consumes Law-generated contracts and headers via:

- `law/specs/vault/yai_vault_abi.h`
- `law/specs/protocol/yai_protocol_ids.h`

---

## Build & Verify (Single Gate)

Single command to validate Law ↔ Kernel ↔ Engine coherence:

```bash
cd yai-core
./scripts/verify-core.sh
```

This runs:

- generated file checks (Law)
- TLC model checking (quick + deep)
- kernel + engine builds

---

## Build Targets

```bash
cd yai-core
make all
make clean
make package
```

`make package` produces:

- `../.yai/artifacts/yai-core/bin/yai-boot`
- `../.yai/artifacts/yai-core/bin/yai-kernel`
- `../.yai/artifacts/yai-core/bin/yai-engine`
- `../.yai/artifacts/yai-core/dist/yai-core-<timestamp>.tar.gz`

---

## Governance

This repository is governed by YAI Law under `law/`.
Any change to runtime behavior that affects axioms, invariants, or boundaries must update Law first.

---

## Non-Goals

yai-core is not:

- a product repo
- an application runtime
- a UI or studio repo

It is the authority + enforcement core only.
