# ===============================
# YAI Boot Build System (L0)
# ===============================

CC := gcc

# ---- Artifact root ----
ART_ROOT ?= $(HOME)/.yai/artifacts/yai-core

OUT_BUILD_DIR ?=
OUT_BIN_DIR   ?=

BUILD_DIR := $(if $(OUT_BUILD_DIR),$(OUT_BUILD_DIR),$(ART_ROOT)/build/boot)
BIN_DIR   := $(if $(OUT_BIN_DIR),$(OUT_BIN_DIR),$(ART_ROOT)/bin)

LOG_DIR   := $(ART_ROOT)/logs
TMP_DIR   := $(ART_ROOT)/tmp
CACHE_DIR := $(ART_ROOT)/cache

# ---- Target ----
TARGET := $(BIN_DIR)/yai-boot

# ---- Compiler flags ----
CFLAGS := -Wall -Wextra -O2 -MMD -MP \
          -I./include \
          -I../kernel/include \
          -I../law/specs \
          -I../law/specs/vault

# ---- Sources ----
SRCS := \
    src/bootstrap.c \
    src/preboot.c \
    src/yai_boot_main.c

OBJS := $(patsubst %.c,$(BUILD_DIR)/%.o,$(SRCS))

# ---- Phony targets ----
.PHONY: all clean dirs

# ===============================
# Main build target
# ===============================
all: dirs $(TARGET)
	@echo "--- [YAI-BOOT] L0 Build Complete ---"

# ---- Directories ----
dirs:
	@mkdir -p $(BUILD_DIR) $(BIN_DIR) $(LOG_DIR) $(TMP_DIR) $(CACHE_DIR)
	@mkdir -p $(BUILD_DIR)/src  # manteniamo struttura per gli oggetti

# ---- Link Boot ----
$(TARGET): $(OBJS)
	@echo "[LINK] Linking YAI Boot..."
	$(CC) $(OBJS) -o $@

# ---- Compile individual objects ----
$(BUILD_DIR)/%.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) $(EXTRA_CFLAGS) -c $< -o $@

# ---- Clean ----
clean:
	@echo "[CLEAN] Boot build..."
	rm -rf $(BUILD_DIR)

# ---- Auto dependencies ----
-include $(OBJS:.o=.d)
