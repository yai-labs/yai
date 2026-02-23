# VERSIONING

This document defines versioning policy for the `yai` runtime implementation repository.

Public release baseline: `v0.1.0` (2026-02-17).

## Scope

This policy applies to:

- runtime implementation (`boot/`, `root/`, `kernel/`, `engine/`, `runtime/`, `mind/`)
- release tooling and verification gates used to build/ship runtime artifacts
- repository governance files that affect delivery guarantees (e.g., release checks)

Normative contracts are defined in `deps/yai-specs` and are not superseded by implementation.

## Version Scheme

`yai` follows Semantic Versioning: `MAJOR.MINOR.PATCH`.

- `MAJOR`: breaking behavior or compatibility guarantees
- `MINOR`: backward-compatible features or additive behavior
- `PATCH`: backward-compatible fixes, hardening, docs, packaging, or maintenance

Pre-1.0 note: while in `0.x`, incompatible changes may still occur, but they must be explicitly declared and treated as breaking for operators/consumers.

## Contract Pinning Requirement

Every release must explicitly declare the contract baseline used during verification:

- pinned `deps/yai-specs` commit (and tag if available)

Release notes must include:

- `yai` version
- pinned `deps/yai-specs` commit/tag
- a concise compatibility impact summary
- any required operator actions (if applicable)

## What Counts as Breaking

A change is breaking if it requires consumers/operators to change behavior or tooling, including:

- protocol envelope changes (fields, semantics, role/authority interpretation)
- control-plane behavior or required commands/flags
- workspace lifecycle and isolation guarantees
- required filesystem paths, socket locations, or runtime directory conventions
- changes that invalidate existing bundles/manifests or verification expectations
- changes that alter contract-facing output formats used by tooling/CI

Breaking changes must be reflected in:

- `CHANGELOG.md` under `Unreleased` (clear, operator-facing bullet)
- release notes for the next tagged version
- supporting evidence (tests/logs) where appropriate

## Compatibility Linkage

The compatibility matrix and platform support guarantees are maintained in `COMPATIBILITY.md`.

`VERSIONING.md` defines how versions are incremented; `COMPATIBILITY.md` defines what is guaranteed at a given version.

## Legacy Tag Note

Legacy tags such as `v1.0.0-foundation` and `v1.1.0-foundation` are pre-public internal milestones.

The public SemVer line for this repository starts at `v0.1.0`.

## License

This policy is part of the Apache-2.0 licensed repository. See `LICENSE` and `NOTICE`.