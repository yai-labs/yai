# =========================================
# YAI — Core (Root Control Plane)
# =========================================

CC ?= cc

# ---- Repo root (core/ is one level down) ----
ROOT_DIR := $(abspath ..)

# ---- Unified repo-local layout ----
BUILD_DIR ?= $(ROOT_DIR)/build/core
BIN_DIR   ?= $(ROOT_DIR)/dist/bin

CFLAGS  ?= -Wall -Wextra -O2 -std=c11 -MMD -MP
LDFLAGS ?=

# ---- Include paths ----
INCLUDES := \
	-I./include \
	-I$(ROOT_DIR)/law/specs \
	-I$(ROOT_DIR)/law/specs/protocol \
	-I$(ROOT_DIR)/law/specs/protocol/runtime

# ---- Sources ----
CORE_SRC := \
	src/yai_root_server.c \
	src/control_transport.c

PROTOCOL_RUNTIME_SRC := \
	$(ROOT_DIR)/law/specs/protocol/runtime/rpc_runtime.c

# ---- Objects ----
OBJS_CORE := $(patsubst %.c,$(BUILD_DIR)/%.o,$(CORE_SRC))
PROTOCOL_RUNTIME_OBJ := $(BUILD_DIR)/protocol_runtime.o

TARGET := $(BIN_DIR)/yai-root-server

.PHONY: all dirs clean

# ===============================
# Main
# ===============================
all: dirs $(TARGET)
	@echo "--- [YAI-CORE] Build Complete ---"

dirs:
	@mkdir -p $(BUILD_DIR) $(BIN_DIR)

# ---- Link ----
$(TARGET): $(OBJS_CORE) $(PROTOCOL_RUNTIME_OBJ)
	@echo "[LINK] $@"
	$(CC) $^ -o $@ $(LDFLAGS)

# ---- Explicit build rules ----
$(PROTOCOL_RUNTIME_OBJ): $(PROTOCOL_RUNTIME_SRC)
	@echo "[CC] $<"
	$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@

$(BUILD_DIR)/%.o: %.c
	@mkdir -p $(dir $@)
	@echo "[CC] $<"
	$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@

clean:
	@rm -rf $(BUILD_DIR)
	@rm -f $(TARGET)
	@echo "--- [YAI-CORE] Cleaned ---"

-include $(OBJS_CORE:.o=.d)
-include $(PROTOCOL_RUNTIME_OBJ:.o=.d)
