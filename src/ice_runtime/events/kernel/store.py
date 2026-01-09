"""
ICE Runtime — Event Store
========================

Append-only Event Log sovrano.

Questo modulo è l'UNICA sorgente di persistenza degli eventi ICE.

NON:
- interpreta
- valida
- filtra
- ordina semanticamente

Fa una sola cosa:
→ registra eventi validi, in ordine temporale
"""

from __future__ import annotations

from typing import Iterable, List
from threading import Lock

from .event import ICEEvent, RunID


class EventStore:
    """
    Event Store append-only, in-memory.

    Invarianti:
    - nessuna mutazione
    - nessuna cancellazione
    - nessun update
    """

    def __init__(self) -> None:
        self._events: List[ICEEvent] = []
        self._lock = Lock()

    # ------------------------------------------------------------------
    # Scrittura (atto irreversibile)
    # ------------------------------------------------------------------

    def append(self, event: ICEEvent) -> None:
        """
        Appende un evento al log.

        L'evento si assume:
        - già validato
        - già autorizzato
        """
        with self._lock:
            self._events.append(event)

    # ------------------------------------------------------------------
    # Lettura (read-only)
    # ------------------------------------------------------------------

    def all(self) -> List[ICEEvent]:
        """
        Restituisce TUTTI gli eventi, in ordine di emissione.
        """
        with self._lock:
            return list(self._events)

    def by_run(self, run_id: RunID) -> List[ICEEvent]:
        """
        Restituisce tutti gli eventi appartenenti a un Run.
        """
        with self._lock:
            return [e for e in self._events if e.run_id == run_id]

    def last(self) -> ICEEvent | None:
        """
        Ultimo evento registrato.
        """
        with self._lock:
            return self._events[-1] if self._events else None

    # ------------------------------------------------------------------
    # Introspezione
    # ------------------------------------------------------------------

    def __len__(self) -> int:
        return len(self._events)

    def __iter__(self) -> Iterable[ICEEvent]:
        return iter(self.all())


"""
Clausola Fondativa:

- Se un evento non è nel store → NON è mai accaduto
- Il Runtime NON ha memoria fuori da questo log
- Ogni replay parte ESCLUSIVAMENTE da qui
"""
