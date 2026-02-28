# ===============================
# YAI Boot Build System (L0)
# ===============================

CC ?= cc

ROOT_DIR := $(abspath ..)

BUILD_ROOT ?= $(ROOT_DIR)/build
BIN_BUILD ?= $(BUILD_ROOT)/bin

COMPONENT_BUILD_DIR ?= $(BUILD_ROOT)/boot
BUILD_DIR := $(COMPONENT_BUILD_DIR)
BIN_DIR := $(BIN_BUILD)
TARGET := $(BIN_DIR)/yai-boot

SPECS_DIR := $(ROOT_DIR)/deps/yai-law
SPECS_INC_PROTOCOL := $(SPECS_DIR)/contracts/protocol/include
SPECS_INC_RUNTIME := $(SPECS_DIR)/contracts/protocol/runtime/include
SPECS_INC_VAULT := $(SPECS_DIR)/contracts/vault/include

CFLAGS ?= -Wall -Wextra -O2 -std=c11 -MMD -MP
CFLAGS += -I./include
CFLAGS += -I$(ROOT_DIR)/kernel/include
CFLAGS += -I$(SPECS_DIR)
CFLAGS += -I$(SPECS_INC_PROTOCOL)
CFLAGS += -I$(SPECS_INC_RUNTIME)
CFLAGS += -I$(SPECS_INC_VAULT)

LDFLAGS ?=

SRCS := \
	src/preboot.c \
	src/bootstrap.c \
	src/yai_boot_main.c

OBJS := $(patsubst src/%.c,$(BUILD_DIR)/src/%.o,$(SRCS))

.PHONY: all build clean dirs

all: build

build: dirs $(TARGET)
	@echo "--- [YAI-BOOT] Build Complete ---"

dirs:
	@mkdir -p $(BUILD_DIR)/src $(BIN_DIR)

$(TARGET): $(OBJS)
	@echo "[LINK] $@"
	$(CC) $(OBJS) -o $@ $(LDFLAGS)

$(BUILD_DIR)/src/%.o: src/%.c | dirs
	@echo "[CC] $<"
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	@echo "[CLEAN] Boot build..."
	@rm -rf $(BUILD_DIR)

-include $(OBJS:.o=.d)
