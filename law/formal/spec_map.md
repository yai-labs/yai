# Spec → Runtime Map (Kernel)

This map links the formal model to runtime enforcement surfaces.
It is normative for alignment and must be kept current with both the TLA+ spec and kernel code.

---

## Canonical Spec Source

- `formal/YAI_KERNEL.tla`

---

## Runtime Sources

- `../kernel/include/yai_vault.h` (canonical L0 state enum)
- `../kernel/src/core/fsm.c` (transition graph + enforcement)
- `../kernel/include/yai_kernel.h` (runtime run-state enum)

---

## State Mapping

TLA+ model states:

- HALT
- PREBOOT
- READY
- HANDOFF_COMPLETE
- RUNNING
- SUSPEND
- ERROR

Runtime L0 state enum in `yai_vault.h`:

- YAI_STATE_HALT
- YAI_STATE_PREBOOT
- YAI_STATE_READY
- YAI_STATE_HANDOFF_COMPLETE
- YAI_STATE_RUNNING
- YAI_STATE_SUSPENDED
- YAI_STATE_ERROR

### Mapping (Aligned)

| TLA State | Kernel L0 State | Notes |
| --- | --- | --- |
| HALT | YAI_STATE_HALT | Direct match |
| PREBOOT | YAI_STATE_PREBOOT | Direct match |
| READY | YAI_STATE_READY | Direct match |
| HANDOFF_COMPLETE | YAI_STATE_HANDOFF_COMPLETE | Direct match |
| RUNNING | YAI_STATE_RUNNING | Direct match |
| SUSPEND | YAI_STATE_SUSPENDED | Direct match |
| ERROR | YAI_STATE_ERROR | Direct match |

---

## Transition Mapping

TLA+ transitions:

- Strap_Preboot: HALT -> PREBOOT
- Preboot_Ready: PREBOOT -> READY
- Handoff_Complete: READY -> HANDOFF_COMPLETE
- Handoff_Run: HANDOFF_COMPLETE -> RUNNING
- Engine_Execute: READY/RUNNING -> RUNNING
- Critical_Invalidation: RUNNING -> SUSPEND
- Reconfigure: SUSPEND (cognitive_map FALSE -> TRUE)
- Suspend_Resume: SUSPEND -> RUNNING (cognitive_map TRUE)
- System_Reset: SUSPEND -> HALT
- Engine_Error: RUNNING -> ERROR
- Engine_Halt: RUNNING -> HALT
- Error_Reset: ERROR -> HALT

Kernel transition graph in `fsm.c`:

- HALT -> PREBOOT
- PREBOOT -> READY
- READY -> RUNNING | HANDOFF_COMPLETE
- HANDOFF_COMPLETE -> RUNNING
- RUNNING -> SUSPENDED | ERROR | HALT
- SUSPENDED -> RUNNING | HALT
- ERROR -> HALT

### Mapping (TLA → Kernel)

| TLA Transition | Kernel Equivalent | Notes |
| --- | --- | --- |
| Strap_Preboot | HALT -> PREBOOT | Direct match |
| Preboot_Ready | PREBOOT -> READY | Direct match |
| Handoff_Complete | READY -> HANDOFF_COMPLETE | Direct match |
| Handoff_Run | HANDOFF_COMPLETE -> RUNNING | Direct match |
| Engine_Execute | READY/RUNNING -> RUNNING | Direct match |
| Critical_Invalidation | RUNNING -> SUSPENDED | Direct match |
| Suspend_Resume | SUSPENDED -> RUNNING | Direct match (guarded by cognitive_map in spec) |
| System_Reset | SUSPENDED -> HALT | Direct match |
| Engine_Error | RUNNING -> ERROR | Direct match |
| Engine_Halt | RUNNING -> HALT | Direct match |
| Error_Reset | ERROR -> HALT | Direct match |
| Reconfigure | SUSPEND, cognitive_map FALSE -> TRUE | Implemented via `YAI_CMD_RECONFIGURE` in engine; clears authority_lock and currently resets to HALT (combined reconfigure+reset). |

---

## Authority / Guard Mapping (Spec vs Runtime)

TLA+ guards:

- AuthorityRequired: RUNNING implies authority != NONE
- CognitiveIntegrity: if cognitive_map = FALSE then state != RUNNING
- ExternalEffectGuard: external_effect implies authority != ENGINE
- EnergySafe: energy >= 0
- TraceBound: trace_id <= TraceBoundMax (bounded model checking)

Runtime enforcement (kernel):

- `fsm.c` enforces state transition legality.
- `fsm.c` enforces **energy guard** (energy_consumed <= energy_quota).
- `fsm.c` enforces **authority guard** for RUNNING using `authority_lock` (mapping: `authority_lock == true` implies no authority / cognitive invalid).
- CognitiveIntegrity is partially enforced via the same authority_lock mapping.
- ExternalEffectGuard is enforced at **L1 + L2** via command-class gating (kernel guard + engine refusal).

---

## Compliance Actions Required

- ExternalEffectGuard is now enforced in kernel; keep classification aligned with protocol commands.
- Reconfigure is now implemented via `YAI_CMD_RECONFIGURE`; keep aligned with kernel transitions.
- Add per-transition trace evidence emission fields beyond state transition (intent/authority/context).

---

## Versioning

This map must be updated whenever either:

- `formal/YAI_KERNEL.tla` changes, or
- kernel state enums / transitions change.
