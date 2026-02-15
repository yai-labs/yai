# ===============================
# YAI CLI Build System (Tools)
# ADR-002/004 aligned
# ===============================

CC := gcc

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

# ---- Sources ----
SRC_DIR := src
SRCS := $(shell find $(SRC_DIR) -name "*.c")

# ---- Objects ----
OBJS := $(SRCS:$(SRC_DIR)/%.c=$(BUILD_DIR)/%.o)

.PHONY: all clean dirs

all: dirs $(TARGET)

dirs:
	@mkdir -p $(BUILD_DIR) $(BIN_DIR)

$(TARGET): $(OBJS)
	@mkdir -p $(dir $@)
	@echo "Linking CLI: $@"
	@$(CC) $(OBJS) -o $@ $(LDFLAGS)
	@echo "--- [YAI-CLI] Build Complete ---"

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c | dirs
	@mkdir -p $(dir $@)
	@echo "CC $<"
	@$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf $(BUILD_DIR) $(TARGET)

-include $(OBJS:.o=.d)
