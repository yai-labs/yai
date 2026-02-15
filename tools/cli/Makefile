# ==========================================
# YAI CLI Build System (Stable)
# ==========================================

CC := gcc

# ---- Artifact roots ----
ART_ROOT ?= $(HOME)/.yai/artifacts/yai-core

OUT_BUILD_DIR ?=
OUT_BIN_DIR ?=

BUILD_DIR := $(if $(OUT_BUILD_DIR),$(OUT_BUILD_DIR),$(CURDIR)/obj)
BIN_DIR   := $(if $(OUT_BIN_DIR),$(OUT_BIN_DIR),$(CURDIR))

TARGET := $(BIN_DIR)/yai

# ---- Include roots ----
LAW_DIR := ../../law/specs

CFLAGS := -Wall -Wextra -O2 -MMD -MP \
	-I./include \
	-I$(LAW_DIR)

LDFLAGS :=

# ---- Explicit Sources (NO find, NO magic) ----
SRCS := \
	src/main.c \
	src/cmd_engine.c \
	src/cmd_kernel.c \
	src/cmd_mind.c \
	src/cmd_root.c \
	src/cmd_ws.c \
	src/cmd_law.c \
	src/cmd_test.c \
	src/cmd_up.c \
	src/env.c \
	src/envelope.c \
	src/fmt.c \
	src/paths.c \
	src/rpc.c

# ---- Objects ----
OBJS := $(patsubst src/%.c,$(BUILD_DIR)/%.o,$(SRCS))

.PHONY: all clean dirs

# ==========================================
# Build
# ==========================================

all: dirs $(TARGET)

dirs:
	@mkdir -p $(BUILD_DIR)
	@mkdir -p $(BIN_DIR)

$(TARGET): $(OBJS)
	@echo "Linking CLI: $@"
	$(CC) $(OBJS) -o $@ $(LDFLAGS)
	@echo "--- [YAI-CLI] Build Complete ---"

# Compile rule
$(BUILD_DIR)/%.o: src/%.c | dirs
	@echo "CC $<"
	$(CC) $(CFLAGS) -c $< -o $@

# ==========================================
# Clean
# ==========================================

clean:
	rm -rf $(BUILD_DIR) $(TARGET)

-include $(OBJS:.o=.d)
