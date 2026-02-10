# Kernel ↔ Law Binding (Normative)

This document defines the **binding contract** between YAI Law and the kernel runtime.
It is normative: kernel behavior MUST conform to these mappings.

---

## Canonical Law Sources

- `axioms/*`
- `invariants/*`
- `boundaries/L0-vault.md`
- `boundaries/L1-kernel.md`
- `formal/YAI_KERNEL.tla`

---

## Variable Binding Map

| Law Concept | TLA Variable | Kernel Runtime Symbol | Notes |
| --- | --- | --- | --- |
| Execution State (A-001) | `state` | `yai_vault_t.status` | Canonical kernel state enum in `yai_vault.h`. |
| Authority (A-002) | `authority` | `yai_vault_t.authority_lock` | **Partial** mapping: `authority_lock == true` implies **no authority**. |
| Cognitive Validity (A-004) | `cognitive_map` | `!yai_vault_t.authority_lock` | Cognitive validity is represented implicitly. |
| Cost Accountability (I-005) | `energy` | `energy_quota`, `energy_consumed` | Guarded in `yai_kernel_transition`. |
| Traceability (I-001) | `trace_id` | `logical_clock` + `yai_trace_transition()` | Kernel emits transition evidence. |
| External Effect Boundary (I-006) | `external_effect` | Kernel + engine command-class gate | Enforced via `yai_command_class_for` in kernel guard and engine refusal. |
| Command Intent | N/A (transition guards) | `last_command_id`, `command_seq` | Command is evaluated by engine; kernel gates state. |

---

## Transition Binding Map

| TLA Transition | Runtime Surface | Notes |
| --- | --- | --- |
| `Strap_Preboot` | `yai_kernel_transition(HALT -> PREBOOT)` | Kernel transition graph. |
| `Preboot_Ready` | `yai_kernel_transition(PREBOOT -> READY)` | Kernel transition graph. |
| `Handoff_Complete` | `yai_kernel_transition(READY -> HANDOFF_COMPLETE)` | Kernel transition graph. |
| `Handoff_Run` | `yai_kernel_transition(HANDOFF_COMPLETE -> RUNNING)` | Kernel transition graph. |
| `Engine_Execute` | Engine command processing | Engine sets RUNNING during execution; must be state-gated. |
| `Critical_Invalidation` | `yai_kernel_transition(RUNNING -> SUSPENDED)` | Kernel transition graph. |
| `Reconfigure` | `YAI_CMD_RECONFIGURE` (engine) | Clears `authority_lock`; runtime currently also sets `HALT` (combined reconfigure+reset). |
| `Suspend_Resume` | `yai_kernel_transition(SUSPENDED -> RUNNING)` | Guarded by `authority_lock == false`. |
| `System_Reset` | `yai_kernel_transition(SUSPENDED -> HALT)` | Kernel transition graph. |
| `Engine_Error` | `yai_kernel_transition(RUNNING -> ERROR)` | Kernel transition graph. |
| `Engine_Halt` | `yai_kernel_transition(RUNNING -> HALT)` | Kernel transition graph. |
| `Error_Reset` | `yai_kernel_transition(ERROR -> HALT)` | Kernel transition graph. |

---

## Guard / Enforcement Binding

| Law Constraint | TLA Predicate | Runtime Enforcement |
| --- | --- | --- |
| Authority Required | `AuthorityRequired` | `yai_guard_authority` in `fsm.c` (`authority_lock` gate). |
| Cognitive Integrity | `CognitiveIntegrity` | Same gate as above (implicit mapping). |
| Energy Safe | `EnergySafe` | `yai_guard_energy` in `fsm.c`. |
| External Effect Guard | `ExternalEffectGuard` | Enforced at L1 + L2 via command-class gating. |
| Trace Bound | `TraceBound` | Formal-only (TLC model bound). |

---

## Alignment Notes

- Runtime conflates **authority** and **cognitive validity** via `authority_lock`.
  If a separate cognitive validity field is introduced, this map MUST be updated.
- `Reconfigure` is implemented as a kernel-safe reset (SUSPEND + clear lock + HALT).
  This is equivalent to `Reconfigure` followed immediately by `System_Reset`.
- External effect classification remains a required open item for I-006 compliance.

---

## Change Control

Any change to the kernel state machine, guards, or vault layout MUST update:

- `formal/YAI_KERNEL.tla`
- this binding document
- the relevant runtime implementation (`yai-kernel`)

Silent drift is non-compliant by definition.
