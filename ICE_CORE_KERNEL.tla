----------------------- MODULE ICE_CORE_KERNEL -----------------------
EXTENDS Naturals, Reals

VARIABLES 
    state,              \* A-001: Execution State (RUN, HALT, SUSPEND)
    authority_bit,      \* A-002: Authority Token (0 or 1)
    cognitive_map,      \* A-004: Validity of cognitive configuration
    energy,             \* I-005: Abstract Cost (Resource Budget)
    trace_id            \* I-001: Structural Traceability index

CONSTANTS 
    MaxEnergy,          \* Budget totale inizializzato dallo Strap
    ActionCost          \* Costo di ogni singola transizione dell'Engine

(* -- Invarianti (Le Leggi Immutabili) -- *)

\* I-005: L'energia non può mai essere negativa (Cost Accountability)
EnergySafe == energy >= 0

\* A-004: Se la mappa cognitiva è invalida, il sistema NON può essere in RUN
CognitiveIntegrity == (cognitive_map = FALSE) => (state /= "RUN")

\* A-002: Per essere in RUN serve il bit di autorità esplicito
AuthorityRequired == (state = "RUN") => (authority_bit = 1)

(* -- Transizioni di Stato -- *)

Init == 
    /\ state = "HALT"
    /\ authority_bit = 0
    /\ cognitive_map = TRUE
    /\ energy = MaxEnergy
    /\ trace_id = 0

\* ICE-STRAP: Il Bootstrap porta il sistema in READY
Strap_Boot ==
    /\ state = "HALT"
    /\ state' = "READY"
    /\ authority_bit' = 1
    /\ energy' = MaxEnergy
    /\ UNCHANGED <<cognitive_map, trace_id>>

\* ICE-ENGINE: Esegue un'azione consumando energia
Engine_Execute ==
    /\ state \in {"READY", "RUN"}
    /\ authority_bit = 1
    /\ cognitive_map = TRUE
    /\ energy >= ActionCost
    /\ state' = "RUN"
    /\ energy' = energy - ActionCost
    /\ trace_id' = (trace_id + 1) % 100
    /\ UNCHANGED <<authority_bit, cognitive_map>>

\* Invalidation (A-004): Rilevamento discrepanza realtà/modello
Critical_Invalidation ==
    /\ state = "RUN"
    /\ cognitive_map' = FALSE
    /\ state' = "SUSPEND"
    /\ UNCHANGED <<authority_bit, energy, trace_id>>

\* RECOVERY: Il Bootstrap o l'Engine resettano la mappa e tornano in HALT
System_Reset ==
    /\ state = "SUSPEND"
    /\ state' = "HALT"
    /\ cognitive_map' = TRUE
    /\ authority_bit' = 0
    /\ UNCHANGED <<energy, trace_id>>

Next == Strap_Boot \/ Engine_Execute \/ Critical_Invalidation \/ System_Reset

Spec == Init /\ [][Next]_<<state, authority_bit, cognitive_map, energy, trace_id>>

THEOREM Spec => []EnergySafe /\ []CognitiveIntegrity /\ []AuthorityRequired
=============================================================================
