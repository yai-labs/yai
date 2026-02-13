# ===============================
# YAI Kernel Build System
# ===============================

CC := gcc

# ---- Artifact root ----
ART_ROOT ?= $(HOME)/.yai/artifacts/yai-core

OUT_BUILD_DIR ?=
OUT_BIN_DIR ?=

BUILD_DIR := $(if $(OUT_BUILD_DIR),$(OUT_BUILD_DIR),$(ART_ROOT)/build/kernel)
BIN_DIR   := $(if $(OUT_BIN_DIR),$(OUT_BIN_DIR),$(ART_ROOT)/bin)

LOG_DIR   := $(ART_ROOT)/logs
TMP_DIR   := $(ART_ROOT)/tmp
CACHE_DIR := $(ART_ROOT)/cache

# ---- ABI source (single source of truth) ----
VAULT_ABI_SRC := ../law/specs/vault/yai_vault_abi.h

# ---- Compiler flags ----
CFLAGS := -Wall -Wextra -O2 -MMD -MP \
          -I./include \
          -I../law/specs/vault

LDFLAGS :=

# ---- Sources ----
BOOT_SRC := \
	src/boot/bootstrap.c \
	src/boot/preboot.c \
	src/bin/yai_boot_main.c

KERNEL_SRC := \
	src/core/enforcement.c \
	src/core/fsm.c \
	src/core/ids.c \
	src/core/logger.c \
	src/core/project_tree.c \
	src/core/transport.c \
	src/core/yai_session.c \
	src/bin/yai_kernel_main.c \
	src/core/control_transport.c



BOOT_OBJ   := $(patsubst %.c,$(BUILD_DIR)/%.o,$(BOOT_SRC))
KERNEL_OBJ := $(patsubst %.c,$(BUILD_DIR)/%.o,$(KERNEL_SRC))

BOOT_TARGET   := $(BIN_DIR)/yai-boot
KERNEL_TARGET := $(BIN_DIR)/yai-kernel

# ---- Phony targets ----
.PHONY: all boot kernel clean purge dirs abi

# ===============================
# Main targets
# ===============================

all: abi boot kernel

boot: $(BOOT_TARGET)

kernel: $(KERNEL_TARGET)

# ===============================
# ABI Guard
# ===============================

abi:
	@test -f $(VAULT_ABI_SRC) || \
	 (echo "ERROR: Missing yai_vault_abi.h. Run scripts/gen-vault-abi"; exit 1)

# ===============================
# Build rules
# ===============================

dirs:
	@mkdir -p $(BUILD_DIR) $(BIN_DIR) $(LOG_DIR) $(TMP_DIR) $(CACHE_DIR)

$(BOOT_TARGET): $(BOOT_OBJ)
	@mkdir -p $(dir $@)
	$(CC) $(BOOT_OBJ) -o $@ $(LDFLAGS)

$(KERNEL_TARGET): $(KERNEL_OBJ)
	@mkdir -p $(dir $@)
	$(CC) $(KERNEL_OBJ) -o $@ $(LDFLAGS)

$(BUILD_DIR)/%.o: %.c | dirs
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

# ===============================
# Cleaning
# ===============================

clean:
	rm -rf $(BUILD_DIR)

purge: clean
	rm -rf $(BIN_DIR) $(LOG_DIR) $(TMP_DIR) $(CACHE_DIR)

# Auto-deps
-include $(BOOT_OBJ:.o=.d) $(KERNEL_OBJ:.o=.d)
