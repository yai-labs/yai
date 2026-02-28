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

# L3 (Mind) — optional plane (not part of default dist/bundle)
MIND_DIR := mind
MIND_BIN := yai-mind

DOXYFILE := Doxyfile
DOXYGEN ?= doxygen
DOXY_OUT ?= $(DIST_ROOT)/docs/doxygen

CANONICAL_BINS := yai-boot yai-root-server yai-kernel yai-engine

.PHONY: all build build-all dist dist-all bundle verify preflight-release \
        boot root core kernel engine mind mind-check mind-dist \
        clean clean-dist clean-all docs docs-clean docs-verify proof-verify \
        release-guards release-guards-dev changelog-verify help

all: build
	@echo "[YAI] dist is now separated from build. Use 'make dist' or 'make bundle'."

# Default build: only canonical planes (L0-L2)
build: runtime-protocol boot root kernel engine
	@echo "--- [YAI] Build Complete ---"

# Convenience: build everything including Mind (L3)
build-all: build mind
	@echo "--- [YAI] Build-All Complete (including mind) ---"

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

# Convenience: dist everything including Mind (still not in bundle unless you want)
dist-all: dist mind-dist
	@echo "--- [YAI] Dist-All staged (including mind) ---"

bundle: dist
	@tools/bin/yai-bundle

boot:
	$(MAKE) -C $(BOOT_DIR) build BUILD_ROOT=$(BUILD_ROOT) BIN_BUILD=$(BIN_BUILD)

root:
	$(MAKE) -C $(ROOT_PLANE_DIR) build BUILD_ROOT=$(BUILD_ROOT) BIN_BUILD=$(BIN_BUILD)

core: root

kernel:
	$(MAKE) -C $(KERNEL_DIR) build BUILD_ROOT=$(BUILD_ROOT) BIN_BUILD=$(BIN_BUILD)

engine:
	$(MAKE) -C $(ENGINE_DIR) build BUILD_ROOT=$(BUILD_ROOT) BIN_BUILD=$(BIN_BUILD)

# -------------------------
# Mind (Rust) — optional
# -------------------------

mind:
	@command -v cargo >/dev/null 2>&1 || { echo "ERROR: cargo not found (install Rust toolchain)"; exit 1; }
	@mkdir -p $(BIN_BUILD)
	@if [ -f "$(ROOT_DIR)/Cargo.lock" ]; then \
		cargo build -p yai-mind --locked ; \
	else \
		echo "[YAI] Cargo.lock missing at repo root; generating lockfile (one-time)"; \
		cargo generate-lockfile --manifest-path $(MIND_DIR)/Cargo.toml || true; \
		cargo build -p yai-mind ; \
	fi
	@if [ -f "target/debug/$(MIND_BIN)" ]; then \
		cp "target/debug/$(MIND_BIN)" "$(BIN_BUILD)/$(MIND_BIN)"; \
		echo "[YAI] mind staged: $(BIN_BUILD)/$(MIND_BIN)"; \
	else \
		echo "ERROR: mind built but binary missing at ./target/debug/$(MIND_BIN)"; \
		exit 1; \
	fi

mind-check:
	@command -v cargo >/dev/null 2>&1 || { echo "ERROR: cargo not found (install Rust toolchain)"; exit 1; }
	@cargo fmt -p yai-mind --check
	@cargo clippy -p yai-mind -- -D warnings
	@cargo test -p yai-mind --locked

mind-dist: mind
	@mkdir -p $(BIN_DIST)
	@if [ -f "$(BIN_BUILD)/$(MIND_BIN)" ]; then \
		cp "$(BIN_BUILD)/$(MIND_BIN)" "$(BIN_DIST)/$(MIND_BIN)"; \
		echo "[YAI] mind dist staged: $(BIN_DIST)/$(MIND_BIN)"; \
	else \
		echo "ERROR: mind artifact missing at $(BIN_BUILD)/$(MIND_BIN) (run 'make mind')"; \
		exit 1; \
	fi

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
	@tools/bin/yai-check-pins

docs:
	@mkdir -p $(DOXY_OUT)
	@$(DOXYGEN) $(DOXYFILE)
	@echo "✔ Doxygen: $(DOXY_OUT)/html/index.html"

docs-clean:
	@rm -rf $(DOXY_OUT)

docs-verify:
	@tools/bin/yai-docs-trace-check --all

proof-verify:
	@tools/bin/yai-proof-check

release-guards:
	@tools/bin/yai-check-pins
	@tools/bin/yai-proof-check

release-guards-dev:
	@STRICT_SPECS_HEAD=0 tools/bin/yai-check-pins
	@tools/bin/yai-proof-check

changelog-verify:
	@BASE_SHA="$$(git rev-parse origin/main)"; \
	HEAD_SHA="$$(git rev-parse HEAD)"; \
	tools/bin/yai-changelog-check --pr --base "$$BASE_SHA" --head "$$HEAD_SHA"

help:
	@echo "Targets:"
	@echo "  build        (boot/root/kernel/engine)"
	@echo "  dist         (stage canonical bins)"
	@echo "  bundle       (bundle canonical dist)"
	@echo "  build-all    (build + mind)"
	@echo "  dist-all     (dist + mind-dist)"
	@echo "  mind         (build mind; stage in build/bin when possible)"
	@echo "  mind-check   (fmt/clippy/test for mind)"
	@echo "  mind-dist    (stage mind binary in dist/bin)"
	@echo "  verify, preflight-release, docs, docs-verify, proof-verify, release-guards, changelog-verify"
.PHONY: runtime-protocol
runtime-protocol:
	$(MAKE) -C runtime-protocol
