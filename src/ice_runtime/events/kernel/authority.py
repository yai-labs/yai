"""
ICE Runtime — Event Authority
=============================

RFC-ICE-007 · Event Taxonomy

Questo modulo definisce CHI può emettere COSA.

Non è policy.
È legge.
"""

from __future__ import annotations

from typing import Set

from .taxonomy import (
    EventCategory,
    category_of,
)

# ------------------------------------------------------------------
# Origin canonici
# ------------------------------------------------------------------

RUNTIME_ORIGIN = "runtime"
SYSTEM_ORIGIN = "system"
AGENT_PREFIX = "agent:"


# ------------------------------------------------------------------
# API Sovrana
# ------------------------------------------------------------------

def is_origin_authorized(*, origin: str, event_type: str) -> bool:
    """
    Verifica se un origin è autorizzato a emettere un event_type.
    """
    category = category_of(event_type)

    if category == EventCategory.RUNTIME:
        return origin == RUNTIME_ORIGIN

    if category == EventCategory.COGNITIVE:
        return origin.startswith(AGENT_PREFIX)

    if category == EventCategory.DOMAIN:
        return origin == RUNTIME_ORIGIN

    if category == EventCategory.MEMORY:
        return origin == RUNTIME_ORIGIN

    if category == EventCategory.CAPABILITY:
        # unica eccezione controllata
        if event_type == "CapabilityRequested":
            return origin.startswith(AGENT_PREFIX)
        return origin == RUNTIME_ORIGIN

    return False


def allowed_origins_for(event_type: str) -> Set[str]:
    """
    Restituisce gli origin ammessi per un event_type.
    """
    category = category_of(event_type)

    if category == EventCategory.COGNITIVE:
        return {AGENT_PREFIX}

    if category == EventCategory.CAPABILITY and event_type == "CapabilityRequested":
        return {AGENT_PREFIX}

    return {RUNTIME_ORIGIN}


"""
Clausola Fondativa:

- Gli agenti NON possono emettere eventi di Runtime
- Il Runtime è l'unica autorità causale
- Nessuna escalation implicita è ammessa

Se un evento passa questo controllo:
→ l'origine è legittima
"""
