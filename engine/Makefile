# Progetto: YAI-ENGINE Core
# Target: Eseguibile binario C-only (no Python)

CC = gcc
ART_ROOT ?= $(HOME)/.yai/artifacts/yai-core
OUT_BUILD_DIR ?=
OUT_BIN_DIR ?=
BUILD_DIR ?= $(if $(OUT_BUILD_DIR),$(OUT_BUILD_DIR),$(ART_ROOT)/build/engine)
BIN_DIR ?= $(if $(OUT_BIN_DIR),$(OUT_BIN_DIR),$(ART_ROOT)/bin)
LOG_DIR ?= $(ART_ROOT)/logs
TMP_DIR ?= $(ART_ROOT)/tmp
CACHE_DIR ?= $(ART_ROOT)/cache

CFLAGS = -Wall -Wextra -O2 -I./include -I../kernel/include -I../law/specs/vault -I../law/specs/protocol -MMD -MP
LDFLAGS = -lpthread

# Directory
SRC_DIR = src
TARGET = $(BIN_DIR)/yai-engine
TEST_CORTEX = $(BUILD_DIR)/cortex_harness

# Sorgenti e Oggetti
SRCS = $(wildcard $(SRC_DIR)/*.c)
OBJS = $(SRCS:$(SRC_DIR)/%.c=$(BUILD_DIR)/%.o)

# Regola Principale
all: dirs $(TARGET)

# Creazione cartelle
dirs:
	@mkdir -p $(BUILD_DIR) $(BIN_DIR) $(LOG_DIR) $(TMP_DIR) $(CACHE_DIR)

# Link finale
$(TARGET): $(OBJS)
	@mkdir -p $(dir $@)
	$(CC) $(OBJS) -o $(TARGET) $(LDFLAGS)
	@echo "--- [YAI-ENGINE] Build Complete ---"

# Compilazione singoli file .c
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c | dirs
	$(CC) $(CFLAGS) -c $< -o $@

test-cortex: dirs $(TEST_CORTEX)
	@$(TEST_CORTEX)

$(TEST_CORTEX): tests/cortex_harness.c src/engine_cortex.c | dirs
	$(CC) $(CFLAGS) tests/cortex_harness.c src/engine_cortex.c -o $(TEST_CORTEX)

# Pulizia
clean:
	rm -rf $(BUILD_DIR) $(BIN_DIR)
	@echo "--- [YAI-ENGINE] Cleaned ---"

purge: clean
	rm -rf $(LOG_DIR) $(TMP_DIR) $(CACHE_DIR)

# Debug mode
debug: CFLAGS += -g -DDEBUG
debug: all

.PHONY: all clean purge dirs debug test-cortex

-include $(OBJS:.o=.d)
