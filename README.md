# ICE-Kernel (Layer 1)

> **The Hardened C11 Execution Core of the ICE Ecosystem.**

ICE-Kernel is the deterministic heart of the ICE environment. It replaces the legacy Python runtime with a high-performance, memory-safe C11 implementation.

## Core Architecture
- **Layer 0 (Bootstrap)**: Authority handoff and Vault allocation.
- **Layer 1 (Kernel)**: State machine enforcement, UDS Transport, and hardware-aligned execution.
- **Layer 2 (Engine)**: High-level orchestration (Connected via Unix Domain Sockets).

## System Status
- **FSM**: Operational (Deterministic State Machine)
- **Vault**: Integrated (C11 Shared Headers)
- **Transport**: UDS Socket (/tmp/ice_runtime.sock)
- **Language**: C11 (GCC/Clang)

## Getting Started
```bash
make clean && make
./bin/ice-kernel
```

---
*Intentional. Inspectable. Deterministic.*

