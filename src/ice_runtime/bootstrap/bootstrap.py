from __future__ import annotations

"""
ICE Runtime Bootstrap
=====================

Bootstrap minimale del Runtime ICE.

Responsabilità:
- costruire EventKernel
- costruire EventEmitter
- istanziare Runtime
- inizializzare logging runtime-side (osservabilità)

NON:
- orchestrazione
- agenti
- LLM
- storage di dominio
"""

from pathlib import Path
from typing import Optional

from ice_runtime.runtime.runtime import Runtime
from ice_runtime.events.kernel.emitter import EventEmitter
from ice_runtime.events.kernel.store import EventStore

from ice_runtime.logging.router import LogRouter
from ice_runtime.logging.runtime import RuntimeContext
from ice_runtime.logging.api import init as init_logging
from ice_runtime.logging.sinks.stdout import StdoutSink


# ============================================================================
# PUBLIC API
# ============================================================================

def bootstrap_runtime(
    *,
    base_dir: Optional[Path] = None,
    runtime_id: str = "ice-runtime",
    enable_stdout_logs: bool = True,
) -> Runtime:
    """
    Costruisce e restituisce un Runtime ICE pronto all'uso.

    Nessun side-effect applicativo.
    Nessun engine.
    Nessuna orchestrazione.
    """

    base_dir = (base_dir or Path.cwd()).resolve()

    # ------------------------------------------------------------------
    # Logging (runtime-side, osservabilità)
    # ------------------------------------------------------------------

    ctx = RuntimeContext(
        runtime_id=runtime_id,
        base_dir=base_dir,
    )

    sinks = [StdoutSink()] if enable_stdout_logs else []
    router = LogRouter(ctx, sinks=sinks)
    init_logging(router)

    # ------------------------------------------------------------------
    # Event Kernel
    # ------------------------------------------------------------------

    store = EventStore()
    emitter = EventEmitter(store=store)

    # ------------------------------------------------------------------
    # Runtime
    # ------------------------------------------------------------------

    runtime = Runtime(emitter=emitter)
    return runtime
