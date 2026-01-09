from __future__ import annotations

"""
ICE Runtime — Run State Machine
===============================

Enforcement formale della RFC-ICE-005.

Questo modulo:
- NON introduce semantica nuova
- NON contiene logica di business
- NON emette eventi
- NON gestisce risorse

Il suo unico compito è:
→ validare transizioni di stato
→ impedire stati illegali
→ rendere impossibili shortcut

Se questo modulo è permissivo, ICE è corrotto.
"""

from typing import Final, Dict, Set


# ============================================================================
# ERRORS
# ============================================================================

class InvalidStateTransition(Exception):
    """Transizione di stato non conforme alla RFC-ICE-005."""


class RuntimeTermination(Exception):
    """Terminazione forzata imposta dal Runtime."""


# ============================================================================
# STATE MACHINE
# ============================================================================

class RunStateMachine:
    """
    State Machine sovrana di un Run.

    Proprietà fondamentali:
    - lo stato NON è mutabile direttamente
    - NON è una sorgente di verità
    - è una proiezione vincolata della storia degli eventi

    Le transizioni sono l'unico meccanismo ammesso.
    """

    # ------------------------------------------------------------------
    # STATI CANONICI (CHIUSI)
    # ------------------------------------------------------------------

    CREATED: Final[str] = "CREATED"
    PROVISIONED: Final[str] = "PROVISIONED"
    CONTEXT_READY: Final[str] = "CONTEXT_READY"
    EXECUTING: Final[str] = "EXECUTING"
    VALIDATING: Final[str] = "VALIDATING"
    COMMITTED: Final[str] = "COMMITTED"
    ABORTED: Final[str] = "ABORTED"
    TERMINATED_BY_RUNTIME: Final[str] = "TERMINATED_BY_RUNTIME"
    TERMINATED: Final[str] = "TERMINATED"

    _ALL_STATES: Final[Set[str]] = {
        CREATED,
        PROVISIONED,
        CONTEXT_READY,
        EXECUTING,
        VALIDATING,
        COMMITTED,
        ABORTED,
        TERMINATED_BY_RUNTIME,
        TERMINATED,
    }

    _TERMINAL_STATES: Final[Set[str]] = {
        TERMINATED,
    }

    # ------------------------------------------------------------------
    # TRANSITION GRAPH (RFC-ICE-005)
    # ------------------------------------------------------------------

    _TRANSITIONS: Final[Dict[str, Set[str]]] = {
        CREATED: {PROVISIONED},
        PROVISIONED: {CONTEXT_READY},
        CONTEXT_READY: {EXECUTING},
        EXECUTING: {VALIDATING, ABORTED},
        VALIDATING: {COMMITTED, ABORTED},
        COMMITTED: {TERMINATED},
        ABORTED: {TERMINATED},
        TERMINATED_BY_RUNTIME: {TERMINATED},
        TERMINATED: set(),
    }

    # ------------------------------------------------------------------
    # INIT
    # ------------------------------------------------------------------

    def __init__(self) -> None:
        self._state: str = self.CREATED

    # ------------------------------------------------------------------
    # READ API
    # ------------------------------------------------------------------

    @property
    def state(self) -> str:
        """
        Stato corrente (read-only).
        """
        return self._state

    def is_terminal(self) -> bool:
        return self._state in self._TERMINAL_STATES

    # ------------------------------------------------------------------
    # TRANSITIONS
    # ------------------------------------------------------------------

    def transition(self, to_state: str) -> None:
        """
        Esegue una transizione valida.

        Il chiamante NON specifica lo stato di partenza.
        """
        if self.is_terminal():
            raise InvalidStateTransition(
                f"Run already TERMINATED, cannot transition to {to_state}"
            )

        self._assert_valid_state(to_state)
        self._assert_transition_allowed(self._state, to_state)

        self._state = to_state

    # ------------------------------------------------------------------
    # RUNTIME-LEVEL INTERRUPTS
    # ------------------------------------------------------------------

    def abort(self) -> None:
        """
        Abort controllato del Run.

        Ammesso solo se il Run non è già terminato.
        """
        if self.is_terminal():
            return

        self._state = self.ABORTED

    def terminate_by_runtime(self) -> None:
        """
        Terminazione forzata imposta dal Runtime.

        NON equivale a ABORT:
        è un atto sovrano esterno al Run.
        """
        if self.is_terminal():
            return

        self._state = self.TERMINATED_BY_RUNTIME

    def finalize(self) -> None:
        """
        Chiusura definitiva del Run.

        Ammessa solo da stati conclusivi.
        """
        if self._state not in {
            self.COMMITTED,
            self.ABORTED,
            self.TERMINATED_BY_RUNTIME,
        }:
            raise InvalidStateTransition(
                f"Cannot finalize Run from state {self._state}"
            )

        self._state = self.TERMINATED

    # ------------------------------------------------------------------
    # INTERNALS
    # ------------------------------------------------------------------

    def _assert_valid_state(self, state: str) -> None:
        if state not in self._ALL_STATES:
            raise InvalidStateTransition(f"Unknown state: {state}")

    def _assert_transition_allowed(self, from_state: str, to_state: str) -> None:
        allowed = self._TRANSITIONS.get(from_state)
        if allowed is None or to_state not in allowed:
            raise InvalidStateTransition(
                f"Illegal transition: {from_state} → {to_state}"
            )
