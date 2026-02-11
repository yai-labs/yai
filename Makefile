ROOT_DIR := $(abspath .)
ARTIFACT_ROOT := $(HOME)/.yai/artifacts/yai-core
BIN_DIR := $(ARTIFACT_ROOT)/bin
BUILD_DIR := $(ARTIFACT_ROOT)/build
DIST_DIR := $(ARTIFACT_ROOT)/dist

KERNEL_DIR := kernel
ENGINE_DIR := engine

GIT_SHA := $(shell git rev-parse --short HEAD 2>/dev/null || echo unknown)
BUILD_TIME := $(shell date -u +"%Y-%m-%dT%H:%M:%SZ")
PKG_TAG := $(shell date -u +"%Y%m%d-%H%M%S")

VAULT_ABI_VERSION := $(shell awk '/YAI_VAULT_ABI_VERSION/{print $$3}' law/specs/vault/yai_vault_abi.h 2>/dev/null)
VAULT_LAYOUT_BYTES := $(shell awk '/YAI_VAULT_LAYOUT_BYTES/{print $$3}' law/specs/vault/yai_vault_abi.h 2>/dev/null)
VAULT_HEADER_SIZE := $(shell awk '/YAI_VAULT_HEADER_SIZE/{print $$3}' law/specs/vault/yai_vault_abi.h 2>/dev/null)
PROTOCOL_IDS_VERSION := $(shell awk '/YAI_PROTOCOL_IDS_VERSION/{print $$3}' law/specs/protocol/yai_protocol_ids.h 2>/dev/null)

.PHONY: all kernel engine clean package

all: kernel engine

kernel:
	$(MAKE) -C $(KERNEL_DIR) OUT_BIN_DIR=$(BIN_DIR) OUT_BUILD_DIR=$(BUILD_DIR)/kernel all

engine:
	$(MAKE) -C $(ENGINE_DIR) OUT_BIN_DIR=$(BIN_DIR) OUT_BUILD_DIR=$(BUILD_DIR)/engine all

clean:
	$(MAKE) -C $(KERNEL_DIR) OUT_BIN_DIR=$(BIN_DIR) OUT_BUILD_DIR=$(BUILD_DIR)/kernel clean
	$(MAKE) -C $(ENGINE_DIR) OUT_BIN_DIR=$(BIN_DIR) OUT_BUILD_DIR=$(BUILD_DIR)/engine clean
	rm -rf $(DIST_DIR)

package: all
	@mkdir -p $(DIST_DIR)
	@printf '{\n' > $(DIST_DIR)/MANIFEST.json
	@printf '  "git_sha": "%s",\n' "$(GIT_SHA)" >> $(DIST_DIR)/MANIFEST.json
	@printf '  "build_time": "%s",\n' "$(BUILD_TIME)" >> $(DIST_DIR)/MANIFEST.json
	@printf '  "protocol_ids_version": %s,\n' "$(PROTOCOL_IDS_VERSION)" >> $(DIST_DIR)/MANIFEST.json
	@printf '  "vault_abi_version": %s,\n' "$(VAULT_ABI_VERSION)" >> $(DIST_DIR)/MANIFEST.json
	@printf '  "vault_layout_bytes": %s,\n' "$(VAULT_LAYOUT_BYTES)" >> $(DIST_DIR)/MANIFEST.json
	@printf '  "vault_header_size": %s\n' "$(VAULT_HEADER_SIZE)" >> $(DIST_DIR)/MANIFEST.json
	@printf '}\n' >> $(DIST_DIR)/MANIFEST.json
	@mkdir -p $(DIST_DIR)/pkg/bin
	@test -f $(BIN_DIR)/yai-boot
	@test -f $(BIN_DIR)/yai-kernel
	@test -f $(BIN_DIR)/yai-engine
	@cp $(BIN_DIR)/yai-boot $(DIST_DIR)/pkg/bin/yai-boot
	@cp $(BIN_DIR)/yai-kernel $(DIST_DIR)/pkg/bin/yai-kernel
	@cp $(BIN_DIR)/yai-engine $(DIST_DIR)/pkg/bin/yai-engine
	@cp $(DIST_DIR)/MANIFEST.json $(DIST_DIR)/pkg/MANIFEST.json
	@tar -czf $(DIST_DIR)/yai-core-$(PKG_TAG).tar.gz -C $(DIST_DIR)/pkg bin MANIFEST.json
	@rm -rf $(DIST_DIR)/pkg
