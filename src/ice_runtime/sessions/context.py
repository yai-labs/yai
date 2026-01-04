from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Dict, Iterable, Optional

from ice_runtime.memory.views import MemoryView
from ice_runtime.runtime.state import RunState
from ice_runtime.sessions.errors import SessionError


@dataclass(frozen=True)
class RunContext:
    """
    ICE Runtime — Run Context (READ-ONLY)

    Questa NON è una sessione.
    Questa è una vista contestuale costruita dal Runtime
    per un singolo Run.

    Proprietà:
    - immutabile
    - non globale
    - non thread-local
    - non mutabile
    """

    run_id: str
    agent_id: Optional[str]

    state: RunState
    workspace_id: str

    memory_views: Iterable[MemoryView]
    capabilities: Iterable[str]

    metadata: Dict[str, Any]

    # =====================================================
    # ACCESS GUARANTEES
    # =====================================================

    def has_capability(self, capability: str) -> bool:
        return capability in self.capabilities

    def require_capability(self, capability: str) -> None:
        if capability not in self.capabilities:
            raise SessionError(
                f"Capability '{capability}' not granted for run {self.run_id}"
            )

    # =====================================================
    # MEMORY ACCESS (SAFE)
    # =====================================================

    def iter_memory(self) -> Iterable[MemoryView]:
        """
        Itera SOLO sulle viste già filtrate dal Runtime.
        """
        return self.memory_views

    # =====================================================
    # NO STATE, NO SETTERS
    # =====================================================

    def get_state(self) -> RunState:
        return self.state

    # =====================================================
    # SERIALIZATION (DEBUG / UI)
    # =====================================================

    def to_dict(self) -> Dict[str, Any]:
        return {
            "run_id": self.run_id,
            "agent_id": self.agent_id,
            "state": self.state.value,
            "workspace_id": self.workspace_id,
            "capabilities": list(self.capabilities),
            "memory_count": len(list(self.memory_views)),
            "metadata": dict(self.metadata),
        }
