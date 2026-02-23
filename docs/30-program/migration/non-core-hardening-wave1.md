# Non-Core Hardening Wave 1 (Historical Note)

## Goal
Reduce non-core surface in `yai` while keeping CI/runtime behavior stable.

## Wave 1 Delivered
- Removed local Python test package under `tools/python/yai_tools/tests`.
- Removed stale Python cache artifacts under `tools/python/yai_tools/__pycache__`.

## Safety Rationale
- These paths are not runtime build artifacts.
- CI workflows do not depend on removed local test/cache paths.
- Runtime domains (`boot/`, `root/`, `kernel/`, `engine/`, `mind/`, `runtime/`, `law/`) were not changed by this wave.

## Current Alignment Status
- Governance docs are now maintained as canonical local content in `docs/`.
- No compatibility stubs are required for templates/policies/dev-guide in this repository.
- Program migration notes are retained only as historical traceability records.
