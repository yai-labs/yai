# ===============================
# YAI Engine Build System (L2)
# ===============================

CC := gcc

# ---- Artifact root ----
ART_ROOT ?= $(HOME)/.yai/artifacts/yai-core

BUILD_DIR ?= $(ART_ROOT)/build/engine
BIN_DIR   ?= $(ART_ROOT)/bin

LOG_DIR   := $(ART_ROOT)/logs
TMP_DIR   := $(ART_ROOT)/tmp
CACHE_DIR := $(ART_ROOT)/cache

# ---- ABI + Protocol source of truth ----
VAULT_ABI_SRC := ../law/specs/vault/yai_vault_abi.h
PROTOCOL_DIR  := ../law/specs/protocol
PROTOCOL_RUNTIME_DIR := ../law/specs/protocol/runtime

# ---- Source roots ----
SRC_DIR := src
THIRD_PARTY_DIR := third_party

# ---- Engine sources ----
ENGINE_SRCS := $(shell find $(SRC_DIR) -name "*.c")
ENGINE_SRCS += $(THIRD_PARTY_DIR)/cjson/cJSON.c

# ---- Protocol runtime (NEW) ----
PROTOCOL_RUNTIME_SRCS := \
	$(PROTOCOL_RUNTIME_DIR)/rpc_runtime.c

# ---- Objects ----
OBJS_ENGINE  := $(patsubst %.c,$(BUILD_DIR)/%.o,$(ENGINE_SRCS))
OBJS_PROTO   := $(patsubst %.c,$(BUILD_DIR)/%.o,$(PROTOCOL_RUNTIME_SRCS))

OBJS := $(OBJS_ENGINE) $(OBJS_PROTO)

# ---- Compiler flags ----
CFLAGS := -Wall -Wextra -O2 -std=c11 -MMD -MP \
          -I./include \
          -I./third_party/cjson \
          -I../kernel/include \
          -I../law/specs \
          -I$(PROTOCOL_DIR) \
          -I$(PROTOCOL_RUNTIME_DIR) \
          -I../law/specs/vault

LDFLAGS := -lpthread -lsqlite3

TARGET := $(BIN_DIR)/yai-engine
TEST_CORTEX := $(BUILD_DIR)/cortex_harness

# ---- Phony ----
.PHONY: all clean purge dirs debug test-cortex abi

# ===============================
# Main targets
# ===============================
all: abi dirs $(TARGET)

# ---- ABI guard ----
abi:
	@test -f $(VAULT_ABI_SRC) || \
	 (echo "ERROR: Missing yai_vault_abi.h. Run scripts/gen-vault-abi"; exit 1)

# ---- Directories ----
dirs:
	@mkdir -p $(BUILD_DIR) $(BIN_DIR) $(LOG_DIR) $(TMP_DIR) $(CACHE_DIR)

# ---- Link Engine ----
$(TARGET): $(OBJS)
	@mkdir -p $(dir $@)
	$(CC) $(OBJS) -o $@ $(LDFLAGS)
	@echo "--- [YAI-ENGINE] Build Complete ---"

# ---- Compile (generic rule) ----
$(BUILD_DIR)/%.o: %.c | dirs
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

# ---- Cortex test harness ----
test-cortex: abi dirs $(TEST_CORTEX)
	@$(TEST_CORTEX)

$(TEST_CORTEX): tests/cortex_harness.c src/cortex/engine_cortex.c | dirs
	$(CC) $(CFLAGS) tests/cortex_harness.c src/cortex/engine_cortex.c -o $(TEST_CORTEX)

# ---- Debug build ----
debug: CFLAGS += -g -DDEBUG
debug: all

# ---- Clean ----
clean:
	rm -rf $(BUILD_DIR)
	@echo "--- [YAI-ENGINE] Cleaned Build ---"

purge: clean
	rm -rf $(BIN_DIR) $(LOG_DIR) $(TMP_DIR) $(CACHE_DIR)

# ---- Auto deps ----
-include $(OBJS_ENGINE:.o=.d)
-include $(OBJS_PROTO:.o=.d)
