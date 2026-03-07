# Mind C Cognition Phase 5

## Scope
Phase 5 ports the cognitive layer from Rust into the C runtime introduced in phases 2-4.
This phase covers:
- agents
- reasoning (roles and scoring)
- orchestration planner
- RAG sessions/context/prompts/pipeline
- daemon transport integration through a cognition protocol path

This phase does not remove Rust legacy sources.

## Rust to C mapping

### Cognition root
- Rust: `src/cognition/mod.rs`
- C: `include/mind_cognition.h`, `src/cognition/cognition.c`

### Agents
- Rust: `agents/code.rs`, `historian.rs`, `knowledge.rs`, `system.rs`, `validator.rs`
- C:
  - `src/cognition/agents/agent_code.c`
  - `src/cognition/agents/agent_historian.c`
  - `src/cognition/agents/agent_knowledge.c`
  - `src/cognition/agents/agent_system.c`
  - `src/cognition/agents/agent_validator.c`
  - `src/cognition/agents/agents_dispatch.c`

### Reasoning
- Rust: `reasoning/roles.rs`, `reasoning/scoring.rs`
- C:
  - `src/cognition/reasoning/reasoning_roles.c`
  - `src/cognition/reasoning/scoring.c`

### Orchestration + planner
- Rust: `orchestration/mod.rs`, `orchestration/planner/mod.rs`
- C: `src/cognition/orchestration/planner.c`

### RAG
- Rust: `rag/context_builder.rs`, `pipeline.rs`, `prompts.rs`, `sessions.rs`
- C:
  - `src/cognition/orchestration/rag_context_builder.c`
  - `src/cognition/orchestration/rag_pipeline.c`
  - `src/cognition/orchestration/rag_prompts.c`
  - `src/cognition/orchestration/rag_sessions.c`

## Public cognition boundary
`mind_cognition.h` now provides concrete C contracts:
- `yai_mind_cognition_request_t`
- `yai_mind_cognition_response_t`
- `yai_mind_agent_role_t`
- `yai_mind_cognition_execute(...)`
- `yai_mind_cognition_execute_text(...)`
- reasoning role/score accessors

## Session and arena usage
RAG session state is stored in fixed slots and each session owns a dedicated temporary arena.
For each cognition cycle:
1. session is acquired/reset,
2. context/prompt buffers are allocated from arena,
3. agent execution consumes these buffers,
4. session remains reusable for next request.

## Minimal cognitive pipeline
Pipeline sequence:
1. acquire session
2. build plan steps
3. build retrieval context via memory query
4. build prompt from context + task
5. select role and compute score
6. execute agent
7. emit response summary/output

## Runtime integration
Transport protocol adds `COGNITION <payload>`.
Dispatch path:
- transport parse
- cognition execute text
- orchestration + memory/provider calls
- formatted protocol response

## Tests introduced
- `tests_c/test_cognition_agents.c`
- `tests_c/test_reasoning_scoring.c`
- `tests_c/test_rag_pipeline.c`
- `tests_c/test_mind_flow.c`

## Deliberate simplifications
- planner is baseline sequencing, not advanced decomposition
- mock provider remains default backend
- scoring is deterministic baseline heuristics
- RAG context uses memory query baseline, no advanced retrieval strategy yet

## Residual risks before Rust purge
- cognition heuristics still simple; planner depth is limited
- no advanced session persistence across process restart
- no multi-request UDS loop mode in this phase
- Rust legacy modules remain and must be removed in later purge phases
