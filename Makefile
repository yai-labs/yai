# =========================================
# YAI — Root Build Orchestrator (Law-Aligned)
# =========================================

ROOT_DIR := $(abspath .)

ARTIFACT_ROOT := $(HOME)/.yai/artifacts/yai-core
BIN_DIR       := $(ARTIFACT_ROOT)/bin
BUILD_DIR     := $(ARTIFACT_ROOT)/build
DIST_DIR      := $(ARTIFACT_ROOT)/dist
VERIFY_DIR    := $(ARTIFACT_ROOT)/verify

KERNEL_DIR := kernel
ENGINE_DIR := engine
MIND_DIR   := mind

GIT_SHA    := $(shell git rev-parse --short HEAD 2>/dev/null || echo unknown)
BUILD_TIME := $(shell date -u +"%Y-%m-%dT%H:%M:%SZ")
PKG_TAG    := $(shell date -u +"%Y%m%d-%H%M%S")

VAULT_ABI_VERSION     := $(shell awk '/YAI_VAULT_ABI_VERSION/{print $$3}' law/specs/vault/yai_vault_abi.h 2>/dev/null)
VAULT_LAYOUT_BYTES    := $(shell awk '/YAI_VAULT_LAYOUT_BYTES/{print $$3}' law/specs/vault/yai_vault_abi.h 2>/dev/null)
VAULT_HEADER_SIZE     := $(shell awk '/YAI_VAULT_HEADER_SIZE/{print $$3}' law/specs/vault/yai_vault_abi.h 2>/dev/null)
PROTOCOL_IDS_VERSION  := $(shell awk '/YAI_PROTOCOL_IDS_VERSION/{print $$3}' law/specs/protocol/yai_protocol_ids.h 2>/dev/null)

.PHONY: all kernel engine mind clean package verify \
        verify-abi verify-protocol verify-binaries verify-hash verify-tests

# =========================================
# BUILD
# =========================================

all: kernel engine mind

kernel:
	$(MAKE) -C $(KERNEL_DIR) \
		OUT_BIN_DIR=$(BIN_DIR) \
		OUT_BUILD_DIR=$(BUILD_DIR)/kernel all

engine:
	$(MAKE) -C $(ENGINE_DIR) \
		OUT_BIN_DIR=$(BIN_DIR) \
		OUT_BUILD_DIR=$(BUILD_DIR)/engine all

mind:
	cd $(MIND_DIR) && cargo build --release

# =========================================
# CLEAN
# =========================================

clean:
	$(MAKE) -C $(KERNEL_DIR) \
		OUT_BIN_DIR=$(BIN_DIR) \
		OUT_BUILD_DIR=$(BUILD_DIR)/kernel clean || true
	$(MAKE) -C $(ENGINE_DIR) \
		OUT_BIN_DIR=$(BIN_DIR) \
		OUT_BUILD_DIR=$(BUILD_DIR)/engine clean || true
	cd $(MIND_DIR) && cargo clean || true
	rm -rf $(DIST_DIR) $(VERIFY_DIR)

# =========================================
# PACKAGE
# =========================================

package: all
	@mkdir -p $(DIST_DIR)/pkg/bin

	@test -f $(BIN_DIR)/yai-boot
	@test -f $(BIN_DIR)/yai-kernel
	@test -f $(BIN_DIR)/yai-engine
	@test -f $(MIND_DIR)/target/release/yai

	@cp $(BIN_DIR)/yai-boot   $(DIST_DIR)/pkg/bin/
	@cp $(BIN_DIR)/yai-kernel $(DIST_DIR)/pkg/bin/
	@cp $(BIN_DIR)/yai-engine $(DIST_DIR)/pkg/bin/
	@cp $(MIND_DIR)/target/release/yai $(DIST_DIR)/pkg/bin/yai-mind

	@printf '{\n'                                >  $(DIST_DIR)/pkg/MANIFEST.json
	@printf '  "git_sha": "%s",\n' "$(GIT_SHA)"   >> $(DIST_DIR)/pkg/MANIFEST.json
	@printf '  "build_time": "%s",\n' "$(BUILD_TIME)" >> $(DIST_DIR)/pkg/MANIFEST.json
	@printf '  "protocol_ids_version": %s,\n' "$(PROTOCOL_IDS_VERSION)" >> $(DIST_DIR)/pkg/MANIFEST.json
	@printf '  "vault_abi_version": %s,\n' "$(VAULT_ABI_VERSION)" >> $(DIST_DIR)/pkg/MANIFEST.json
	@printf '  "vault_layout_bytes": %s,\n' "$(VAULT_LAYOUT_BYTES)" >> $(DIST_DIR)/pkg/MANIFEST.json
	@printf '  "vault_header_size": %s\n' "$(VAULT_HEADER_SIZE)" >> $(DIST_DIR)/pkg/MANIFEST.json
	@printf '}\n' >> $(DIST_DIR)/pkg/MANIFEST.json

	@tar -czf $(DIST_DIR)/yai-core-$(PKG_TAG).tar.gz -C $(DIST_DIR)/pkg bin MANIFEST.json
	@rm -rf $(DIST_DIR)/pkg

	@echo ""
	@echo "✔ Package created: $(DIST_DIR)/yai-core-$(PKG_TAG).tar.gz"

