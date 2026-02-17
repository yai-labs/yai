# =========================================
# YAI — Root Control Plane
# =========================================

CC ?= cc

# ---- Repo root (root/ is one level down) ----
ROOT_DIR := $(abspath ..)

# ---- Orchestrator outputs (optional override) ----
OUT_BUILD_DIR ?= $(ROOT_DIR)/build/root
OUT_BIN_DIR   ?= $(ROOT_DIR)/dist/bin

BUILD_DIR := $(OUT_BUILD_DIR)
BIN_DIR   := $(OUT_BIN_DIR)

CFLAGS  ?= -Wall -Wextra -O2 -std=c11 -MMD -MP
LDFLAGS ?=

# ---- Externalized specs (submodule) ----
SPECS_DIR := $(ROOT_DIR)/deps/yai-specs

# ---- Include paths ----
INCLUDES := \
	-I./include \
	-I$(SPECS_DIR) \
	-I$(SPECS_DIR)/protocol \
	-I$(SPECS_DIR)/protocol/runtime \
	-I$(SPECS_DIR)/vault

# ---- Sources ----
ROOT_SRC := \
	src/yai_root_server.c \
	src/control_transport.c

# ---- Protocol runtime (implementation lives in core repo, not in specs) ----
PROTOCOL_RUNTIME_SRC := \
	$(ROOT_DIR)/runtime/protocol/rpc_runtime.c

# ---- Objects ----
OBJS_ROOT := $(patsubst %.c,$(BUILD_DIR)/%.o,$(ROOT_SRC))
PROTOCOL_RUNTIME_OBJ := $(BUILD_DIR)/protocol_runtime.o

TARGET := $(BIN_DIR)/yai-root-server

.PHONY: all dirs clean

# ===============================
# Main
# ===============================
all: dirs $(TARGET)
	@echo "--- [YAI-ROOT] Build Complete ---"

dirs:
	@mkdir -p $(BUILD_DIR) $(BIN_DIR)

# ---- Link ----
$(TARGET): $(OBJS_ROOT) $(PROTOCOL_RUNTIME_OBJ)
	@echo "[LINK] $@"
	$(CC) $^ -o $@ $(LDFLAGS)

# ---- Explicit build rules ----
$(PROTOCOL_RUNTIME_OBJ): $(PROTOCOL_RUNTIME_SRC) | dirs
	@echo "[CC] $<"
	$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@

$(BUILD_DIR)/%.o: %.c | dirs
	@mkdir -p $(dir $@)
	@echo "[CC] $<"
	$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@

clean:
	@rm -rf $(BUILD_DIR)
	@rm -f $(TARGET)
	@echo "--- [YAI-ROOT] Cleaned ---"

-include $(OBJS_ROOT:.o=.d)
-include $(PROTOCOL_RUNTIME_OBJ:.o=.d)
