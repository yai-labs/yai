# Security Policy

## Scope

This policy covers the `yai` runtime repository.
Contract-level normative behavior is defined in `deps/yai-specs` and consumed by this runtime.

## Disclosure Process

Use one of the following channels:

- Open a private GitHub Security Advisory for this repository, if available.
- Otherwise open a GitHub issue with label `security` and include clear impact, reproduction, and affected paths.

Do not include secrets, tokens, credentials, or private data in reports.

## Supported Versions

- Active development line: `main`
- Release support windows and compatibility guarantees are defined in `VERSIONING.md` and `COMPATIBILITY.md`.

## Exposure Model

`yai` is local-first and not internet-exposed by default.
Primary runtime surfaces are local process boundaries and workspace-scoped Unix Domain Sockets.

## Threat Model (Summary)

Primary security boundaries:

- Root plane authority and process supervision (`root/`)
- Workspace UDS control sockets and envelope validation
- Workspace isolation across run directories and process state
- Provider gate attachment/detachment and trust transitions (`engine/`)

## Hardening Checklist

- Keep `deps/yai-specs` pinned and verified before upgrade.
- Validate request envelope/version before dispatch.
- Enforce role/arming/authority checks on privileged operations.
- Keep workspace-scoped sockets, locks, and runtime files isolated.
- Ensure logs/events are emitted for critical state transitions and denials.
- Do not commit runtime logs, secrets, or generated state.

## License

This security policy document is licensed under Apache-2.0.
