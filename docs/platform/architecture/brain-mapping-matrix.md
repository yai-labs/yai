# Brain Mapping Matrix

| Legacy path | Target path | Action | Semantic responsibility | Rationale | Dependency impact | Test impact | Follow-up |
|---|---|---|---|---|---|---|---|
| `mind/src/cognition/cognition.c` | `lib/brain/cognition/cognition.c` | move | cognition facade | brain-native entry | consumed by `yai-core` brain lib | none | none |
| `mind/src/cognition/agents/agent_code.c` | `lib/brain/cognition/agents/agent_code.c` | move | agent execution | preserve cognitive roles | internal brain | role tests unchanged | flatten naming later |
| `mind/src/cognition/agents/agent_knowledge.c` | `lib/brain/cognition/agents/agent_knowledge.c` | move | agent execution | preserve cognitive roles | internal brain | role tests unchanged | flatten naming later |
| `mind/src/cognition/agents/agent_validator.c` | `lib/brain/cognition/agents/agent_validator.c` | move | agent execution | preserve cognitive roles | internal brain | role tests unchanged | flatten naming later |
| `mind/src/cognition/orchestration/planner.c` | `lib/brain/cognition/orchestration/planner.c` | move | planning | stable boundary | internal brain | rag tests unchanged | optional flatten |
| `mind/src/cognition/orchestration/rag_pipeline.c` | `lib/brain/cognition/orchestration/rag_pipeline.c` | move | orchestration | stable boundary | internal brain | rag tests unchanged | none |
| `mind/src/cognition/reasoning/scoring.c` | `lib/brain/cognition/reasoning/scoring.c` | move | scoring | stable boundary | internal brain | scoring tests unchanged | none |
| `mind/src/memory/graph/semantic_db.c` | `lib/brain/memory/graph/semantic_db.c` | move | semantic memory | preserve graph behavior | internal brain | memory tests unchanged | possible merge pass |
| `mind/src/memory/graph/vector_index.c` | `lib/brain/memory/graph/vector_index.c` | move | vector memory | preserve vector baseline | internal brain | memory tests unchanged | possible merge pass |
| `mind/src/memory/storage_bridge.c` | `lib/brain/memory/storage_bridge.c` | move | memory bridge | brain-local persistence bridge | internal brain | none | none |
| `mind/src/memory/arena.c` | `lib/brain/memory/arena_store.c` | rename | temp allocation | keep brain-specific allocator | internal brain | none | evaluate split to support |
| `mind/src/providers/provider_registry.c` | `lib/brain/bridge/provider_registry.c` | move | provider registry | brain provider bridge | internal brain | provider tests unchanged | none |
| `mind/src/providers/client_bridge.c` | `lib/brain/bridge/client_bridge.c` | move | provider dispatch | stable bridge boundary | internal brain | provider tests unchanged | none |
| `mind/src/providers/providers.c` | `lib/brain/bridge/providers.c` | move | provider facade | lifecycle-oriented bridge | internal brain | none | may merge later |
| `mind/src/transport/transport.c` | `lib/brain/transport/brain_transport.c` | rename | transport entry | explicit brain transport naming | internal brain | transport tests unchanged | none |
| `mind/src/transport/protocol.c` | `lib/brain/transport/brain_protocol.c` | rename | brain protocol | brain-specific, not foundation | internal brain | transport tests unchanged | split if shared later |
| `mind/src/transport/uds_server.c` | `lib/brain/transport/uds_server.c` | move | UDS runtime I/O | preserve baseline daemon transport | internal brain | daemon smoke unchanged | none |
| `mind/src/mind.c` | `lib/brain/lifecycle/brain_lifecycle.c` | rename | lifecycle | explicit brain lifecycle role | internal brain | lifecycle tests unchanged | none |
| `mind/src/main.c` | `mind/src/main.c` | keep-temporary | legacy entrypoint | compatibility target for `make -C mind` | no core dependency | unchanged | absorb into `cmd/yai-core` later |
| `mind/src/cognition/agents/agent_historian.c` | `lib/brain/cognition/agents/agent_historian.c` | move + keep-temporary | role specialization | still used in role dispatch/tests | internal brain | role tests rely on it | evaluate merge/remove |
| `mind/src/cognition/agents/agent_system.c` | `lib/brain/cognition/agents/agent_system.c` | move + keep-temporary | role specialization | still used in role dispatch/tests | internal brain | role tests rely on it | evaluate merge/remove |
| `mind/src/cognition/agents/agents_dispatch.c` | `lib/brain/cognition/agents/agents_dispatch.c` | move + keep-temporary | role dispatcher | active routing boundary | internal brain | role tests rely on it | maybe split/inline |
| `mind/src/providers/mock_provider.c` | `lib/brain/bridge/mock_provider.c` | move + keep-temporary | deterministic provider fallback | runtime baseline uses mock bootstrap | internal brain | provider tests rely on it | move-to-tests when real provider exists |
| `mind/src/providers/embedder_mock.c` | `lib/brain/bridge/embedder_mock.c` | move + keep-temporary | embedding fallback | runtime baseline uses it | internal brain | provider tests rely on it | move-to-tests when real embedder exists |
| `mind/include/mind.h` | `include/yai/brain/brain.h` | rename + wrapper | public brain API | converge public grammar | stable external include | none | remove wrapper later |
| `mind/include/mind_cognition.h` | `include/yai/brain/cognition.h` | rename + wrapper | public brain API | converge public grammar | stable external include | none | remove wrapper later |
| `mind/include/mind_memory.h` | `include/yai/brain/memory.h` | rename + wrapper | public brain API | converge public grammar | stable external include | none | remove wrapper later |
| `mind/include/mind_providers.h` | `include/yai/brain/providers.h` | rename + wrapper | public brain API | converge public grammar | stable external include | none | remove wrapper later |
| `mind/include/mind_transport.h` | `include/yai/brain/transport.h` | rename + wrapper | public brain API | converge public grammar | stable external include | none | remove wrapper later |
| `mind/include/mind_types.h` | `include/yai/brain/types.h` | split + wrapper | public type contracts | stable typed surface | lower coupling | none | remove wrapper later |
| `mind/include/mind_error.h` | `include/yai/brain/errors.h` | split + wrapper | public error contracts | stable error surface | lower coupling | none | evaluate merge with support/errors |
| `mind/tests_c/*` | `tests/unit/brain/mind_legacy_tests/*` | move-to-tests | unit/smoke tests | align with repo test topology | none | path update only | rename directory later |
