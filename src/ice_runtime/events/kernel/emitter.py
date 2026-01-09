"""
ICE Runtime — Event Emitter
==========================

Unico punto di emissione degli eventi ICE.

Questo modulo:
- valida
- verifica autorità
- persiste

NON esistono altri canali.
"""

from __future__ import annotations

from datetime import datetime
from typing import Set

from .event import ICEEvent, EventID
from .validator import EventValidator
from .store import EventStore


class EventEmissionError(Exception):
    """Tentativo di emissione evento illegale."""
    pass


class EventEmitter:
    """
    L'Emitter è l'UNICO gateway di scrittura eventi.

    Ogni evento:
    - passa da qui
    - oppure NON ESISTE
    """

    def __init__(self, *, store: EventStore) -> None:
        self._store = store

    # ------------------------------------------------------------------
    # API Pubblica
    # ------------------------------------------------------------------

    def emit(self, event: ICEEvent) -> ICEEvent:
        """
        Emissione ufficiale di un evento ICE.

        Ordine NON modificabile:
        1. Validazione
        2. Persistenza
        """

        try:
            known_ids: Set[EventID] = {
                e.event_id for e in self._store.by_run(event.run_id)
            }

            last_event = self._store.last()
            last_ts: datetime | None = (
                last_event.timestamp if last_event else None
            )

            EventValidator.validate(
                event,
                known_event_ids=known_ids,
                last_timestamp=last_ts,
            )

        except Exception as exc:
            raise EventEmissionError(str(exc)) from exc

        # Atto irreversibile
        self._store.append(event)
        return event

    # ------------------------------------------------------------------
    # Accesso controllato (read-only)
    # ------------------------------------------------------------------

    @property
    def store(self) -> EventStore:
        """
        Espone lo store SOLO in lettura.
        """
        return self._store


"""
Clausola Finale:

- Tutti gli eventi passano da qui
- Se non passano → NON ESISTONO
- Nessun evento "interno" è ammesso

ICE è:
→ event-only
→ auditabile
→ replayable
"""
