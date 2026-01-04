from __future__ import annotations

from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum
from pathlib import Path
from typing import Any, Dict, Optional, Protocol, List

from ice_core.logging.bridge import get_logger

logger = get_logger(__name__)


class WorkspaceState(str, Enum):
    CREATED = "created"
    INITIALIZING = "initializing"
    ACTIVE = "active"
    SUSPENDED = "suspended"
    CLOSED = "closed"
    ERROR = "error"


@dataclass
class WorkspaceMetadata:
    id: str
    name: str
    created_at: datetime = field(default_factory=datetime.utcnow)
    updated_at: datetime = field(default_factory=datetime.utcnow)
    settings: Dict[str, Any] = field(default_factory=dict)

    def touch(self) -> None:
        self.updated_at = datetime.utcnow()

    def to_dict(self) -> Dict[str, Any]:
        return {
            "id": self.id,
            "name": self.name,
            "created_at": self.created_at.isoformat(),
            "updated_at": self.updated_at.isoformat(),
            "settings": dict(self.settings),
        }


class WorkspaceLifecycleHook(Protocol):
    def on_initialize(self, ws: "Workspace") -> None: ...
    def on_activate(self, ws: "Workspace") -> None: ...
    def on_suspend(self, ws: "Workspace") -> None: ...
    def on_close(self, ws: "Workspace") -> None: ...
    def on_error(self, ws: "Workspace", error: Exception) -> None: ...


class Workspace:
    """
    Runtime Workspace.

    NON FA:
    - backend init
    - storage
    - AI wiring

    FA:
    - lifecycle
    - path isolation
    - capability + repository registry
    """

    def __init__(
        self,
        *,
        workspace_id: str,
        name: str,
        base_path: Path | str,
        metadata: Optional[WorkspaceMetadata] = None,
        lifecycle_hooks: Optional[List[WorkspaceLifecycleHook]] = None,
    ):
        self.id = workspace_id
        self.name = name
        self.base_path = Path(base_path).resolve()

        self.metadata = metadata or WorkspaceMetadata(
            id=workspace_id,
            name=name,
        )

        self.state: WorkspaceState = WorkspaceState.CREATED
        self._hooks: List[WorkspaceLifecycleHook] = list(lifecycle_hooks or [])

        self._capabilities: Dict[str, Any] = {}
        self._repositories: Dict[str, Any] = {}

        logger.debug(
            "Workspace created id=%s name=%s path=%s",
            self.id,
            self.name,
            self.base_path,
        )

    # ------------------------------------------------------------------
    # LIFECYCLE
    # ------------------------------------------------------------------

    async def initialize(self) -> None:
        if self.state != WorkspaceState.CREATED:
            return

        try:
            self.state = WorkspaceState.INITIALIZING
            self.base_path.mkdir(parents=True, exist_ok=True)
            self._trigger("on_initialize")

            self.state = WorkspaceState.ACTIVE
            self._trigger("on_activate")

        except Exception as e:
            self.state = WorkspaceState.ERROR
            self._trigger("on_error", error=e)
            raise

    async def suspend(self) -> None:
        if self.state != WorkspaceState.ACTIVE:
            return
        self.state = WorkspaceState.SUSPENDED
        self._trigger("on_suspend")

    async def close(self) -> None:
        if self.state == WorkspaceState.CLOSED:
            return

        self._trigger("on_close")
        self._capabilities.clear()
        self._repositories.clear()
        self.state = WorkspaceState.CLOSED

    # ------------------------------------------------------------------
    # REGISTRIES
    # ------------------------------------------------------------------

    def register_capability(self, name: str, obj: Any) -> None:
        self._capabilities[name] = obj

    def get_capability(self, name: str) -> Any:
        return self._capabilities.get(name)

    def register_repository(self, name: str, repo: Any) -> None:
        self._repositories[name] = repo

    def get_repository(self, name: str) -> Any:
        return self._repositories.get(name)

    # ------------------------------------------------------------------
    # HOOKS
    # ------------------------------------------------------------------

    def add_lifecycle_hook(self, hook: WorkspaceLifecycleHook) -> None:
        self._hooks.append(hook)

    def _trigger(self, event: str, **kwargs) -> None:
        for hook in self._hooks:
            try:
                fn = getattr(hook, event, None)
                if fn:
                    fn(self, **kwargs)
            except Exception as e:
                logger.error("Workspace hook error (%s): %s", event, e)

    # ------------------------------------------------------------------
    # INFO
    # ------------------------------------------------------------------

    def get_info(self) -> Dict[str, Any]:
        return {
            "id": self.id,
            "name": self.name,
            "state": self.state.value,
            "base_path": str(self.base_path),
            "metadata": self.metadata.to_dict(),
            "capabilities": list(self._capabilities.keys()),
            "repositories": list(self._repositories.keys()),
        }
