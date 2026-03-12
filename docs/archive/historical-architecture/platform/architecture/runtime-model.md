---
id: ARCH-RUNTIME-MODEL
status: historical
effective_date: 2026-02-19
revision: 1
owner: architecture
law_refs:
  - ../governance/foundation/boundaries/L1-kernel.md
  - ../governance/foundation/boundaries/L2-engine.md
  - ../governance/foundation/invariants/I-001-traceability.md
---

# Runtime Model

> Historical archive: this document reflects pre-cutover multi-plane topology.
> Authoritative runtime truth is single-binary ingress in `cmd/yai/main.c` (`yai up` + `~/.yai/run/control.sock`).

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

- ADRs: `docs/program/adr/adr-runtime-002-root-entrypoint.md`, `docs/program/adr/adr-runtime-003-kernel-authority.md`, `docs/program/adr/adr-orchestration-004-engine-execution.md`, `docs/program/adr/adr-workspace-008-connection-lifecycle.md`, `docs/program/adr/adr-runtime-010-boot-entrypoint.md`
- Runbooks: `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md`, `docs/archive/legacy/program/milestone-packs/runtime-baselines/workspace/mp-runtime-000-workspaces-lifecycle.md`
- MPs: `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md`, `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md`
- L0 anchors: `../governance/contracts/protocol/include/transport.h`, `../governance/contracts/protocol/include/session.h`, `../governance/contracts/protocol/runtime/include/rpc_runtime.h`
