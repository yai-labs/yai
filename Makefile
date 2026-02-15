# =========================================
# YAI — Root Build Orchestrator (Law-Aligned)
# =========================================

ROOT_DIR := $(abspath .)
ARTIFACT_ROOT := $(HOME)/.yai/artifacts/yai-core
BIN_DIR       := $(ARTIFACT_ROOT)/bin
BUILD_DIR     := $(ARTIFACT_ROOT)/build
DIST_DIR      := $(ARTIFACT_ROOT)/dist
VERIFY_DIR    := $(ARTIFACT_ROOT)/verify

# Path di installazione sistema
PREFIX        ?= /usr/local
INSTALL_BIN   := $(PREFIX)/bin

KERNEL_DIR := kernel
ENGINE_DIR := engine
MIND_DIR   := mind
CLI_DIR    := tools/cli

GIT_SHA    := $(shell git rev-parse --short HEAD 2>/dev/null || echo unknown)
BUILD_TIME := $(shell date -u +"%Y-%m-%dT%H:%M:%SZ")
PKG_TAG    := $(shell date -u +"%Y%m%d-%H%M%S")

# Law extraction
PROTOCOL_IDS_VERSION  := $(shell awk '/YAI_PROTOCOL_IDS_VERSION/{print $$3}' law/specs/protocol/yai_protocol_ids.h 2>/dev/null)

.PHONY: all kernel engine mind cli clean package verify install uninstall

# =========================================
# BUILD
# =========================================

all: kernel engine mind cli

kernel:
	$(MAKE) -C $(KERNEL_DIR) \
		OUT_BIN_DIR=$(BIN_DIR) \
		OUT_BUILD_DIR=$(BUILD_DIR)/kernel \
		EXTRA_CFLAGS="-I$(ROOT_DIR)/law/specs" all

engine:
	$(MAKE) -C $(ENGINE_DIR) \
		OUT_BIN_DIR=$(BIN_DIR) \
		OUT_BUILD_DIR=$(BUILD_DIR)/engine all

mind:
	cargo build --release --workspace
	@mkdir -p $(BIN_DIR)
	# Essendo un workspace, il binario è in ./target/release/
	@cp target/release/yai-mind $(BIN_DIR)/yai-mind || cp target/release/mind $(BIN_DIR)/yai-mind

cli:
	$(MAKE) -C $(CLI_DIR)
	@mkdir -p $(BIN_DIR)
	@cp $(CLI_DIR)/yai $(BIN_DIR)/yai

# =========================================
# INSTALL (Sovereign, PATH-friendly)
# =========================================

UNAME_S := $(shell uname -s 2>/dev/null)
UNAME_M := $(shell uname -m 2>/dev/null)

# macOS Homebrew default:
# - arm64: /opt/homebrew/bin
# - intel: /usr/local/bin
DEFAULT_PREFIX := /usr/local
ifeq ($(UNAME_S),Darwin)
  ifeq ($(UNAME_M),arm64)
    DEFAULT_PREFIX := /opt/homebrew
  endif
endif

PREFIX      ?= $(DEFAULT_PREFIX)
INSTALL_BIN := $(PREFIX)/bin

.PHONY: install uninstall

install: all
	@echo "[INSTALL] Deploying YAI binaries to $(INSTALL_BIN)..."
	@sudo mkdir -p $(INSTALL_BIN)
	@sudo install -m 755 $(BIN_DIR)/yai        $(INSTALL_BIN)/yai
	@sudo install -m 755 $(BIN_DIR)/yai-kernel $(INSTALL_BIN)/yai-kernel
	@sudo install -m 755 $(BIN_DIR)/yai-engine $(INSTALL_BIN)/yai-engine
	@sudo install -m 755 $(BIN_DIR)/yai-boot   $(INSTALL_BIN)/yai-boot
	@sudo install -m 755 $(BIN_DIR)/yai-mind   $(INSTALL_BIN)/yai-mind
	@echo "✔ Installed."
	@echo "Try:"
	@echo "  yai law check"
	@echo "  yai up"
	@echo "  yai kernel status"
	@echo "If 'yai' is not found, ensure $(INSTALL_BIN) is in your PATH."

uninstall:
	@echo "[UNINSTALL] Removing YAI binaries from $(INSTALL_BIN)..."
	@sudo rm -f $(INSTALL_BIN)/yai
	@sudo rm -f $(INSTALL_BIN)/yai-kernel
	@sudo rm -f $(INSTALL_BIN)/yai-engine
	@sudo rm -f $(INSTALL_BIN)/yai-boot
	@sudo rm -f $(INSTALL_BIN)/yai-mind
	@echo "✔ Removed."


# =========================================
# CLEAN
# =========================================

clean:
	$(MAKE) -C $(KERNEL_DIR) OUT_BIN_DIR=$(BIN_DIR) OUT_BUILD_DIR=$(BUILD_DIR)/kernel clean || true
	$(MAKE) -C $(ENGINE_DIR) OUT_BIN_DIR=$(BIN_DIR) OUT_BUILD_DIR=$(BUILD_DIR)/engine clean || true
	$(MAKE) -C $(CLI_DIR) clean || true
	cd $(MIND_DIR) && cargo clean || true
	rm -rf $(DIST_DIR) $(VERIFY_DIR)

# =========================================
# PACKAGE
# =========================================

package: all
	@mkdir -p $(DIST_DIR)/pkg/bin
	@cp $(BIN_DIR)/yai-boot   $(DIST_DIR)/pkg/bin/
	@cp $(BIN_DIR)/yai-kernel $(DIST_DIR)/pkg/bin/
	@cp $(BIN_DIR)/yai-engine $(DIST_DIR)/pkg/bin/
	@cp $(BIN_DIR)/yai        $(DIST_DIR)/pkg/bin/yai
	@cp $(BIN_DIR)/yai-mind   $(DIST_DIR)/pkg/bin/yai-mind

	@printf '{\n  "git_sha": "%s",\n  "protocol_ids_version": %s\n}\n' \
		"$(GIT_SHA)" "$(PROTOCOL_IDS_VERSION)" > $(DIST_DIR)/pkg/MANIFEST.json
	
	@tar -czf $(DIST_DIR)/yai-core-$(PKG_TAG).tar.gz -C $(DIST_DIR)/pkg bin MANIFEST.json
	@echo "✔ Package created: $(DIST_DIR)/yai-core-$(PKG_TAG).tar.gz"

verify: all
	@echo "[VERIFY] Checking CLI integrity..."
	@$(BIN_DIR)/yai law check