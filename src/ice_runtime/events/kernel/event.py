# src/ice_runtime/events/kernel/event.py
"""
ICE Runtime — Event Kernel (Foundational Event)
================================================

Questo modulo definisce COSA È un evento ICE.

Responsabilità ESCLUSIVE:
- struttura canonica dell’evento
- invarianti strutturali
- integrità deterministica
- serializzazione stabile

Questo modulo NON:
- emette eventi
- valida policy
- conosce tassonomie
- persiste eventi
- prende decisioni

Se questo file è errato, ICE non ha realtà.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Dict, Optional, Tuple
from datetime import datetime, timezone
import hashlib
import json


# ============================================================================
# Primitive Types (esplicite, non semantiche)
# ============================================================================

EventID = str
RunID = str
EventType = str
Origin = str            # es: "runtime", "engine", "agent:<id>", "system"
Timestamp = datetime
IntegrityHash = str


# ============================================================================
# Kernel Exceptions
# ============================================================================

class EventInvariantViolation(Exception):
    """
    Violazione di un invariante strutturale dell’evento ICE.

    Questo errore indica:
    → evento malformato
    → corruzione del kernel
    → bug architetturale
    """
    pass


# ============================================================================
# ICE Event (Fondativo)
# ============================================================================

@dataclass(frozen=True, slots=True)
class ICEEvent:
    """
    Evento ICE immutabile.

    Un ICEEvent rappresenta un FATTO già accaduto,
    osservabile e registrabile sotto autorità del Runtime.

    Un evento NON è:
    - una intenzione
    - una richiesta
    - una promessa
    - una previsione
    """

    # ------------------------------------------------------------------ #
    # Identità
    # ------------------------------------------------------------------ #

    event_id: EventID
    run_id: RunID
    event_type: EventType

    # ------------------------------------------------------------------ #
    # Temporalità
    # ------------------------------------------------------------------ #

    timestamp: Timestamp

    # ------------------------------------------------------------------ #
    # Autorità di origine
    # ------------------------------------------------------------------ #

    origin: Origin

    # ------------------------------------------------------------------ #
    # Contenuto fattuale
    # ------------------------------------------------------------------ #

    payload: Dict[str, Any]

    # ------------------------------------------------------------------ #
    # Causalità esplicita
    # ------------------------------------------------------------------ #

    causality: Optional[Tuple[EventID, ...]] = None

    # ------------------------------------------------------------------ #
    # Integrità
    # ------------------------------------------------------------------ #

    integrity: IntegrityHash = field(init=False)

    # ================================================================== #
    # Post-init
    # ================================================================== #

    def __post_init__(self) -> None:
        # normalizzazione temporale (UTC obbligatorio)
        if self.timestamp.tzinfo is None:
            object.__setattr__(
                self,
                "timestamp",
                self.timestamp.replace(tzinfo=timezone.utc),
            )

        object.__setattr__(self, "integrity", self._compute_integrity())
        self._enforce_invariants()

    # ================================================================== #
    # Invarianti strutturali HARD
    # ================================================================== #

    def _enforce_invariants(self) -> None:
        if not self.event_id or not isinstance(self.event_id, str):
            raise EventInvariantViolation("event_id mancante o non valido")

        if not self.run_id or not isinstance(self.run_id, str):
            raise EventInvariantViolation("run_id mancante o non valido")

        if not self.event_type or not isinstance(self.event_type, str):
            raise EventInvariantViolation("event_type mancante o non valido")

        if not isinstance(self.timestamp, datetime):
            raise EventInvariantViolation("timestamp non valido")

        if not self.origin or not isinstance(self.origin, str):
            raise EventInvariantViolation("origin mancante o non valido")

        if not isinstance(self.payload, dict):
            raise EventInvariantViolation("payload deve essere un dict")

        if self.causality is not None:
            if not isinstance(self.causality, tuple):
                raise EventInvariantViolation("causality deve essere una tuple")

            if not self.causality:
                raise EventInvariantViolation("causality vuota non ammessa")

            for eid in self.causality:
                if not isinstance(eid, str):
                    raise EventInvariantViolation(
                        "causality deve contenere solo EventID validi"
                    )

    # ================================================================== #
    # Integrità deterministica
    # ================================================================== #

    def _compute_integrity(self) -> IntegrityHash:
        """
        Calcola un hash deterministico dell’evento.

        Regole:
        - include TUTTI i campi strutturali
        - esclude `integrity`
        - serializzazione canonica
        """
        canonical = {
            "event_id": self.event_id,
            "run_id": self.run_id,
            "event_type": self.event_type,
            "timestamp": self.timestamp.isoformat(),
            "origin": self.origin,
            "payload": self.payload,
            "causality": self.causality,
        }

        encoded = json.dumps(
            canonical,
            sort_keys=True,
            separators=(",", ":"),
            ensure_ascii=False,
            default=str,
        ).encode("utf-8")

        return hashlib.sha256(encoded).hexdigest()

    # ================================================================== #
    # Serializzazione
    # ================================================================== #

    def to_dict(self) -> Dict[str, Any]:
        """
        Rappresentazione serializzabile dell’evento.

        Usabile per:
        - logging
        - persistenza
        - trasporto
        - audit
        """
        return {
            "event_id": self.event_id,
            "run_id": self.run_id,
            "event_type": self.event_type,
            "timestamp": self.timestamp.isoformat(),
            "origin": self.origin,
            "payload": self.payload,
            "causality": self.causality,
            "integrity": self.integrity,
        }
