# ===============================
# YAI Engine Build System (L2)
# ===============================

CC := gcc

# ---- Artifact root ----
ART_ROOT ?= $(HOME)/.yai/artifacts/yai-core

OUT_BUILD_DIR ?=
OUT_BIN_DIR ?=

BUILD_DIR := $(if $(OUT_BUILD_DIR),$(OUT_BUILD_DIR),$(ART_ROOT)/build/engine)
BIN_DIR   := $(if $(OUT_BIN_DIR),$(OUT_BIN_DIR),$(ART_ROOT)/bin)

LOG_DIR   := $(ART_ROOT)/logs
TMP_DIR   := $(ART_ROOT)/tmp
CACHE_DIR := $(ART_ROOT)/cache

# ---- ABI + Protocol source of truth ----
VAULT_ABI_SRC := ../law/specs/vault/yai_vault_abi.h
PROTOCOL_DIR  := ../law/specs/protocol

# ---- Sources ----
SRC_DIR := src
# Il find ricorsivo è perfetto, ma dobbiamo assicurarci che cJSON sia incluso 
# se si trova fuori dal path ricorsivo o ha bisogno di flag specifici
SRCS := $(shell find $(SRC_DIR) -name "*.c")

# ---- Objects ----
OBJS := $(SRCS:$(SRC_DIR)/%.c=$(BUILD_DIR)/%.o)

# ---- Compiler flags ----
# Aggiungiamo -I./src/external per cJSON
CFLAGS := -Wall -Wextra -O2 -MMD -MP \
          -I./include \
          -I./src/external \
          -I../kernel/include \
          -I../law/specs/vault \
          -I$(PROTOCOL_DIR)

# Aggiunto sqlite3 e pthread come richiesto
LDFLAGS := -lpthread -lsqlite3

# ---- Sources ----
# ---- Sources (RECURSIVE) ----
SRC_DIR := src
# Cerca tutti i .c in src e in ogni sottocartella (bridge, cortex, gates, config)
SRCS := $(shell find $(SRC_DIR) -name "*.c")
# Trasforma i path dei sorgenti in path degli oggetti mantenendo la struttura
OBJS := $(SRCS:$(SRC_DIR)/%.c=$(BUILD_DIR)/%.o)

LDFLAGS := -lpthread -lsqlite3
SRC += src/external/cJSON.c
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

# ---- Link ----
$(TARGET): $(OBJS)
	@mkdir -p $(dir $@)
	$(CC) $(OBJS) -o $@ $(LDFLAGS)
	@echo "--- [YAI-ENGINE] Build Complete ---"

# ---- Compile ----
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c | dirs
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

# ---- Cortex test ----
test-cortex: abi dirs $(TEST_CORTEX)
	@$(TEST_CORTEX)

$(TEST_CORTEX): tests/cortex_harness.c src/engine_cortex.c | dirs
	$(CC) $(CFLAGS) tests/cortex_harness.c src/engine_cortex.c -o $(TEST_CORTEX)

# ---- Debug ----
debug: CFLAGS += -g -DDEBUG
debug: all

# ---- Clean ----
clean:
	rm -rf $(BUILD_DIR)
	@echo "--- [YAI-ENGINE] Cleaned Build ---"

purge: clean
	rm -rf $(BIN_DIR) $(LOG_DIR) $(TMP_DIR) $(CACHE_DIR)

# Auto deps
-include $(OBJS:.o=.d)
