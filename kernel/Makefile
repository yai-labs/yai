# ===============================
# YAI Kernel Build System (L0+L1)
# ===============================

CC ?= cc

# ---- Artifact root ----
ART_ROOT  ?= $(HOME)/.yai/artifacts/yai-core
BUILD_DIR ?= $(ART_ROOT)/build/kernel
BIN_DIR   ?= $(ART_ROOT)/bin
LOG_DIR   ?= $(ART_ROOT)/logs
TMP_DIR   ?= $(ART_ROOT)/tmp
CACHE_DIR ?= $(ART_ROOT)/cache

# ---- Compiler flags ----
CFLAGS  ?= -Wall -Wextra -O2 -std=c11 -MMD -MP
LDFLAGS ?=

# ---- Include paths ----
INCLUDES := \
	-I./include \
	-I../boot/include \
	-I../core/include \
	-I../law/specs \
	-I../law/specs/protocol \
	-I../law/specs/protocol/runtime \
	-I../law/specs/vault

# -----------------------------
# L0 Boot sources
# -----------------------------
BOOT_SRC := \
	../boot/src/bootstrap.c \
	../boot/src/preboot.c

# -----------------------------
# L1 Kernel sources
# -----------------------------
KERNEL_SRC := \
	src/enforcement/enforcement.c \
	src/core/fsm.c \
	src/core/ids.c \
	src/core/logger.c \
	src/core/project_tree.c \
	src/core/transport.c \
	src/core/yai_session.c \
	src/core/rpc_binary.c \
	src/core/rpc_codec.c \
	src/bin/workspace_kernel_main.c

# -----------------------------
# External shared sources
# -----------------------------
PROTOCOL_RUNTIME_SRC := ../law/specs/protocol/runtime/rpc_runtime.c
CORE_CONTROL_SRC     := ../core/src/control_transport.c

# -----------------------------
# Objects
# -----------------------------
OBJS_BOOT   := $(patsubst %.c,$(BUILD_DIR)/%.o,$(BOOT_SRC))
OBJS_KERNEL := $(patsubst %.c,$(BUILD_DIR)/%.o,$(KERNEL_SRC))

PROTOCOL_RUNTIME_OBJ := $(BUILD_DIR)/protocol_runtime.o
CORE_CONTROL_OBJ     := $(BUILD_DIR)/control_transport.o

# Kernel binary
TARGET_KERNEL := $(BIN_DIR)/yai-kernel

# -----------------------------
# Phony targets
# -----------------------------
.PHONY: all dirs abi kernel root clean purge

all: dirs abi kernel root
	@echo "--- [YAI] Build Complete (kernel + root) ---"

# -----------------------------
# ABI guard
# -----------------------------
abi:
	@test -f ../law/specs/vault/yai_vault_abi.h || \
	(echo "ERROR: Missing yai_vault_abi.h. Run scripts/gen-vault-abi"; exit 1)

# -----------------------------
# Create directories
# -----------------------------
dirs:
	@mkdir -p $(BUILD_DIR) $(BIN_DIR) $(LOG_DIR) $(TMP_DIR) $(CACHE_DIR)

# -----------------------------
# Link kernel
# -----------------------------
kernel: $(TARGET_KERNEL)

$(TARGET_KERNEL): $(OBJS_BOOT) $(OBJS_KERNEL) $(PROTOCOL_RUNTIME_OBJ) $(CORE_CONTROL_OBJ)
	@echo "[LINK] $@"
	@mkdir -p $(dir $@)
	$(CC) $^ -o $@ $(LDFLAGS)

# -----------------------------
# Build Root Control Plane
# -----------------------------
root:
	@echo "[DELEGATE] building root control plane in ../core ..."
	@$(MAKE) -C ../core

# -----------------------------
# Explicit build rules
# -----------------------------
$(PROTOCOL_RUNTIME_OBJ): $(PROTOCOL_RUNTIME_SRC)
	@echo "[CC] $<"
	$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@

$(CORE_CONTROL_OBJ): $(CORE_CONTROL_SRC)
	@echo "[CC] $<"
	$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@

# -----------------------------
# Generic rule (kernel + boot only)
# -----------------------------
$(BUILD_DIR)/%.o: %.c
	@mkdir -p $(dir $@)
	@echo "[CC] $<"
	$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@

# -----------------------------
# Cleaning
# -----------------------------
clean:
	@echo "[CLEAN] kernel build..."
	@rm -rf $(BUILD_DIR)
	@rm -f $(TARGET_KERNEL)

purge: clean
	@rm -rf $(BIN_DIR) $(LOG_DIR) $(TMP_DIR) $(CACHE_DIR)

# -----------------------------
# Auto-dependencies
# -----------------------------
-include $(OBJS_BOOT:.o=.d)
-include $(OBJS_KERNEL:.o=.d)
-include $(PROTOCOL_RUNTIME_OBJ:.o=.d)
-include $(CORE_CONTROL_OBJ:.o=.d)
