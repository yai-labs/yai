# Protocol Spec (Source of Truth)

This directory is the authoritative definition of YAI wire formats and IDs.

Rules:
- **No runtime code** lives here.
- Implementations live in **Kernel/Engine (C)** and **Mind/API (Rust)**.
- **Backward compatible changes** increment minor rules (keep `YAI_PROTOCOL_VERSION` unless breaking).
- **Breaking ABI** increments `YAI_PROTOCOL_VERSION` and requires migration.

Additions:
1. Update the relevant header in this folder.
2. Add vectors in `specs/vectors/` (if applicable).
3. Mirror in Rust with `#[repr(C)]` or bindgen where needed.
