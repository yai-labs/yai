# K-7 Kernel Policy and Grants Core

## Scope

Kernel owns only the privileged root of enforceable permissions:
- capability classes
- grant validity
- admission-time policy gating
- containment-linked permission checks
- revocation primitives
- low-level enforcement linkage

Kernel does not own high-level policy composition or governance workflows.

## Canonical capability classes

Defined in `kernel/include/yai/kernel/capabilities.h`:
- create-container
- bind-session
- enter-container
- attach-mount
- request-escape
- open-priv-shell
- spawn-daemon-binding
- consume-isolation-class
- access-ipc-class
- access-resource-class
- trigger-control-op

## Canonical grant model

Defined in `kernel/include/yai/kernel/grants.h`:
- grant id
- subject handle
- capability class
- scope handle
- validity state
- issued/expires/revoked timestamps
- flags

Grant states:
- pending
- active
- suspended
- revoked
- expired
- denied

## Kernel policy hooks

Defined in `kernel/include/yai/kernel/policy.h` and implemented in `kernel/policy/policy_hooks.c`:
- can-admit-session
- can-bind-container
- can-mount
- can-escape
- can-spawn

## Enforcement linkage

- `yai_kernel_capability_check(...)`
- `yai_kernel_grant_check(...)`

Grant lifecycle primitives:
- issue
- activate
- suspend
- revoke
- expire

## Ownership lock

Higher layers may request or propose policy/grants, but cannot bypass kernel for:
- effective grant activation
- capability enforcement on low-level operations
- canonical revocation state
