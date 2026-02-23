# Architecture Overview — Mind (L3)

Mind is the **cognitive plane** of YAI. It runs within governed boundaries and produces proposals, context, and memory updates—while remaining **authority-bound**.

## Responsibilities

- Build context for reasoning (RAG context builder + session orchestration)
- Propose plans (planner module) and score options (reasoning/scoring)
- Maintain memory graphs (activation/authority/episodic/semantic/vector)
- Interact with providers through an abstracted registry/client

## Non-responsibilities

Mind does not:
- execute irreversible external effects directly
- bypass authority requirements enforced by lower planes
- silently persist personal data without policy alignment

## Major modules

- `cognition/` — agents, orchestration, reasoning roles & scoring
- `memory/graph/` — graph facade + domain APIs + backend RPC
- `providers/` — embedder abstraction + provider registry and types
- `transport/` — UDS server + protocol wiring to runtime

## Integration points

- Contract surfaces are defined in `yai-specs` (canonical)
- Runtime enforcement and authority boundaries are implemented in lower planes (`yai` core)
