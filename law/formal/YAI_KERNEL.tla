----------------------- MODULE YAI_KERNEL -----------------------
EXTENDS Naturals

CONSTANTS 
    MaxEnergy,
    ActionCost,
    TraceBoundMax

States == {"HALT", "PREBOOT", "READY", "HANDOFF_COMPLETE", "RUNNING", "SUSPEND", "ERROR"}
Authority == {"NONE", "STRAP", "ENGINE"}

VARIABLES 
    state,
    authority,
    cognitive_map,
    energy,
    trace_id,
    external_effect

TypeInvariant ==
    /\ state \in States
    /\ authority \in Authority
    /\ cognitive_map \in BOOLEAN
    /\ energy \in Nat
    /\ trace_id \in Nat
    /\ external_effect \in BOOLEAN

EnergySafe ==
    energy >= 0

CognitiveIntegrity ==
    (cognitive_map = FALSE) => (state /= "RUNNING")

AuthorityRequired ==
    (state = "RUNNING") => (authority # "NONE")

ExternalEffectGuard ==
    external_effect => authority # "ENGINE"

TraceBound ==
    trace_id <= TraceBoundMax

Init ==
    /\ state = "HALT"
    /\ authority = "NONE"
    /\ cognitive_map = TRUE
    /\ energy = MaxEnergy
    /\ trace_id = 0
    /\ external_effect = FALSE

Strap_Preboot ==
    /\ state = "HALT"
    /\ state' = "PREBOOT"
    /\ authority' = "STRAP"
    /\ energy' = MaxEnergy
    /\ UNCHANGED <<cognitive_map, trace_id, external_effect>>

Preboot_Ready ==
    /\ state = "PREBOOT"
    /\ state' = "READY"
    /\ UNCHANGED <<authority, cognitive_map, energy, trace_id, external_effect>>

Handoff_Complete ==
    /\ state = "READY"
    /\ state' = "HANDOFF_COMPLETE"
    /\ UNCHANGED <<authority, cognitive_map, energy, trace_id, external_effect>>

Handoff_Run ==
    /\ state = "HANDOFF_COMPLETE"
    /\ state' = "RUNNING"
    /\ UNCHANGED <<authority, cognitive_map, energy, trace_id, external_effect>>

Engine_Execute ==
    /\ state \in {"READY", "RUNNING"}
    /\ authority # "NONE"
    /\ cognitive_map = TRUE
    /\ energy >= ActionCost
    /\ state' = "RUNNING"
    /\ energy' = energy - ActionCost
    /\ trace_id' = trace_id + 1
    /\ external_effect' = FALSE
    /\ UNCHANGED <<authority, cognitive_map>>

Critical_Invalidation ==
    /\ state = "RUNNING"
    /\ cognitive_map' = FALSE
    /\ state' = "SUSPEND"
    /\ UNCHANGED <<authority, energy, trace_id, external_effect>>

Suspend_Resume ==
    /\ state = "SUSPEND"
    /\ cognitive_map = TRUE
    /\ state' = "RUNNING"
    /\ UNCHANGED <<authority, cognitive_map, energy, trace_id, external_effect>>

System_Reset ==
    /\ state = "SUSPEND"
    /\ state' = "HALT"
    /\ authority' = "NONE"
    /\ cognitive_map' = TRUE
    /\ UNCHANGED <<energy, trace_id, external_effect>>

Engine_Error ==
    /\ state = "RUNNING"
    /\ state' = "ERROR"
    /\ UNCHANGED <<authority, cognitive_map, energy, trace_id, external_effect>>

Engine_Halt ==
    /\ state = "RUNNING"
    /\ state' = "HALT"
    /\ UNCHANGED <<authority, cognitive_map, energy, trace_id, external_effect>>

Error_Reset ==
    /\ state = "ERROR"
    /\ state' = "HALT"
    /\ UNCHANGED <<authority, cognitive_map, energy, trace_id, external_effect>>

Reconfigure ==
    /\ state = "SUSPEND"
    /\ cognitive_map = FALSE
    /\ cognitive_map' = TRUE
    /\ UNCHANGED <<authority, state, energy, trace_id, external_effect>>

Next ==
    Strap_Preboot
    \/ Preboot_Ready
    \/ Handoff_Complete
    \/ Handoff_Run
    \/ Engine_Execute
    \/ Critical_Invalidation
    \/ Reconfigure
    \/ Suspend_Resume
    \/ System_Reset
    \/ Engine_Error
    \/ Engine_Halt
    \/ Error_Reset

Spec ==
    Init /\ [][Next]_<<state, authority, cognitive_map, energy, trace_id, external_effect>>

THEOREM Spec =>
    []TypeInvariant
    /\ []EnergySafe
    /\ []CognitiveIntegrity
    /\ []AuthorityRequired
    /\ []ExternalEffectGuard
=============================================================================
