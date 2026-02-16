# =========================================
# YAI — Root Build Orchestrator
# =========================================

ROOT_DIR   := $(abspath .)
BIN_DIR    := $(ROOT_DIR)/dist/bin
BUILD_DIR  := $(ROOT_DIR)/build
DIST_DIR   := $(ROOT_DIR)/dist
VERIFY_DIR := $(DIST_DIR)/verify

BOOT_DIR   := boot
CORE_DIR   := core
KERNEL_DIR := kernel
ENGINE_DIR := engine

# Externalized specs (submodule)
SPECS_DIR  := $(ROOT_DIR)/deps/yai-specs

DOXYFILE := Doxyfile

.PHONY: all boot core kernel engine clean docs

# -----------------------------------------
# Build Order
# -----------------------------------------
all: boot core kernel engine
	@echo "--- [YAI] Build Complete ---"

# -----------------------------------------
# Boot
# -----------------------------------------
boot:
	$(MAKE) -C $(BOOT_DIR) \
	OUT_BIN_DIR=$(BIN_DIR) \
	OUT_BUILD_DIR=$(BUILD_DIR)/boot \
	EXTRA_CFLAGS="-I$(SPECS_DIR) -I$(SPECS_DIR)/protocol -I$(SPECS_DIR)/vault -I$(SPECS_DIR)/protocol/runtime" all

# -----------------------------------------
# Core (Root Plane)
# -----------------------------------------
core:
	$(MAKE) -C $(CORE_DIR) \
	OUT_BIN_DIR=$(BIN_DIR) \
	OUT_BUILD_DIR=$(BUILD_DIR)/core \
	EXTRA_CFLAGS="-I$(SPECS_DIR) -I$(SPECS_DIR)/protocol -I$(SPECS_DIR)/vault -I$(SPECS_DIR)/protocol/runtime" all

# -----------------------------------------
# Kernel
# -----------------------------------------
kernel:
	$(MAKE) -C $(KERNEL_DIR) \
	OUT_BIN_DIR=$(BIN_DIR) \
	OUT_BUILD_DIR=$(BUILD_DIR)/kernel \
	EXTRA_CFLAGS="-I$(SPECS_DIR) -I$(SPECS_DIR)/protocol -I$(SPECS_DIR)/vault -I$(SPECS_DIR)/protocol/runtime" all

# -----------------------------------------
# Engine
# -----------------------------------------
engine:
	$(MAKE) -C $(ENGINE_DIR) \
	OUT_BIN_DIR=$(BIN_DIR) \
	OUT_BUILD_DIR=$(BUILD_DIR)/engine \
	EXTRA_CFLAGS="-I$(SPECS_DIR) -I$(SPECS_DIR)/protocol -I$(SPECS_DIR)/vault -I$(SPECS_DIR)/protocol/runtime" all

# -----------------------------------------
# Clean
# -----------------------------------------
clean:
	rm -rf $(BUILD_DIR)
	rm -rf $(DIST_DIR)

# -----------------------------------------
# Docs
# -----------------------------------------
docs:
	@mkdir -p dist/docs/doxygen
	@doxygen $(DOXYFILE)
	@echo "✔ Doxygen: dist/docs/doxygen/html/index.html"
