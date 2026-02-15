# =========================================
# YAI — Root Build Orchestrator (Law-Aligned)
# =========================================

ROOT_DIR      := $(abspath .)
ARTIFACT_ROOT := $(HOME)/.yai/artifacts/yai-core
BIN_DIR       := $(ARTIFACT_ROOT)/bin
BUILD_DIR     := $(ARTIFACT_ROOT)/build
DIST_DIR      := $(ARTIFACT_ROOT)/dist
VERIFY_DIR    := $(ARTIFACT_ROOT)/verify

BOOT_DIR    := boot
KERNEL_DIR  := kernel
ENGINE_DIR  := engine
MIND_DIR    := mind
CLI_DIR     := tools/cli

GIT_SHA    := $(shell git rev-parse --short HEAD 2>/dev/null || echo unknown)
BUILD_TIME := $(shell date -u +"%Y-%m-%dT%H:%M:%SZ")
PKG_TAG    := $(shell date -u +"%Y%m%d-%H%M%S")

PROTOCOL_IDS_VERSION := $(shell awk '/YAI_PROTOCOL_IDS_VERSION/{print $$3}' law/specs/protocol/yai_protocol_ids.h 2>/dev/null)

.PHONY: all boot kernel engine mind cli clean package verify install uninstall

# =========================================
# BUILD ORDER (deterministic)
# =========================================
# Boot first (spawns runtime)
# Kernel second (root + tenant planes)
# Engine third
# Mind fourth
# CLI LAST (depends on protocol headers)

all: boot kernel engine mind cli
	@echo "--- [YAI-ROOT] Build Complete ---"

# =========================================
# BOOT
# =========================================
boot:
	$(MAKE) -C $(BOOT_DIR) \
		OUT_BIN_DIR=$(BIN_DIR) \
		OUT_BUILD_DIR=$(BUILD_DIR)/boot \
		EXTRA_CFLAGS="-I$(ROOT_DIR)/law/specs" all

# =========================================
# KERNEL (includes root server)
# =========================================
kernel:
	$(MAKE) -C $(KERNEL_DIR) \
		OUT_BIN_DIR=$(BIN_DIR) \
		OUT_BUILD_DIR=$(BUILD_DIR)/kernel \
		EXTRA_CFLAGS="-I$(ROOT_DIR)/law/specs" all

# =========================================
# ENGINE
# =========================================
engine:
	$(MAKE) -C $(ENGINE_DIR) \
		OUT_BIN_DIR=$(BIN_DIR) \
		OUT_BUILD_DIR=$(BUILD_DIR)/engine all

# =========================================
# MIND (Rust)
# =========================================
mind:
	cargo build --release --workspace
	@mkdir -p $(BIN_DIR)
	@cp target/release/yai-mind $(BIN_DIR)/yai-mind 2>/dev/null || \
	 cp target/release/mind $(BIN_DIR)/yai-mind

# =========================================
# CLI (NO COPY — deterministic)
# =========================================
cli:
	$(MAKE) -C $(CLI_DIR) \
		OUT_BIN_DIR=$(BIN_DIR) \
		OUT_BUILD_DIR=$(BUILD_DIR)/cli

# =========================================
# INSTALL
# =========================================
PREFIX ?= /usr/local
INSTALL_BIN := $(PREFIX)/bin

install: all
	@echo "[INSTALL] Deploying YAI binaries to $(INSTALL_BIN)..."
	@sudo mkdir -p $(INSTALL_BIN)
	@sudo install -m 755 $(BIN_DIR)/yai        $(INSTALL_BIN)/yai
	@sudo install -m 755 $(BIN_DIR)/yai-boot   $(INSTALL_BIN)/yai-boot
	@sudo install -m 755 $(BIN_DIR)/yai-kernel $(INSTALL_BIN)/yai-kernel
	@sudo install -m 755 $(BIN_DIR)/yai-engine $(INSTALL_BIN)/yai-engine
	@sudo install -m 755 $(BIN_DIR)/yai-mind   $(INSTALL_BIN)/yai-mind
	@echo "✔ Installed."

uninstall:
	@echo "[UNINSTALL] Removing YAI binaries..."
	@sudo rm -f $(INSTALL_BIN)/yai*
	@echo "✔ Removed."

# =========================================
# CLEAN (full deterministic reset)
# =========================================
clean:
	$(MAKE) -C $(BOOT_DIR)   OUT_BUILD_DIR=$(BUILD_DIR)/boot clean   || true
	$(MAKE) -C $(KERNEL_DIR) OUT_BUILD_DIR=$(BUILD_DIR)/kernel clean || true
	$(MAKE) -C $(ENGINE_DIR) OUT_BUILD_DIR=$(BUILD_DIR)/engine clean || true
	$(MAKE) -C $(CLI_DIR)    OUT_BUILD_DIR=$(BUILD_DIR)/cli clean    || true
	cd $(MIND_DIR) && cargo clean || true
	rm -rf $(DIST_DIR) $(VERIFY_DIR)
	rm -rf $(BUILD_DIR)
	rm -rf $(BIN_DIR)

# =========================================
# PACKAGE
# =========================================
package: all
	@mkdir -p $(DIST_DIR)/pkg/bin
	@cp $(BIN_DIR)/* $(DIST_DIR)/pkg/bin/

	@printf '{\n  "git_sha": "%s",\n  "protocol_ids_version": %s\n}\n' \
		"$(GIT_SHA)" "$(PROTOCOL_IDS_VERSION)" > $(DIST_DIR)/pkg/MANIFEST.json

	@tar -czf $(DIST_DIR)/yai-core-$(PKG_TAG).tar.gz -C $(DIST_DIR)/pkg bin MANIFEST.json
	@echo "✔ Package created."

# =========================================
# VERIFY
# =========================================
verify: all
	@echo "[VERIFY] CLI integrity..."
	@$(BIN_DIR)/yai law check
