"""
ICE Runtime — Event Taxonomy
===========================

RFC-ICE-007 (v1)

Questo modulo definisce la TASSONOMIA CHIUSA degli eventi ICE.

Regola fondamentale:
Se un event_type NON è definito qui:
- NON ESISTE
- NON è valido
- NON può essere emesso
- NON può essere validato
- NON può essere persistito

Questo file è:
→ normativa
→ fondativa
→ versione-bloccata

NON è configurazione.
"""

from __future__ import annotations

from enum import Enum
from typing import FrozenSet, Dict


# ============================================================================
# Categorie Canoniche (CHIUSE)
# ============================================================================

class EventCategory(str, Enum):
    """
    Categoria canonica di un evento ICE.

    La categoria:
    - NON implica semantica
    - NON guida il comportamento
    - serve per:
        * validazione
        * policy
        * introspezione
        * routing
    """
    RUNTIME = "runtime"
    COGNITIVE = "cognitive"
    DOMAIN = "domain"
    MEMORY = "memory"
    CAPABILITY = "capability"


# ============================================================================
# Runtime Events (Lifecycle & Authority)
# ============================================================================

RUNTIME_EVENTS: FrozenSet[str] = frozenset({
    "RunProvisioned",
    "ResourcesAllocated",
    "ContextResolved",
    "MemoryMounted",
    "CapabilitiesBound",
    "ValidationStarted",
    "ValidationPassed",
    "ValidationFailed",
    "RunCommitted",
    "RunAborted",
    "AbortReasonDeclared",
    "ResourcesReleased",
    "RunTerminated",
})


# ============================================================================
# Cognitive Events (AI / Reasoning Surface)
# ============================================================================

COGNITIVE_EVENTS: FrozenSet[str] = frozenset({
    "InferenceStep",
    "DecisionProposed",
    "HypothesisGenerated",
    "UncertaintyDeclared",
    "PlanStepProposed",
})


# ============================================================================
# Domain Events (External / IO / Business)
# ============================================================================

DOMAIN_EVENTS: FrozenSet[str] = frozenset({
    "FileRead",
    "FileWritten",
    "TaskStarted",
    "TaskCompleted",
    "APIRequestExecuted",
    "UserNotificationSent",
    "WorkflowAdvanced",
})


# ============================================================================
# Memory Events (Lifecycle & Evolution)
# ============================================================================

MEMORY_EVENTS: FrozenSet[str] = frozenset({
    "MemoryPromoted",
    "MemoryExpired",
    "MemoryDeprecated",
    "MemorySuperseded",
    "MemoryInvalidated",
})


# ============================================================================
# Capability Events (Authority & Permission)
# ============================================================================

CAPABILITY_EVENTS: FrozenSet[str] = frozenset({
    "CapabilityRequested",
    "CapabilityGranted",
    "CapabilityUsed",
    "CapabilityRevoked",
    "CapabilityExpired",
})


# ============================================================================
# Indici Derivati (AUTORITATIVI)
# ============================================================================

ALL_EVENTS: FrozenSet[str] = frozenset().union(
    RUNTIME_EVENTS,
    COGNITIVE_EVENTS,
    DOMAIN_EVENTS,
    MEMORY_EVENTS,
    CAPABILITY_EVENTS,
)

EVENT_CATEGORY_MAP: Dict[str, EventCategory] = {
    **{e: EventCategory.RUNTIME for e in RUNTIME_EVENTS},
    **{e: EventCategory.COGNITIVE for e in COGNITIVE_EVENTS},
    **{e: EventCategory.DOMAIN for e in DOMAIN_EVENTS},
    **{e: EventCategory.MEMORY for e in MEMORY_EVENTS},
    **{e: EventCategory.CAPABILITY for e in CAPABILITY_EVENTS},
}


# ============================================================================
# Kernel Validation API
# ============================================================================

def is_valid_event_type(event_type: str) -> bool:
    """
    Verifica se un event_type è ammesso dalla tassonomia ICE.

    Questa funzione:
    - NON normalizza
    - NON corregge
    - NON fa fallback

    True = evento esistente
    False = evento illegale
    """
    return event_type in ALL_EVENTS


def category_of(event_type: str) -> EventCategory:
    """
    Ritorna la categoria canonica di un event_type.

    Solleva KeyError se l'evento NON è ammesso.
    Questo è INTENZIONALE.
    """
    return EVENT_CATEGORY_MAP[event_type]


# ============================================================================
# Clausola di Chiusura (NORMATIVA)
# ============================================================================

"""
Questo modulo NON deve:
- essere esteso dinamicamente
- leggere configurazioni
- accettare override
- essere monkey-patchato

La tassonomia è CHIUSA per definizione.

Aggiungere un nuovo evento richiede:
- nuova RFC
- bump di versione
- modifica esplicita di questo file
"""
