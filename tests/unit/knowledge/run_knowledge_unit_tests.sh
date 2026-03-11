#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
CONTRACT_ROOT="${YAI_GOVERNANCE_CONTRACT_ROOT:-$ROOT/governance/contracts}"
LAW_ROOT="${LAW_COMPAT_ROOT:-}"
if [[ ! -d "$CONTRACT_ROOT/protocol/include" ]]; then
  if [[ -z "$LAW_ROOT" ]]; then
    CANDIDATE="$(cd "$ROOT/.." && pwd)/law"
    [[ -d "$CANDIDATE" ]] && LAW_ROOT="$CANDIDATE"
  fi
  if [[ -z "$LAW_ROOT" ]]; then
    echo "contract root not found (expected governance/contracts or ../law/contracts)" >&2
    exit 2
  fi
  CONTRACT_ROOT="$LAW_ROOT/contracts"
fi
if [[ -z "$LAW_ROOT" ]]; then
  CANDIDATE="$(cd "$ROOT/.." && pwd)/law"
  [[ -d "$CANDIDATE" ]] && LAW_ROOT="$CANDIDATE"
fi

OUT_ROOT="$ROOT/build/test/knowledge"
OBJ_DIR="$OUT_ROOT/obj"
BIN_DIR="$OUT_ROOT/bin"
mkdir -p "$OBJ_DIR" "$BIN_DIR"

BRAIN_SRCS=(
  lib/runtime/lifecycle/runtime_capabilities.c
  lib/runtime/workspace/workspace_binding.c
  lib/runtime/workspace/workspace_recovery.c
  lib/data/binding/store_binding.c
  lib/data/binding/workspace_binding.c
  lib/knowledge/runtime_compat.c
  lib/knowledge/cognition/cognition.c
  lib/agents/dispatch/agents_dispatch.c
  lib/agents/roles/agent_code.c
  lib/agents/roles/agent_historian.c
  lib/agents/roles/agent_knowledge.c
  lib/agents/roles/agent_system.c
  lib/agents/roles/agent_validator.c
  lib/orchestration/planner/planner.c
  lib/orchestration/workflow/rag_sessions.c
  lib/orchestration/actions/rag_context_builder.c
  lib/orchestration/actions/rag_prompts.c
  lib/orchestration/execution/rag_pipeline.c
  lib/knowledge/cognition/reasoning/reasoning_roles.c
  lib/knowledge/cognition/reasoning/scoring.c
  lib/knowledge/memory/memory.c
  lib/knowledge/memory/arena_store.c
  lib/knowledge/memory/storage_bridge.c
  lib/graph/state/graph_backend.c
  lib/graph/state/graph_backend_rpc.c
  lib/graph/state/graph_facade.c
  lib/graph/state/graph.c
  lib/graph/state/ids.c
  lib/knowledge/semantic/semantic_db.c
  lib/knowledge/vector/vector_index.c
  lib/knowledge/cognition/activation.c
  lib/knowledge/memory/authority.c
  lib/knowledge/episodic/episodic.c
  lib/providers/registry/providers.c
  lib/providers/registry/provider_registry.c
  lib/providers/inference/client_inference.c
  lib/providers/embedding/client_embedding.c
  lib/providers/mocks/mock_provider.c
  lib/providers/embedding/embedder_mock.c
  lib/exec/transport/brain_transport.c
  lib/exec/transport/brain_protocol.c
  lib/exec/transport/uds_server.c
)

OBJS=()
for src in "${BRAIN_SRCS[@]}"; do
  obj="$OBJ_DIR/${src%.c}.o"
  mkdir -p "$(dirname "$obj")"
  cc -Wall -Wextra -std=c11 -O2 -I"$ROOT/include" -I"$ROOT/include/yai" -I"$CONTRACT_ROOT/protocol/include" -c "$ROOT/$src" -o "$obj"
  OBJS+=("$obj")
done

UNIT_TESTS=(
  tests/unit/knowledge/mind_legacy_tests/test_memory_graph.c
  tests/unit/knowledge/mind_legacy_tests/test_memory_domains.c
  tests/unit/knowledge/mind_legacy_tests/test_providers.c
  tests/unit/knowledge/mind_legacy_tests/test_transport.c
  tests/unit/knowledge/mind_legacy_tests/test_cognition_agents.c
  tests/unit/knowledge/mind_legacy_tests/test_reasoning_scoring.c
  tests/unit/knowledge/mind_legacy_tests/test_rag_pipeline.c
  tests/unit/knowledge/mind_legacy_tests/test_mind_flow.c
  tests/unit/knowledge/mind_legacy_tests/test_lifecycle_reinit.c
)

RAN=0
for t in "${UNIT_TESTS[@]}"; do
  if [[ ! -f "$ROOT/$t" ]]; then
    echo "knowledge_unit_tests: skip missing $t" >&2
    continue
  fi
  name="$(basename "${t%.c}")"
  cc -Wall -Wextra -std=c11 -O2 -I"$ROOT/include" -I"$ROOT/include/yai" "$ROOT/$t" "${OBJS[@]}" -o "$BIN_DIR/$name" -lm
  "$BIN_DIR/$name"
  RAN=1
done

if [[ "$RAN" -eq 0 ]]; then
  echo "knowledge_unit_tests: no C unit cases found, build-only check passed"
fi
