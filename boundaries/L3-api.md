# Boundary L3 — API / Orchestrator

## Scope
Defines the boundary between **Foundation authority** and the **API / Orchestrator** layer.
The API performs routing, planning, and command emission, but cannot redefine axioms or invariants.

## Foundation Position
- Foundation defines validity and authority constraints.
- API/Orchestrator applies those constraints to decisions and plans.

## Non‑Responsibilities
Foundation does NOT define:
- Routing heuristics or planner strategies
- Agent selection algorithms
- LLM prompting or scoring policies

## Constraint
If orchestration decisions violate invariants, the decision is invalid — not the Foundation.
