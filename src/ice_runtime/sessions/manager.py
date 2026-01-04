from __future__ import annotations

from pathlib import Path
from typing import Dict, Optional, List

from ice_core.logging.bridge import get_logger

from .context import SessionContext, SessionConfig
from .workspace import Workspace
from .errors import (
    SessionError,
    WorkspaceNotFoundError,
)

logger = get_logger(__name__)


class SessionRegistry:
    """
    Runtime registry puro.
    """

    def __init__(self) -> None:
        self._workspaces: Dict[str, Workspace] = {}
        self._contexts: Dict[str, SessionContext] = {}

    # --- workspace -------------------------------------------------

    def register_workspace(self, ws: Workspace) -> None:
        self._workspaces[ws.id] = ws

    def get_workspace(self, workspace_id: str) -> Optional[Workspace]:
        return self._workspaces.get(workspace_id)

    def list_workspaces(self) -> List[Workspace]:
        return list(self._workspaces.values())

    # --- context ---------------------------------------------------

    def register_context(self, ctx: SessionContext) -> None:
        self._contexts[ctx.context_id] = ctx

    def unregister_context(self, context_id: str) -> None:
        self._contexts.pop(context_id, None)

    def active_contexts(self) -> List[SessionContext]:
        return list(self._contexts.values())


class SessionManager:
    """
    Runtime SessionManager.

    Coordina:
    - Workspace runtime
    - SessionContext
    """

    _instance: Optional["SessionManager"] = None

    def __init__(self, base_path: Path | str):
        self.base_path = Path(base_path).resolve()
        self.registry = SessionRegistry()
        self._initialized = False

    # --------------------------------------------------------------
    # SINGLETON
    # --------------------------------------------------------------

    @classmethod
    def get(cls) -> "SessionManager":
        if not cls._instance:
            raise SessionError("SessionManager not initialized")
        return cls._instance

    @classmethod
    def set(cls, mgr: Optional["SessionManager"]) -> None:
        cls._instance = mgr

    # --------------------------------------------------------------
    # INIT / SHUTDOWN
    # --------------------------------------------------------------

    async def initialize(self) -> None:
        if self._initialized:
            return

        self.base_path.mkdir(parents=True, exist_ok=True)
        self._initialized = True
        SessionManager.set(self)

        logger.info("SessionManager initialized")

    async def shutdown(self) -> None:
        for ctx in self.registry.active_contexts():
            try:
                await ctx.close()
            except Exception:
                pass

        for ws in self.registry.list_workspaces():
            try:
                await ws.close()
            except Exception:
                pass

        SessionManager.set(None)
        self._initialized = False

    # --------------------------------------------------------------
    # WORKSPACES
    # --------------------------------------------------------------

    async def register_workspace(
        self,
        *,
        workspace_id: str,
        name: str,
        path: Path | str,
    ) -> Workspace:
        ws = Workspace(
            workspace_id=workspace_id,
            name=name,
            base_path=path,
        )
        await ws.initialize()
        self.registry.register_workspace(ws)
        return ws

    async def get_workspace(self, workspace_id: str) -> Workspace:
        ws = self.registry.get_workspace(workspace_id)
        if not ws:
            raise WorkspaceNotFoundError(workspace_id)
        return ws

    # --------------------------------------------------------------
    # CONTEXT
    # --------------------------------------------------------------

    async def activate_workspace(
        self,
        *,
        workspace_id: str,
        config: Optional[SessionConfig] = None,
    ) -> SessionContext:
        ws = await self.get_workspace(workspace_id)

        ctx = SessionContext.create(
            workspace=ws,
            config=config,
            set_current=True,
        )

        self.registry.register_context(ctx)
        ctx.on_close(lambda c: self.registry.unregister_context(c.context_id))
        return ctx
