CC = gcc
ART_ROOT ?= $(HOME)/.yai/artifacts/yai-core
OUT_BUILD_DIR ?=
OUT_BIN_DIR ?=
BUILD_DIR ?= $(if $(OUT_BUILD_DIR),$(OUT_BUILD_DIR),$(ART_ROOT)/build/kernel)
BIN_DIR ?= $(if $(OUT_BIN_DIR),$(OUT_BIN_DIR),$(ART_ROOT)/bin)
LOG_DIR ?= $(ART_ROOT)/logs
TMP_DIR ?= $(ART_ROOT)/tmp
CACHE_DIR ?= $(ART_ROOT)/cache

CFLAGS = -Wall -Wextra -I./include -I../law/specs/vault -O2 -MMD -MP

BOOT_SRC = src/boot/bootstrap.c src/boot/preboot.c src/bin/yai_boot_main.c
KERNEL_SRC = src/kernel/enforcement.c src/kernel/fsm.c src/kernel/ids.c src/kernel/logger.c src/kernel/project_tree.c src/kernel/transport.c src/bin/yai_kernel_main.c

BOOT_OBJ = $(patsubst %.c,$(BUILD_DIR)/%.o,$(BOOT_SRC))
KERNEL_OBJ = $(patsubst %.c,$(BUILD_DIR)/%.o,$(KERNEL_SRC))

BOOT_TARGET = $(BIN_DIR)/yai-boot
KERNEL_TARGET = $(BIN_DIR)/yai-kernel

all: boot kernel

boot: $(BOOT_TARGET)

kernel: $(KERNEL_TARGET)

dirs:
	@mkdir -p $(BUILD_DIR) $(BIN_DIR) $(LOG_DIR) $(TMP_DIR) $(CACHE_DIR)

$(BOOT_TARGET): $(BOOT_OBJ)
	@mkdir -p $(dir $@)
	$(CC) $(BOOT_OBJ) -o $(BOOT_TARGET)

$(KERNEL_TARGET): $(KERNEL_OBJ)
	@mkdir -p $(dir $@)
	$(CC) $(KERNEL_OBJ) -o $(KERNEL_TARGET)

$(BUILD_DIR)/%.o: %.c | dirs
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf $(BUILD_DIR) $(BIN_DIR)

purge: clean
	rm -rf $(LOG_DIR) $(TMP_DIR) $(CACHE_DIR)

-include $(BOOT_OBJ:.o=.d) $(KERNEL_OBJ:.o=.d)
