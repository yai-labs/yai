from __future__ import annotations

from pathlib import Path
from typing import Any, Dict, List, Optional, Protocol

from ice_core.logging.bridge import get_logger

logger = get_logger(__name__)


class FileSemanticProvider(Protocol):
    def classify(self, path: Path) -> Dict[str, Any]: ...


def default_semantic(path: Path) -> Dict[str, Any]:
    return {
        "kind": "folder" if path.is_dir() else "file",
        "language": None,
        "hidden": path.name.startswith("."),
    }


class ProjectTreeBuilder:
    """
    Runtime-only project tree view.
    """

    def __init__(
        self,
        root: str | Path,
        *,
        semantic_provider: Optional[FileSemanticProvider] = None,
        ignore_hidden: bool = False,
        max_depth: Optional[int] = None,
    ):
        self.root = Path(root).resolve()
        self.semantic_provider = semantic_provider
        self.ignore_hidden = ignore_hidden
        self.max_depth = max_depth

    def build(self) -> Dict[str, Any]:
        if not self.root.exists():
            raise FileNotFoundError(self.root)

        return {
            "root": str(self.root),
            "items": self._walk(self.root, depth=0),
        }

    def _walk(self, folder: Path, *, depth: int) -> List[Dict[str, Any]]:
        if self.max_depth is not None and depth > self.max_depth:
            return []

        nodes: List[Dict[str, Any]] = []

        for item in sorted(folder.iterdir(), key=lambda p: (p.is_file(), p.name)):
            if item.is_symlink():
                continue
            if self.ignore_hidden and item.name.startswith("."):
                continue

            nodes.append(self._node(item, depth))

        return nodes

    def _node(self, path: Path, depth: int) -> Dict[str, Any]:
        semantic = self._semantic(path)

        if path.is_dir():
            return {
                "name": path.name,
                "path": str(path),
                "type": "folder",
                "semantic": semantic,
                "children": self._walk(path, depth=depth + 1),
            }

        return {
            "name": path.name,
            "path": str(path),
            "type": "file",
            "semantic": semantic,
        }

    def _semantic(self, path: Path) -> Dict[str, Any]:
        if self.semantic_provider:
            try:
                return self.semantic_provider.classify(path)
            except Exception as e:
                logger.warning("semantic provider failed for %s: %s", path, e)
        return default_semantic(path)
