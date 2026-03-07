CC ?= cc
CFLAGS ?= -Wall -Wextra -std=c11 -O2
ROOT := $(abspath .)
BUILD_DIR := $(ROOT)/build
BIN_DIR := $(ROOT)/dist/bin
TEST_BIN_DIR := $(BUILD_DIR)/tests_c
TARGET := $(BIN_DIR)/yai-mind

INCLUDES := -I$(ROOT)/include

SRCS := \
  src/main.c \
  src/mind.c \
  src/cognition/cognition.c \
  src/cognition/agents/agents_dispatch.c \
  src/cognition/agents/agent_code.c \
  src/cognition/agents/agent_historian.c \
  src/cognition/agents/agent_knowledge.c \
  src/cognition/agents/agent_system.c \
  src/cognition/agents/agent_validator.c \
  src/cognition/orchestration/planner.c \
  src/cognition/orchestration/rag_sessions.c \
  src/cognition/orchestration/rag_context_builder.c \
  src/cognition/orchestration/rag_prompts.c \
  src/cognition/orchestration/rag_pipeline.c \
  src/cognition/reasoning/reasoning_roles.c \
  src/cognition/reasoning/scoring.c \
  src/memory/memory.c \
  src/memory/arena.c \
  src/memory/graph/graph_backend.c \
  src/memory/graph/graph_backend_rpc.c \
  src/memory/graph/graph_facade.c \
  src/memory/graph/graph.c \
  src/memory/graph/ids.c \
  src/memory/graph/domain_activation.c \
  src/memory/graph/domain_authority.c \
  src/memory/graph/domain_episodic.c \
  src/memory/graph/domain_semantic.c \
  src/memory/graph/semantic_db.c \
  src/memory/graph/vector_index.c \
  src/memory/storage_bridge.c \
  src/providers/providers.c \
  src/providers/provider_registry.c \
  src/providers/client_bridge.c \
  src/providers/mock_provider.c \
  src/providers/embedder_mock.c \
  src/transport/transport.c \
  src/transport/protocol.c \
  src/transport/uds_server.c

OBJS := $(patsubst %.c,$(BUILD_DIR)/%.o,$(SRCS))
DEPS := $(OBJS:.o=.d)

TEST_GRAPH := $(TEST_BIN_DIR)/test_memory_graph
TEST_DOMAINS := $(TEST_BIN_DIR)/test_memory_domains
TEST_PROVIDERS := $(TEST_BIN_DIR)/test_providers
TEST_TRANSPORT := $(TEST_BIN_DIR)/test_transport
TEST_DAEMON := $(TEST_BIN_DIR)/test_mind_daemon_smoke
TEST_COG_AGENTS := $(TEST_BIN_DIR)/test_cognition_agents
TEST_REASONING := $(TEST_BIN_DIR)/test_reasoning_scoring
TEST_RAG := $(TEST_BIN_DIR)/test_rag_pipeline
TEST_FLOW := $(TEST_BIN_DIR)/test_mind_flow
TEST_RUNTIME_PRIMARY := $(TEST_BIN_DIR)/test_runtime_primary
TEST_REINIT := $(TEST_BIN_DIR)/test_lifecycle_reinit

LIB_SRCS := $(filter-out src/main.c,$(SRCS))
LIB_OBJS := $(patsubst %.c,$(BUILD_DIR)/%.o,$(LIB_SRCS))

.PHONY: all clean check test run dirs

all: dirs $(TARGET)

clean:
	rm -rf $(BUILD_DIR) $(BIN_DIR)

dirs:
	@mkdir -p $(BUILD_DIR) $(BIN_DIR) $(TEST_BIN_DIR)

$(TARGET): $(OBJS)
	$(CC) $(OBJS) -o $@ -lm

$(BUILD_DIR)/%.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) $(INCLUDES) -MMD -MP -c $< -o $@

$(TEST_GRAPH): tests_c/test_memory_graph.c $(LIB_OBJS)
	$(CC) $(CFLAGS) $(INCLUDES) $^ -o $@ -lm

$(TEST_DOMAINS): tests_c/test_memory_domains.c $(LIB_OBJS)
	$(CC) $(CFLAGS) $(INCLUDES) $^ -o $@ -lm

$(TEST_PROVIDERS): tests_c/test_providers.c $(LIB_OBJS)
	$(CC) $(CFLAGS) $(INCLUDES) $^ -o $@ -lm

$(TEST_TRANSPORT): tests_c/test_transport.c $(LIB_OBJS)
	$(CC) $(CFLAGS) $(INCLUDES) $^ -o $@ -lm

$(TEST_DAEMON): tests_c/test_mind_daemon_smoke.c
	$(CC) $(CFLAGS) $(INCLUDES) $< -o $@

$(TEST_COG_AGENTS): tests_c/test_cognition_agents.c $(LIB_OBJS)
	$(CC) $(CFLAGS) $(INCLUDES) $^ -o $@ -lm

$(TEST_REASONING): tests_c/test_reasoning_scoring.c $(LIB_OBJS)
	$(CC) $(CFLAGS) $(INCLUDES) $^ -o $@ -lm

$(TEST_RAG): tests_c/test_rag_pipeline.c $(LIB_OBJS)
	$(CC) $(CFLAGS) $(INCLUDES) $^ -o $@ -lm

$(TEST_FLOW): tests_c/test_mind_flow.c $(LIB_OBJS)
	$(CC) $(CFLAGS) $(INCLUDES) $^ -o $@ -lm

$(TEST_RUNTIME_PRIMARY): tests_c/test_runtime_primary.c $(LIB_OBJS)
	$(CC) $(CFLAGS) $(INCLUDES) $^ -o $@ -lm

$(TEST_REINIT): tests_c/test_lifecycle_reinit.c $(LIB_OBJS)
	$(CC) $(CFLAGS) $(INCLUDES) $^ -o $@ -lm

check: all
	$(TARGET) --check

run: all
	$(TARGET)

test: check $(TEST_GRAPH) $(TEST_DOMAINS) $(TEST_PROVIDERS) $(TEST_TRANSPORT) $(TEST_DAEMON) $(TEST_COG_AGENTS) $(TEST_REASONING) $(TEST_RAG) $(TEST_FLOW) $(TEST_RUNTIME_PRIMARY) $(TEST_REINIT)
	$(TEST_GRAPH)
	$(TEST_DOMAINS)
	$(TEST_PROVIDERS)
	$(TEST_TRANSPORT)
	$(TEST_DAEMON)
	$(TEST_COG_AGENTS)
	$(TEST_REASONING)
	$(TEST_RAG)
	$(TEST_FLOW)
	$(TEST_RUNTIME_PRIMARY)
	$(TEST_REINIT)

-include $(DEPS)
