# =========================================
# YAI — Root Build Orchestrator
# =========================================

ROOT_DIR   := $(abspath .)
BIN_DIR    := $(ROOT_DIR)/dist/bin
BUILD_DIR  := $(ROOT_DIR)/build
DIST_DIR   := $(ROOT_DIR)/dist
VERIFY_DIR := $(DIST_DIR)/verify

BOOT_DIR   := boot
ROOT_PLANE_DIR := root
KERNEL_DIR := kernel
ENGINE_DIR := engine

# Externalized specs (submodule)
SPECS_DIR  := $(ROOT_DIR)/deps/yai-specs

DOXYFILE := Doxyfile
DOXYGEN ?= doxygen
DOXY_OUT ?= dist/docs/doxygen

.PHONY: all boot root core kernel engine clean docs docs-clean help

# -----------------------------------------
# Build Order
# -----------------------------------------
all: docs boot root kernel engine
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
# Root Plane
# -----------------------------------------
root:
	$(MAKE) -C $(ROOT_PLANE_DIR) \
	OUT_BIN_DIR=$(BIN_DIR) \
	OUT_BUILD_DIR=$(BUILD_DIR)/root \
	EXTRA_CFLAGS="-I$(SPECS_DIR) -I$(SPECS_DIR)/protocol -I$(SPECS_DIR)/vault -I$(SPECS_DIR)/protocol/runtime" all

# Compatibility alias
core: root

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
	@mkdir -p $(DOXY_OUT)
	@$(DOXYGEN) $(DOXYFILE)
	@echo "✔ Doxygen: $(DOXY_OUT)/html/index.html"

docs-clean:
	@rm -rf $(DOXY_OUT)

# -----------------------------------------
# Help
# -----------------------------------------
help:
	@echo "Targets: all, boot, root (core alias), kernel, engine, docs, docs-clean, clean"
