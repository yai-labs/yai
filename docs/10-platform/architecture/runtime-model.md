---
id: ARCH-RUNTIME-MODEL
status: active
effective_date: 2026-02-19
revision: 1
owner: architecture
law_refs:
  - deps/yai-specs/contracts/boundaries/L1-kernel.md
  - deps/yai-specs/contracts/boundaries/L2-engine.md
  - deps/yai-specs/contracts/invariants/I-001-traceability.md
---

# Runtime Model

## Role

Specify runtime planes, interfaces, and boundary rules using code-backed statements.

## Canonical Topology

Canonical Topology: Root -> Kernel -> Engine with Mind as planned/external L3 plane.

## Plane map (current)

- Root ingress: `~/.yai/run/root/root.sock` in current implementation (`root/src/yai_root_server.c`).
- Kernel control: `~/.yai/run/kernel/control.sock` (`kernel/src/bin/workspace_kernel_main.c`).
- Engine internal path references include `~/.yai/run/root/control.sock` for client connection (`engine/src/bridge/transport_client.c`).
- Boot creates runtime dirs and known socket paths (`boot/src/preboot.c`).

## Authority flow

- Root performs envelope/path guardrails and handshake gating.
- Kernel validates command semantics and handshake transitions.
- Engine executes downstream workloads and gate dispatch.
- Mind is not an authority surface.

## Current drift notes

- Root socket naming differs between files (`root.sock` vs `control.sock` path assumptions).
- Runtime documentation must treat current socket values as implemented reality and mark harmonization as pending.

## Traceability

- ADRs: `docs/20-governance/design/adr/ADR-002-root-entrypoint.md`, `docs/20-governance/design/adr/ADR-003-kernel-authority.md`, `docs/20-governance/design/adr/ADR-004-engine-execution.md`, `docs/20-governance/design/adr/ADR-008-connection-lifecycle.md`, `docs/20-governance/design/adr/ADR-010-boot-entrypoint.md`
- Runbooks: `docs/20-governance/runbooks/root-hardening.md`, `docs/20-governance/runbooks/workspaces-lifecycle.md`
- MPs: `docs/20-governance/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`, `docs/20-governance/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.1.md`
- L0 anchors: `deps/yai-specs/specs/protocol/include/transport.h`, `deps/yai-specs/specs/protocol/include/session.h`, `deps/yai-specs/specs/protocol/runtime/include/rpc_runtime.h`
