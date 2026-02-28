# ===============================
# YAI Engine Build System (L2)
# ===============================

CC ?= gcc

ROOT_DIR := $(abspath ..)

BUILD_ROOT ?= $(ROOT_DIR)/build
BIN_BUILD ?= $(BUILD_ROOT)/bin

COMPONENT_BUILD_DIR ?= $(BUILD_ROOT)/engine
BUILD_DIR := $(COMPONENT_BUILD_DIR)
BIN_DIR := $(BIN_BUILD)

SPECS_DIR := $(ROOT_DIR)/deps/yai-law
SPECS_INC_PROTOCOL := $(SPECS_DIR)/contracts/protocol/include
SPECS_INC_RUNTIME := $(SPECS_DIR)/contracts/protocol/runtime/include
SPECS_INC_VAULT := $(SPECS_DIR)/contracts/vault/include
VAULT_ABI_SRC := $(SPECS_INC_VAULT)/yai_vault_abi.h

SRC_DIR := src
THIRD_PARTY_DIR := third_party

ENGINE_SRCS := $(shell find $(SRC_DIR) -name "*.c")
ENGINE_SRCS += $(THIRD_PARTY_DIR)/cjson/cJSON.c

# Single protocol runtime artifact (Fix #2)
PROTOCOL_RUNTIME_LIB := ../build/runtime-protocol/libruntime_protocol.a

OBJS_ENGINE := $(patsubst %.c,$(BUILD_DIR)/%.o,$(ENGINE_SRCS))
OBJS := $(OBJS_ENGINE)

CFLAGS ?= -Wall -Wextra -O2 -std=c11 -MMD -MP
CFLAGS += -I./include
CFLAGS += -I./third_party/cjson
CFLAGS += -I$(ROOT_DIR)/kernel/include
CFLAGS += -I$(SPECS_DIR)
CFLAGS += -I$(SPECS_INC_PROTOCOL)
CFLAGS += -I$(SPECS_INC_RUNTIME)
CFLAGS += -I$(SPECS_INC_VAULT)

LDFLAGS ?= -lpthread -lsqlite3

TARGET := $(BIN_DIR)/yai-engine
TEST_CORTEX := $(BUILD_DIR)/tests/cortex_harness

.PHONY: all build clean dirs debug test-cortex abi

all: build

build: abi dirs $(TARGET)
	@echo "--- [YAI-ENGINE] Build Complete ---"

abi:
	@test -f $(VAULT_ABI_SRC) || \
	 (echo "ERROR: Missing yai_vault_abi.h in ../deps/yai-law/contracts/vault/include"; exit 1)

dirs:
	@mkdir -p $(BUILD_DIR) $(BIN_DIR) $(BUILD_DIR)/tests

$(TARGET): $(OBJS) $(PROTOCOL_RUNTIME_LIB)
	@mkdir -p $(dir $@)
	$(CC) $(OBJS) $(PROTOCOL_RUNTIME_LIB) -o $@ $(LDFLAGS)

$(BUILD_DIR)/%.o: %.c | dirs
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

test-cortex: abi dirs $(TEST_CORTEX)
	@$(TEST_CORTEX)

$(TEST_CORTEX): tests/cortex_harness.c src/cortex/engine_cortex.c | dirs
	$(CC) $(CFLAGS) tests/cortex_harness.c src/cortex/engine_cortex.c -o $(TEST_CORTEX)

debug: CFLAGS += -g -DDEBUG
debug: build

clean:
	rm -rf $(BUILD_DIR)
	@echo "--- [YAI-ENGINE] Cleaned Build ---"

-include $(OBJS_ENGINE:.o=.d)
