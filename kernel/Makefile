# ===============================
# YAI Kernel Build System (L0+L1)
# ===============================

CC := gcc

# ---- Artifact root ----
ART_ROOT ?= $(HOME)/.yai/artifacts/yai-core

OUT_BUILD_DIR ?=
OUT_BIN_DIR   ?=

BUILD_DIR := $(if $(OUT_BUILD_DIR),$(OUT_BUILD_DIR),$(ART_ROOT)/build/kernel)
BIN_DIR   := $(if $(OUT_BIN_DIR),$(OUT_BIN_DIR),$(ART_ROOT)/bin)

# ---- Directory di Sistema ----
LOG_DIR   := $(ART_ROOT)/logs
TMP_DIR   := $(ART_ROOT)/tmp
CACHE_DIR := $(ART_ROOT)/cache

# ---- Compiler flags ----
# Includiamo ../boot/include per trovare la logica di Ignition
CFLAGS := -Wall -Wextra -O2 -MMD -MP \
          -I./include \
          -I../boot/include \
          -I../law/specs \
          -I../law/specs/protocol \
          -I../law/specs/vault


LDFLAGS :=

# ---- Sources Hierarchy ----

# L0: Ignition (Ora puntano alla ROOT)
BOOT_SRC := \
    ../boot/src/bootstrap.c \
    ../boot/src/preboot.c

# L1: Authority (Il cuore del Kernel)
KERNEL_CORE_SRC := \
    src/enforcement/enforcement.c \
    src/core/fsm.c \
    src/core/ids.c \
    src/core/logger.c \
    src/core/project_tree.c \
    src/core/transport.c \
    src/core/yai_session.c \
    src/core/control_transport.c \
    src/bin/yai_kernel_main.c \
    src/core/rpc_binary.c \


# Unifichiamo tutto nell'unico binario sovrano
SRCS := $(BOOT_SRC) $(KERNEL_CORE_SRC)
OBJS := $(patsubst %.c,$(BUILD_DIR)/%.o,$(SRCS))

TARGET := $(BIN_DIR)/yai-kernel

# ---- Phony targets ----
.PHONY: all clean purge dirs abi

# ===============================
# Main targets
# ===============================

all: dirs abi $(TARGET)
	@echo "--- [YAI-KERNEL] L0+L1 Build Complete ---"

# ===============================
# ABI Guard
# ===============================

abi:
	@test -f ../law/specs/vault/yai_vault_abi.h || \
	 (echo "ERROR: Missing yai_vault_abi.h. Run scripts/gen-vault-abi"; exit 1)

# ===============================
# Build rules
# ===============================

dirs:
	@mkdir -p $(BUILD_DIR) $(BIN_DIR) $(LOG_DIR) $(TMP_DIR) $(CACHE_DIR)
	@mkdir -p $(BUILD_DIR)/../boot/src # Necessario per gli oggetti fuori dalla dir locale

$(TARGET): $(OBJS)
	@echo "Linking Sovereign Kernel..."
	@mkdir -p $(dir $@)
	$(CC) $(OBJS) -o $@ $(LDFLAGS)

# Regola generica per compilare file sia locali che esterni (../boot)
$(BUILD_DIR)/%.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) $(EXTRA_CFLAGS) -c $< -o $@

# ===============================
# Cleaning
# ===============================

clean:
	@echo "Cleaning Kernel build..."
	rm -rf $(BUILD_DIR)

purge: clean
	rm -rf $(BIN_DIR) $(LOG_DIR) $(TMP_DIR) $(CACHE_DIR)

# Auto-deps
-include $(OBJS:.o=.d)