# =========================================
# VERIFY — L0 GATE
# =========================================

verify: all verify-abi verify-protocol verify-binaries verify-hash verify-tests
	@echo ""
	@echo "=================================="
	@echo "✔ YAI L0 VERIFY PASSED"
	@echo "=================================="

verify-abi:
	@echo "[VERIFY] Vault ABI..."
	@test -f law/specs/vault/yai_vault_abi.h
	@grep -q "YAI_VAULT_ABI_VERSION" law/specs/vault/yai_vault_abi.h
	@grep -q "YAI_VAULT_LAYOUT_BYTES" law/specs/vault/yai_vault_abi.h
	@grep -q "YAI_VAULT_HEADER_SIZE" law/specs/vault/yai_vault_abi.h
	@echo "  ABI OK"

verify-protocol:
	@echo "[VERIFY] Protocol IDs..."
	@test -f law/specs/protocol/yai_protocol_ids.h
	@grep -q "YAI_PROTOCOL_IDS_VERSION" law/specs/protocol/yai_protocol_ids.h
	@echo "  Protocol OK"

verify-binaries:
	@echo "[VERIFY] Binary presence..."
	@test -f $(BIN_DIR)/yai-boot
	@test -f $(BIN_DIR)/yai-kernel
	@test -f $(BIN_DIR)/yai-engine
	@test -f $(MIND_DIR)/target/release/yai
	@echo "  Binaries OK"

verify-hash:
	@echo "[VERIFY] Binary hashes..."
	@mkdir -p $(VERIFY_DIR)
	@shasum -a 256 $(BIN_DIR)/yai-boot   > $(VERIFY_DIR)/yai-boot.sha256
	@shasum -a 256 $(BIN_DIR)/yai-kernel > $(VERIFY_DIR)/yai-kernel.sha256
	@shasum -a 256 $(BIN_DIR)/yai-engine > $(VERIFY_DIR)/yai-engine.sha256
	@shasum -a 256 $(MIND_DIR)/target/release/yai > $(VERIFY_DIR)/yai-mind.sha256
	@echo "  Hashes written to $(VERIFY_DIR)"

verify-tests:
	@echo "[VERIFY] Rust tests..."
	cd $(MIND_DIR) && cargo test --quiet
	@echo "  Tests OK"


.PHONY: verify-strict update-hash

verify-strict: all verify-abi verify-protocol verify-binaries
	@echo "[VERIFY-STRICT] Comparing hashes..."
	@test -f $(VERIFY_DIR)/baseline.sha256 || (echo "No baseline found. Run make update-hash first."; exit 1)
	@shasum -a 256 $(BIN_DIR)/yai-boot \
	                $(BIN_DIR)/yai-kernel \
	                $(BIN_DIR)/yai-engine \
	                $(MIND_DIR)/target/release/yai > $(VERIFY_DIR)/current.sha256
	@diff $(VERIFY_DIR)/baseline.sha256 $(VERIFY_DIR)/current.sha256 || (echo "Binary drift detected."; exit 1)
	@echo "  Deterministic check OK"

update-hash: all
	@mkdir -p $(VERIFY_DIR)
	@shasum -a 256 $(BIN_DIR)/yai-boot \
	                $(BIN_DIR)/yai-kernel \
	                $(BIN_DIR)/yai-engine \
	                $(MIND_DIR)/target/release/yai > $(VERIFY_DIR)/baseline.sha256
	@echo "Baseline hash updated."
