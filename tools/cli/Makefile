# ==========================================
# YAI CLI Build System (Standalone-Ready)
# ==========================================

CC ?= gcc

# ---- Local repo layout ----
ROOT_DIR := $(abspath .)

# Compat con orchestrator (monorepo) oppure standalone
OUT_BUILD_DIR ?= $(ROOT_DIR)/build
OUT_BIN_DIR   ?= $(ROOT_DIR)/dist/bin

BUILD_DIR := $(OUT_BUILD_DIR)
BIN_DIR   := $(OUT_BIN_DIR)

TARGET := $(BIN_DIR)/yai-cli

# ---- Protocol include (temporary monorepo link) ----
# In monorepo: ../../law/specs
# In repo standalone: puoi mettere deps/yai-specs oppure vendor/specs
LAW_DIR ?= ../../law/specs

CFLAGS ?= -Wall -Wextra -O2 -std=c11 -MMD -MP \
          -I./include \
          -I$(LAW_DIR)

LDFLAGS ?=

# ---- Sources ----
SRCS := \
    src/main.c \
    src/cmd_dispatch.c \
    src/cmd_engine.c \
    src/cmd_kernel.c \
    src/cmd_mind.c \
    src/cmd_root.c \
    src/cmd_ws.c \
    src/cmd_law.c \
    src/cmd_test.c \
    src/cmd_up.c \
    src/env.c \
    src/fmt.c \
    src/paths.c \
    src/rpc.c

OBJS := $(patsubst src/%.c,$(BUILD_DIR)/%.o,$(SRCS))

.PHONY: all clean dirs

all: dirs $(TARGET)
	@echo "--- [YAI-CLI] Build Complete ---"

dirs:
	@mkdir -p $(BUILD_DIR)
	@mkdir -p $(BIN_DIR)

$(TARGET): $(OBJS)
	@echo "[LINK] CLI: $@"
	@$(CC) $(OBJS) -o $@ $(LDFLAGS)

$(BUILD_DIR)/%.o: src/%.c | dirs
	@echo "CC $<"
	@$(CC) $(CFLAGS) -c $< -o $@

clean:
	@rm -rf $(BUILD_DIR) $(BIN_DIR)

-include $(OBJS:.o=.d)
