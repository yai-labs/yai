from __future__ import annotations


class SessionError(RuntimeError):
    """Errore base per il sottosistema sessions."""


class SessionNotFoundError(SessionError):
    def __init__(self, session_id: str):
        super().__init__(f"Session not found: {session_id}")
        self.session_id = session_id


class SessionStateError(SessionError):
    def __init__(self, message: str):
        super().__init__(message)


class WorkspaceError(RuntimeError):
    def __init__(self, workspace_id: str, message: str):
        super().__init__(f"[workspace={workspace_id}] {message}")
        self.workspace_id = workspace_id


class WorkspaceNotFoundError(WorkspaceError):
    def __init__(self, workspace_id: str):
        super().__init__(workspace_id, "Workspace not found")


class WorkspaceAlreadyExistsError(WorkspaceError):
    def __init__(self, workspace_id: str):
        super().__init__(workspace_id, "Workspace already exists")
