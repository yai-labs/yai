# ===============================
# YAI Engine Build System (L2)
# ===============================

CC ?= gcc

# ---- Repo root inferred from engine/ ----
ROOT_DIR := $(abspath ..)

# ---- Orchestrator outputs (optional override) ----
OUT_BUILD_DIR ?= $(ROOT_DIR)/build/engine
OUT_BIN_DIR   ?= $(ROOT_DIR)/dist/bin

BUILD_DIR := $(OUT_BUILD_DIR)
BIN_DIR   := $(OUT_BIN_DIR)

LOG_DIR   := $(ROOT_DIR)/dist/logs
TMP_DIR   := $(ROOT_DIR)/dist/tmp
CACHE_DIR := $(ROOT_DIR)/dist/cache

# ---- Externalized specs (submodule) ----
SPECS_DIR := $(ROOT_DIR)/deps/yai-specs

# ---- ABI source of truth ----
VAULT_ABI_SRC := $(SPECS_DIR)/vault/yai_vault_abi.h

# ---- Source roots ----
SRC_DIR := src
THIRD_PARTY_DIR := third_party

# ---- Engine sources ----
ENGINE_SRCS := $(shell find $(SRC_DIR) -name "*.c")
ENGINE_SRCS += $(THIRD_PARTY_DIR)/cjson/cJSON.c

# ---- Protocol runtime (implementation lives in core repo, not in specs) ----
PROTOCOL_RUNTIME_SRCS := \
	$(ROOT_DIR)/runtime/protocol/rpc_runtime.c

# ---- Objects ----
OBJS_ENGINE  := $(patsubst %.c,$(BUILD_DIR)/%.o,$(ENGINE_SRCS))
OBJS_PROTO   := $(patsubst %.c,$(BUILD_DIR)/%.o,$(PROTOCOL_RUNTIME_SRCS))

OBJS := $(OBJS_ENGINE) $(OBJS_PROTO)

# ---- Compiler flags ----
CFLAGS ?= -Wall -Wextra -O2 -std=c11 -MMD -MP
CFLAGS += -I./include
CFLAGS += -I./third_party/cjson
CFLAGS += -I$(ROOT_DIR)/kernel/include

# specs headers
CFLAGS += -I$(SPECS_DIR)
CFLAGS += -I$(SPECS_DIR)/protocol
CFLAGS += -I$(SPECS_DIR)/protocol/runtime
CFLAGS += -I$(SPECS_DIR)/vault

LDFLAGS ?= -lpthread -lsqlite3

TARGET := $(BIN_DIR)/yai-engine
TEST_CORTEX := $(BUILD_DIR)/cortex_harness

.PHONY: all clean purge dirs debug test-cortex abi

# ===============================
# Main targets
# ===============================

all: abi dirs $(TARGET)
	@echo "--- [YAI-ENGINE] Build Complete ---"

abi:
	@test -f $(VAULT_ABI_SRC) || \
	 (echo "ERROR: Missing yai_vault_abi.h in deps/yai-specs/vault"; exit 1)

dirs:
	@mkdir -p $(BUILD_DIR) $(BIN_DIR) $(LOG_DIR) $(TMP_DIR) $(CACHE_DIR)

$(TARGET): $(OBJS)
	@mkdir -p $(dir $@)
	$(CC) $(OBJS) -o $@ $(LDFLAGS)

$(BUILD_DIR)/%.o: %.c | dirs
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

test-cortex: abi dirs $(TEST_CORTEX)
	@$(TEST_CORTEX)

$(TEST_CORTEX): tests/cortex_harness.c src/cortex/engine_cortex.c | dirs
	$(CC) $(CFLAGS) tests/cortex_harness.c src/cortex/engine_cortex.c -o $(TEST_CORTEX)

debug: CFLAGS += -g -DDEBUG
debug: all

clean:
	rm -rf $(BUILD_DIR)
	@echo "--- [YAI-ENGINE] Cleaned Build ---"

purge: clean
	rm -rf $(BIN_DIR) $(LOG_DIR) $(TMP_DIR) $(CACHE_DIR)

-include $(OBJS_ENGINE:.o=.d)
-include $(OBJS_PROTO:.o=.d)
