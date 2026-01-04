from __future__ import annotations

"""
ICE Runtime — Run Context Lifecycle

Questo modulo:
- NON mantiene stato
- NON persiste nulla
- NON esegue side-effects
- Coordina nascita e distruzione di RunContext
- Emette SOLO eventi runtime

È un orchestratore, non un owner.
"""

from typing import Optional

from ice_runtime.events.kernel.emitter import EventEmitter
from ice_runtime.events.kernel.taxonomy import RuntimeEvent
from ice_runtime.runtime.state import RunState
from ice_runtime.sessions.context import RunContext
from ice_runtime.sessions.errors import SessionError


class RunContextLifecycle:
    """
    Lifecycle minimale e ICE-compliant per RunContext.
    """

    def __init__(self, emitter: EventEmitter):
        self._emitter = emitter

    # =====================================================
    # CREATION
    # =====================================================

    def create(
        self,
        *,
        run_id: str,
        agent_id: Optional[str],
        workspace_id: str,
        state: RunState,
        memory_views,
        capabilities,
        metadata,
    ) -> RunContext:
        """
        Crea una vista di contesto per un Run.
        Nessuna mutazione globale.
        """

        if state not in {RunState.CONTEXT_READY, RunState.EXECUTING}:
            raise SessionError(
                f"RunContext cannot be created in state {state.value}"
            )

        self._emitter.emit(
            RuntimeEvent.ContextViewCreated,
            run_id=run_id,
            payload={
                "agent_id": agent_id,
                "workspace_id": workspace_id,
            },
        )

        return RunContext(
            run_id=run_id,
            agent_id=agent_id,
            state=state,
            workspace_id=workspace_id,
            memory_views=memory_views,
            capabilities=capabilities,
            metadata=metadata,
        )

    # =====================================================
    # DESTRUCTION
    # =====================================================

    def destroy(self, *, run_id: str) -> None:
        """
        Distrugge logicamente una vista di contesto.
        Nessuna cleanup diretta.
        """

        self._emitter.emit(
            RuntimeEvent.ContextViewDestroyed,
            run_id=run_id,
            payload={},
        )
