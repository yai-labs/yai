# =========================================
# YAI — Root Build Orchestrator
# =========================================

ROOT_DIR := $(abspath .)

BUILD_ROOT ?= $(ROOT_DIR)/build
DIST_ROOT ?= $(ROOT_DIR)/dist

BIN_BUILD := $(BUILD_ROOT)/bin
BIN_DIST := $(DIST_ROOT)/bin

BOOT_DIR := boot
ROOT_PLANE_DIR := root
KERNEL_DIR := kernel
ENGINE_DIR := engine

DOXYFILE := Doxyfile
DOXYGEN ?= doxygen
DOXY_OUT ?= $(DIST_ROOT)/docs/doxygen

CANONICAL_BINS := yai-boot yai-root-server yai-kernel yai-engine

.PHONY: all build dist bundle verify preflight-release boot root core kernel engine clean clean-dist clean-all docs docs-clean help

all: build
	@echo "[YAI] dist is now separated from build. Use 'make dist' or 'make bundle'."

build: boot root kernel engine
	@echo "--- [YAI] Build Complete ---"

dist: build
	@mkdir -p $(BIN_DIST)
	@set -e; \
	for bin in $(CANONICAL_BINS); do \
		if [ ! -f "$(BIN_BUILD)/$$bin" ]; then \
			echo "ERROR: missing build artifact $(BIN_BUILD)/$$bin (run 'make build')"; \
			exit 1; \
		fi; \
		cp "$(BIN_BUILD)/$$bin" "$(BIN_DIST)/$$bin"; \
	done
	@echo "--- [YAI] Dist staged in $(BIN_DIST) ---"

bundle: dist
	@bash tools/bundle/build_bundle.sh

boot:
	$(MAKE) -C $(BOOT_DIR) build BUILD_ROOT=$(BUILD_ROOT) BIN_BUILD=$(BIN_BUILD)

root:
	$(MAKE) -C $(ROOT_PLANE_DIR) build BUILD_ROOT=$(BUILD_ROOT) BIN_BUILD=$(BIN_BUILD)

core: root

kernel:
	$(MAKE) -C $(KERNEL_DIR) build BUILD_ROOT=$(BUILD_ROOT) BIN_BUILD=$(BIN_BUILD)

engine:
	$(MAKE) -C $(ENGINE_DIR) build BUILD_ROOT=$(BUILD_ROOT) BIN_BUILD=$(BIN_BUILD)

clean:
	rm -rf $(BUILD_ROOT)

clean-dist:
	rm -rf $(DIST_ROOT)

clean-all: clean clean-dist

verify:
	@if [ -x ./tools/bin/yai-verify ]; then \
		./tools/bin/yai-verify; \
	else \
		echo "No verify script found at ./tools/bin/yai-verify"; \
	fi

preflight-release:
	@bash tools/release/check_pins.sh

docs:
	@mkdir -p $(DOXY_OUT)
	@$(DOXYGEN) $(DOXYFILE)
	@echo "✔ Doxygen: $(DOXY_OUT)/html/index.html"

docs-clean:
	@rm -rf $(DOXY_OUT)

docs-verify:
	@tools/bin/yai-docs-trace-check --all

help:
	@echo "Targets: all, build, dist, bundle, verify, preflight-release, boot, root (core alias), kernel, engine, clean, clean-dist, clean-all, docs, docs-clean"
