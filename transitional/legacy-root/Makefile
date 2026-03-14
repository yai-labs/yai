# =========================================
# YAI — Unified Root Build Spine
# =========================================

ROOT_DIR := $(abspath .)

CC ?= cc
AR ?= ar
PKG_CONFIG ?= pkg-config
DOXYGEN ?= doxygen

BUILD_DIR ?= $(ROOT_DIR)/build
BIN_DIR ?= $(BUILD_DIR)/bin
OBJ_DIR ?= $(BUILD_DIR)/obj
LIB_DIR ?= $(BUILD_DIR)/lib
TEST_DIR ?= $(BUILD_DIR)/test

DIST_ROOT ?= $(ROOT_DIR)/dist
BIN_DIST ?= $(DIST_ROOT)/bin

PROTOCOL_CONTRACT_ROOT ?= $(ROOT_DIR)/target_fs/sys/ifc/proto

CPPFLAGS ?= -I$(ROOT_DIR) \
            -I$(ROOT_DIR)/system/include \
            -I$(ROOT_DIR)/kernel/include \
            -I$(ROOT_DIR)/user/include \
            -I$(ROOT_DIR)/sdk/c/libyai/include \
            -I$(ROOT_DIR)/sdk/c/libyai/third_party/cjson \
            -I$(ROOT_DIR)/vendor/cjson \
            -I$(PROTOCOL_CONTRACT_ROOT)

CFLAGS ?= -Wall -Wextra -std=c11 -O2
LDFLAGS ?=
LDLIBS ?= -lm

LMDB_CFLAGS := $(shell $(PKG_CONFIG) --cflags liblmdb 2>/dev/null)
LMDB_LIBS := $(shell $(PKG_CONFIG) --libs liblmdb 2>/dev/null)
HIREDIS_CFLAGS := $(shell $(PKG_CONFIG) --cflags hiredis 2>/dev/null)
HIREDIS_LIBS := $(shell $(PKG_CONFIG) --libs hiredis 2>/dev/null)
DUCKDB_CFLAGS := $(if $(wildcard /opt/homebrew/opt/duckdb/include/duckdb.h),-I/opt/homebrew/opt/duckdb/include,)
DUCKDB_LIBS := $(if $(wildcard /opt/homebrew/opt/duckdb/lib/libduckdb.dylib),-L/opt/homebrew/opt/duckdb/lib -lduckdb,)

ifneq ($(strip $(LMDB_LIBS)),)
CPPFLAGS += $(LMDB_CFLAGS) -DYAI_HAVE_LMDB=1
LDLIBS += $(LMDB_LIBS)
endif

ifneq ($(strip $(HIREDIS_LIBS)),)
CPPFLAGS += $(HIREDIS_CFLAGS) -DYAI_HAVE_HIREDIS=1
LDLIBS += $(HIREDIS_LIBS)
endif

ifneq ($(strip $(DUCKDB_LIBS)),)
CPPFLAGS += $(DUCKDB_CFLAGS) -DYAI_HAVE_DUCKDB=1
LDLIBS += $(DUCKDB_LIBS)
endif

define find_c
$(sort $(shell find $(1) -type f -name '*.c' ! -name '*.inc.c' 2>/dev/null))
endef

# -----------------------------------------
# Entry points
# -----------------------------------------

YAI_CLI_MAIN := user/bin/yai/main.c
YAI_CTL_MAIN := user/bin/yai-ctl/main.c
YAI_SH_MAIN := user/bin/yai-sh/main.c

YAI_DAEMOND_MAIN := system/daemon/cmd/yai-daemond/main.c
YAI_CONTAINERD_MAIN := system/container/yai-containerd/main.c
YAI_DATAD_MAIN := system/data/yai-datad/main.c
YAI_GRAPHD_MAIN := system/graph/yai-graphd/main.c
YAI_NETD_MAIN := system/network/yai-netd/main.c
YAI_ORCHESTRATORD_MAIN := system/orchestration/yai-orchestratord/main.c
YAI_POLICYD_MAIN := system/policy/yai-policyd/main.c
YAI_GOVERNANCED_MAIN := system/governance/yai-governanced/main.c
YAI_METRICSD_MAIN := system/observability/yai-metricsd/main.c
YAI_AUDITD_MAIN := system/observability/yai-auditd/main.c
YAI_SUPERVISORD_MAIN := system/supervisor/yai-supervisord/main.c

YAI_CLI_MAIN_OBJ := $(OBJ_DIR)/$(YAI_CLI_MAIN:.c=.o)
YAI_CTL_MAIN_OBJ := $(OBJ_DIR)/$(YAI_CTL_MAIN:.c=.o)
YAI_SH_MAIN_OBJ := $(OBJ_DIR)/$(YAI_SH_MAIN:.c=.o)

YAI_DAEMOND_MAIN_OBJ := $(OBJ_DIR)/$(YAI_DAEMOND_MAIN:.c=.o)
YAI_CONTAINERD_MAIN_OBJ := $(OBJ_DIR)/$(YAI_CONTAINERD_MAIN:.c=.o)
YAI_DATAD_MAIN_OBJ := $(OBJ_DIR)/$(YAI_DATAD_MAIN:.c=.o)
YAI_GRAPHD_MAIN_OBJ := $(OBJ_DIR)/$(YAI_GRAPHD_MAIN:.c=.o)
YAI_NETD_MAIN_OBJ := $(OBJ_DIR)/$(YAI_NETD_MAIN:.c=.o)
YAI_ORCHESTRATORD_MAIN_OBJ := $(OBJ_DIR)/$(YAI_ORCHESTRATORD_MAIN:.c=.o)
YAI_POLICYD_MAIN_OBJ := $(OBJ_DIR)/$(YAI_POLICYD_MAIN:.c=.o)
YAI_GOVERNANCED_MAIN_OBJ := $(OBJ_DIR)/$(YAI_GOVERNANCED_MAIN:.c=.o)
YAI_METRICSD_MAIN_OBJ := $(OBJ_DIR)/$(YAI_METRICSD_MAIN:.c=.o)
YAI_AUDITD_MAIN_OBJ := $(OBJ_DIR)/$(YAI_AUDITD_MAIN:.c=.o)
YAI_SUPERVISORD_MAIN_OBJ := $(OBJ_DIR)/$(YAI_SUPERVISORD_MAIN:.c=.o)

YAI_BIN := $(BIN_DIR)/yai
YAI_CTL_BIN := $(BIN_DIR)/yai-ctl
YAI_SH_BIN := $(BIN_DIR)/yai-sh
YAI_DAEMOND_BIN := $(BIN_DIR)/yai-daemond
YAI_DAEMON_BIN := $(BIN_DIR)/yai-daemon
YAI_CONTAINERD_BIN := $(BIN_DIR)/yai-containerd
YAI_DATAD_BIN := $(BIN_DIR)/yai-datad
YAI_GRAPHD_BIN := $(BIN_DIR)/yai-graphd
YAI_NETD_BIN := $(BIN_DIR)/yai-netd
YAI_ORCHESTRATORD_BIN := $(BIN_DIR)/yai-orchestratord
YAI_POLICYD_BIN := $(BIN_DIR)/yai-policyd
YAI_GOVERNANCED_BIN := $(BIN_DIR)/yai-governanced
YAI_METRICSD_BIN := $(BIN_DIR)/yai-metricsd
YAI_AUDITD_BIN := $(BIN_DIR)/yai-auditd
YAI_SUPERVISORD_BIN := $(BIN_DIR)/yai-supervisord
YAI_EDGE_ALIAS_BIN := $(BIN_DIR)/yai-edge

# -----------------------------------------
# Source discovery
# -----------------------------------------

LIBCORE_SRCS := $(call find_c,kernel/lib/core)
HAL_SRCS := $(call find_c,kernel/hal/core)
PROTOCOL_SRCS := $(call find_c,kernel/ipc)
CJSON_SRC := vendor/cjson/cJSON.c

KERNEL_SRCS := \
	$(call find_c,kernel/core) \
	$(call find_c,kernel/lifecycle) \
	$(call find_c,kernel/state) \
	$(call find_c,kernel/registry) \
	$(call find_c,kernel/policy) \
	$(call find_c,kernel/grants) \
	$(call find_c,kernel/security) \
	$(call find_c,kernel/session) \
	$(call find_c,kernel/container) \
	$(call find_c,kernel/net) \
	$(call find_c,kernel/proc) \
	$(call find_c,kernel/trace) \
	$(call find_c,kernel/mm) \
	$(call find_c,kernel/fs) \
	$(call find_c,kernel/drivers) \
	$(call find_c,kernel/arch) \
	$(call find_c,kernel/scheduler)

SYSTEM_CORE_SRCS := \
	$(call find_c,system/services) \
	$(call find_c,system/supervisor) \
	$(call find_c,system/interfaces)

CONTAINER_SRCS := \
	$(call find_c,kernel/container) \
	kernel/session/bindings.c \
	kernel/session/session_binding.c
DAEMON_SRCS := $(call find_c,system/daemon)
DATA_SRCS := $(call find_c,system/data)
GRAPH_SRCS := $(call find_c,system/graph)
NETWORK_SRCS := $(call find_c,system/network)
ORCHESTRATION_SRCS := $(call find_c,system/orchestration)
POLICY_SRCS := $(call find_c,system/policy)
GOVERNANCE_SRCS := $(call find_c,system/governance)
OBSERVABILITY_SRCS := $(call find_c,system/observability)

USER_CLI_SRCS := $(call find_c,user/shell)
USER_LIBYAI_SRCS := $(call find_c,sdk/c/libyai)

BUILD_EXCLUDE_SRCS := \
	$(YAI_CLI_MAIN) \
	$(YAI_CTL_MAIN) \
	$(YAI_SH_MAIN) \
	$(YAI_DAEMOND_MAIN) \
	$(YAI_CONTAINERD_MAIN) \
	$(YAI_DATAD_MAIN) \
	$(YAI_GRAPHD_MAIN) \
	$(YAI_NETD_MAIN) \
	$(YAI_ORCHESTRATORD_MAIN) \
	$(YAI_POLICYD_MAIN) \
	$(YAI_GOVERNANCED_MAIN) \
	$(YAI_METRICSD_MAIN) \
	$(YAI_AUDITD_MAIN) \
	$(YAI_SUPERVISORD_MAIN)

# -----------------------------------------
# Objects
# -----------------------------------------

SUPPORT_OBJS := $(patsubst %.c,$(OBJ_DIR)/%.o,$(filter-out $(BUILD_EXCLUDE_SRCS),$(LIBCORE_SRCS)))
HAL_OBJS := $(patsubst %.c,$(OBJ_DIR)/%.o,$(filter-out $(BUILD_EXCLUDE_SRCS),$(HAL_SRCS)))
PROTOCOL_OBJS := $(patsubst %.c,$(OBJ_DIR)/%.o,$(filter-out $(BUILD_EXCLUDE_SRCS),$(PROTOCOL_SRCS)))
CJSON_OBJ := $(OBJ_DIR)/$(CJSON_SRC:.c=.o)
KERNEL_OBJS := $(patsubst %.c,$(OBJ_DIR)/%.o,$(filter-out $(BUILD_EXCLUDE_SRCS),$(KERNEL_SRCS)))
SYSTEM_CORE_OBJS := $(patsubst %.c,$(OBJ_DIR)/%.o,$(filter-out $(BUILD_EXCLUDE_SRCS),$(SYSTEM_CORE_SRCS)))
CONTAINER_OBJS := $(patsubst %.c,$(OBJ_DIR)/%.o,$(filter-out $(BUILD_EXCLUDE_SRCS),$(CONTAINER_SRCS)))
DAEMON_OBJS := $(patsubst %.c,$(OBJ_DIR)/%.o,$(filter-out $(BUILD_EXCLUDE_SRCS),$(DAEMON_SRCS)))
DATA_OBJS := $(patsubst %.c,$(OBJ_DIR)/%.o,$(filter-out $(BUILD_EXCLUDE_SRCS),$(DATA_SRCS)))
GRAPH_OBJS := $(patsubst %.c,$(OBJ_DIR)/%.o,$(filter-out $(BUILD_EXCLUDE_SRCS),$(GRAPH_SRCS)))
NETWORK_OBJS := $(patsubst %.c,$(OBJ_DIR)/%.o,$(filter-out $(BUILD_EXCLUDE_SRCS),$(NETWORK_SRCS)))
ORCHESTRATION_OBJS := $(patsubst %.c,$(OBJ_DIR)/%.o,$(filter-out $(BUILD_EXCLUDE_SRCS),$(ORCHESTRATION_SRCS)))
POLICY_OBJS := $(patsubst %.c,$(OBJ_DIR)/%.o,$(filter-out $(BUILD_EXCLUDE_SRCS),$(POLICY_SRCS)))
GOVERNANCE_OBJS := $(patsubst %.c,$(OBJ_DIR)/%.o,$(filter-out $(BUILD_EXCLUDE_SRCS),$(GOVERNANCE_SRCS)))
OBSERVABILITY_OBJS := $(patsubst %.c,$(OBJ_DIR)/%.o,$(filter-out $(BUILD_EXCLUDE_SRCS),$(OBSERVABILITY_SRCS)))
USER_CLI_OBJS := $(patsubst %.c,$(OBJ_DIR)/%.o,$(filter-out $(BUILD_EXCLUDE_SRCS),$(USER_CLI_SRCS)))
USER_LIBYAI_OBJS := $(patsubst %.c,$(OBJ_DIR)/%.o,$(filter-out $(BUILD_EXCLUDE_SRCS),$(USER_LIBYAI_SRCS)))

# -----------------------------------------
# Libraries
# -----------------------------------------

SUPPORT_LIB := $(LIB_DIR)/libyai_support.a
HAL_LIB := $(LIB_DIR)/libyai_hal.a
PROTOCOL_LIB := $(LIB_DIR)/libyai_protocol.a
CJSON_LIB := $(LIB_DIR)/libcjson.a
KERNEL_LIB := $(LIB_DIR)/libyai_kernel.a
SYSTEM_CORE_LIB := $(LIB_DIR)/libyai_system_core.a
CONTAINER_LIB := $(LIB_DIR)/libyai_container.a
DAEMON_LIB := $(LIB_DIR)/libyai_daemon.a
DATA_LIB := $(LIB_DIR)/libyai_data.a
GRAPH_LIB := $(LIB_DIR)/libyai_graph.a
NETWORK_LIB := $(LIB_DIR)/libyai_network.a
ORCHESTRATION_LIB := $(LIB_DIR)/libyai_orchestration.a
POLICY_LIB := $(LIB_DIR)/libyai_policy.a
GOVERNANCE_LIB := $(LIB_DIR)/libyai_governance.a
OBSERVABILITY_LIB := $(LIB_DIR)/libyai_observability.a
USER_LIBYAI := $(LIB_DIR)/libyai_user_sdk.a

SPINE_DIRS := $(BIN_DIR) $(OBJ_DIR) $(LIB_DIR) $(TEST_DIR)

DOXYFILE := docs/transitional/root-meta/Doxyfile
DOXY_OUT ?= $(DIST_ROOT)/docs/doxygen

# -----------------------------------------
# Primary targets
# -----------------------------------------

.PHONY: all \
	yai yai-ctl yai-sh yai-daemond yai-daemon yai-containerd yai-datad yai-graphd yai-netd \
	yai-orchestratord yai-policyd yai-governanced yai-metricsd yai-auditd yai-supervisord yai-edge \
	foundations support platform hal protocol kernel-core system-core \
	container daemon data graph network orchestration policy governance observability \
	build build-all dist dist-all clean clean-dist clean-all dirs help \
	kernel-check kernel-smoke docs docs-clean

all: yai yai-ctl yai-sh yai-daemond yai-daemon yai-containerd yai-datad yai-graphd yai-netd yai-orchestratord yai-policyd yai-governanced yai-metricsd yai-auditd yai-supervisord foundations
	@echo "[YAI] unified binary spine ready: $(YAI_BIN) + $(YAI_CTL_BIN) + $(YAI_SH_BIN) + $(YAI_DAEMOND_BIN) + $(YAI_DAEMON_BIN) + $(YAI_CONTAINERD_BIN) + $(YAI_DATAD_BIN) + $(YAI_GRAPHD_BIN) + $(YAI_NETD_BIN) + $(YAI_ORCHESTRATORD_BIN) + $(YAI_POLICYD_BIN) + $(YAI_GOVERNANCED_BIN) + $(YAI_METRICSD_BIN) + $(YAI_AUDITD_BIN) + $(YAI_SUPERVISORD_BIN)"

yai: $(YAI_BIN)
yai-ctl: $(YAI_CTL_BIN)
yai-sh: $(YAI_SH_BIN)
yai-daemond: $(YAI_DAEMOND_BIN)
yai-daemon: yai-daemond
	@cp "$(YAI_DAEMOND_BIN)" "$(YAI_DAEMON_BIN)"
yai-containerd: $(YAI_CONTAINERD_BIN)
yai-datad: $(YAI_DATAD_BIN)
yai-graphd: $(YAI_GRAPHD_BIN)
yai-netd: $(YAI_NETD_BIN)
yai-orchestratord: $(YAI_ORCHESTRATORD_BIN)
yai-policyd: $(YAI_POLICYD_BIN)
yai-governanced: $(YAI_GOVERNANCED_BIN)
yai-metricsd: $(YAI_METRICSD_BIN)
yai-auditd: $(YAI_AUDITD_BIN)
yai-supervisord: $(YAI_SUPERVISORD_BIN)
yai-edge: yai-daemon
	@cp "$(YAI_DAEMON_BIN)" "$(YAI_EDGE_ALIAS_BIN)"

foundations: support hal protocol cjson kernel-core
support: $(SUPPORT_LIB)
platform: hal
hal: $(HAL_LIB)
protocol: $(PROTOCOL_LIB)
cjson: $(CJSON_LIB)
kernel-core: $(KERNEL_LIB)
system-core: $(SYSTEM_CORE_LIB)

container: $(CONTAINER_LIB)
daemon: $(DAEMON_LIB)
data: $(DATA_LIB)
graph: $(GRAPH_LIB)
network: $(NETWORK_LIB)
orchestration: $(ORCHESTRATION_LIB)
policy: $(POLICY_LIB)
governance: $(GOVERNANCE_LIB)
observability: $(OBSERVABILITY_LIB)

kernel-check:
	@$(MAKE) -C kernel check

kernel-smoke:
	@$(MAKE) -C kernel smoke

# -----------------------------------------
# Link rules
# -----------------------------------------

$(YAI_BIN): $(YAI_CLI_MAIN_OBJ) $(USER_CLI_OBJS) $(USER_LIBYAI) $(SYSTEM_CORE_LIB) $(CONTAINER_LIB) $(DAEMON_LIB) $(DATA_LIB) $(GRAPH_LIB) $(NETWORK_LIB) $(ORCHESTRATION_LIB) $(POLICY_LIB) $(GOVERNANCE_LIB) $(OBSERVABILITY_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) | dirs
	$(CC) $(LDFLAGS) $(YAI_CLI_MAIN_OBJ) $(USER_CLI_OBJS) -o $@ $(USER_LIBYAI) $(SYSTEM_CORE_LIB) $(CONTAINER_LIB) $(DAEMON_LIB) $(DATA_LIB) $(GRAPH_LIB) $(NETWORK_LIB) $(ORCHESTRATION_LIB) $(POLICY_LIB) $(GOVERNANCE_LIB) $(OBSERVABILITY_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) $(LDLIBS)

$(YAI_CTL_BIN): $(YAI_CTL_MAIN_OBJ) | dirs
	$(CC) $(LDFLAGS) $(YAI_CTL_MAIN_OBJ) -o $@ $(LDLIBS)

$(YAI_SH_BIN): $(YAI_SH_MAIN_OBJ) $(USER_CLI_OBJS) $(USER_LIBYAI) $(SYSTEM_CORE_LIB) $(CONTAINER_LIB) $(DAEMON_LIB) $(DATA_LIB) $(GRAPH_LIB) $(NETWORK_LIB) $(ORCHESTRATION_LIB) $(POLICY_LIB) $(GOVERNANCE_LIB) $(OBSERVABILITY_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) | dirs
	$(CC) $(LDFLAGS) $(YAI_SH_MAIN_OBJ) $(USER_CLI_OBJS) -o $@ $(USER_LIBYAI) $(SYSTEM_CORE_LIB) $(CONTAINER_LIB) $(DAEMON_LIB) $(DATA_LIB) $(GRAPH_LIB) $(NETWORK_LIB) $(ORCHESTRATION_LIB) $(POLICY_LIB) $(GOVERNANCE_LIB) $(OBSERVABILITY_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) $(LDLIBS)

$(YAI_DAEMOND_BIN): $(YAI_DAEMOND_MAIN_OBJ) $(DAEMON_LIB) $(SYSTEM_CORE_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) | dirs
	$(CC) $(LDFLAGS) $(YAI_DAEMOND_MAIN_OBJ) -o $@ $(DAEMON_LIB) $(SYSTEM_CORE_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) $(LDLIBS)

$(YAI_CONTAINERD_BIN): $(YAI_CONTAINERD_MAIN_OBJ) $(CONTAINER_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) | dirs
	$(CC) $(LDFLAGS) $(YAI_CONTAINERD_MAIN_OBJ) -o $@ $(CONTAINER_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) $(LDLIBS)

$(YAI_DATAD_BIN): $(YAI_DATAD_MAIN_OBJ) $(DATA_LIB) $(SYSTEM_CORE_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) | dirs
	$(CC) $(LDFLAGS) $(YAI_DATAD_MAIN_OBJ) -o $@ $(DATA_LIB) $(SYSTEM_CORE_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) $(LDLIBS)

$(YAI_GRAPHD_BIN): $(YAI_GRAPHD_MAIN_OBJ) $(GRAPH_LIB) $(DATA_LIB) $(ORCHESTRATION_LIB) $(SYSTEM_CORE_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) | dirs
	$(CC) $(LDFLAGS) $(YAI_GRAPHD_MAIN_OBJ) -o $@ $(GRAPH_LIB) $(DATA_LIB) $(ORCHESTRATION_LIB) $(SYSTEM_CORE_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) $(LDLIBS)

$(YAI_NETD_BIN): $(YAI_NETD_MAIN_OBJ) $(NETWORK_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) | dirs
	$(CC) $(LDFLAGS) $(YAI_NETD_MAIN_OBJ) -o $@ $(NETWORK_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) $(LDLIBS)

$(YAI_ORCHESTRATORD_BIN): $(YAI_ORCHESTRATORD_MAIN_OBJ) $(ORCHESTRATION_LIB) $(DATA_LIB) $(GRAPH_LIB) $(NETWORK_LIB) $(POLICY_LIB) $(GOVERNANCE_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) | dirs
	$(CC) $(LDFLAGS) $(YAI_ORCHESTRATORD_MAIN_OBJ) -o $@ $(ORCHESTRATION_LIB) $(DATA_LIB) $(GRAPH_LIB) $(NETWORK_LIB) $(POLICY_LIB) $(GOVERNANCE_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) $(LDLIBS)

$(YAI_POLICYD_BIN): $(YAI_POLICYD_MAIN_OBJ) $(POLICY_LIB) $(GOVERNANCE_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) | dirs
	$(CC) $(LDFLAGS) $(YAI_POLICYD_MAIN_OBJ) -o $@ $(POLICY_LIB) $(GOVERNANCE_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) $(LDLIBS)

$(YAI_GOVERNANCED_BIN): $(YAI_GOVERNANCED_MAIN_OBJ) $(GOVERNANCE_LIB) $(POLICY_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) | dirs
	$(CC) $(LDFLAGS) $(YAI_GOVERNANCED_MAIN_OBJ) -o $@ $(GOVERNANCE_LIB) $(POLICY_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) $(LDLIBS)

$(YAI_METRICSD_BIN): $(YAI_METRICSD_MAIN_OBJ) $(OBSERVABILITY_LIB) $(SYSTEM_CORE_LIB) $(KERNEL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) | dirs
	$(CC) $(LDFLAGS) $(YAI_METRICSD_MAIN_OBJ) -o $@ $(OBSERVABILITY_LIB) $(SYSTEM_CORE_LIB) $(KERNEL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) $(LDLIBS)

$(YAI_AUDITD_BIN): $(YAI_AUDITD_MAIN_OBJ) $(OBSERVABILITY_LIB) $(SYSTEM_CORE_LIB) $(KERNEL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) | dirs
	$(CC) $(LDFLAGS) $(YAI_AUDITD_MAIN_OBJ) -o $@ $(OBSERVABILITY_LIB) $(SYSTEM_CORE_LIB) $(KERNEL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) $(LDLIBS)

$(YAI_SUPERVISORD_BIN): $(YAI_SUPERVISORD_MAIN_OBJ) $(SYSTEM_CORE_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) | dirs
	$(CC) $(LDFLAGS) $(YAI_SUPERVISORD_MAIN_OBJ) -o $@ $(SYSTEM_CORE_LIB) $(KERNEL_LIB) $(PROTOCOL_LIB) $(SUPPORT_LIB) $(HAL_LIB) $(CJSON_LIB) $(LDLIBS)

# -----------------------------------------
# Archive rules
# -----------------------------------------

$(SUPPORT_LIB): $(SUPPORT_OBJS) | dirs
	$(AR) rcs $@ $^

$(HAL_LIB): $(HAL_OBJS) | dirs
	$(AR) rcs $@ $^

$(PROTOCOL_LIB): $(PROTOCOL_OBJS) | dirs
	$(AR) rcs $@ $^

$(CJSON_LIB): $(CJSON_OBJ) | dirs
	$(AR) rcs $@ $^

$(KERNEL_LIB): $(KERNEL_OBJS) | dirs
	$(AR) rcs $@ $^

$(SYSTEM_CORE_LIB): $(SYSTEM_CORE_OBJS) | dirs
	$(AR) rcs $@ $^

$(CONTAINER_LIB): $(CONTAINER_OBJS) | dirs
	$(AR) rcs $@ $^

$(DAEMON_LIB): $(DAEMON_OBJS) | dirs
	$(AR) rcs $@ $^

$(DATA_LIB): $(DATA_OBJS) | dirs
	$(AR) rcs $@ $^

$(GRAPH_LIB): $(GRAPH_OBJS) | dirs
	$(AR) rcs $@ $^

$(NETWORK_LIB): $(NETWORK_OBJS) | dirs
	$(AR) rcs $@ $^

$(ORCHESTRATION_LIB): $(ORCHESTRATION_OBJS) | dirs
	$(AR) rcs $@ $^

$(POLICY_LIB): $(POLICY_OBJS) | dirs
	$(AR) rcs $@ $^

$(GOVERNANCE_LIB): $(GOVERNANCE_OBJS) | dirs
	$(AR) rcs $@ $^

$(OBSERVABILITY_LIB): $(OBSERVABILITY_OBJS) | dirs
	$(AR) rcs $@ $^

$(USER_LIBYAI): $(USER_LIBYAI_OBJS) | dirs
	$(AR) rcs $@ $^

# -----------------------------------------
# Generic compile
# -----------------------------------------

$(OBJ_DIR)/%.o: %.c | dirs
	@mkdir -p $(dir $@)
	$(CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@

dirs:
	@mkdir -p $(SPINE_DIRS)

# -----------------------------------------
# Build / dist / docs
# -----------------------------------------

build: all
	@echo "[YAI] build complete"

build-all: build
	@echo "[YAI] build-all complete"

dist: build
	@mkdir -p $(BIN_DIST)
	@cp "$(YAI_BIN)" "$(BIN_DIST)/yai"
	@if [ -f "$(YAI_CTL_BIN)" ]; then cp "$(YAI_CTL_BIN)" "$(BIN_DIST)/yai-ctl"; fi
	@if [ -f "$(YAI_SH_BIN)" ]; then cp "$(YAI_SH_BIN)" "$(BIN_DIST)/yai-sh"; fi
	@if [ -f "$(YAI_DAEMOND_BIN)" ]; then cp "$(YAI_DAEMOND_BIN)" "$(BIN_DIST)/yai-daemond"; fi
	@if [ -f "$(YAI_DAEMON_BIN)" ]; then cp "$(YAI_DAEMON_BIN)" "$(BIN_DIST)/yai-daemon"; fi
	@if [ -f "$(YAI_CONTAINERD_BIN)" ]; then cp "$(YAI_CONTAINERD_BIN)" "$(BIN_DIST)/yai-containerd"; fi
	@if [ -f "$(YAI_DATAD_BIN)" ]; then cp "$(YAI_DATAD_BIN)" "$(BIN_DIST)/yai-datad"; fi
	@if [ -f "$(YAI_GRAPHD_BIN)" ]; then cp "$(YAI_GRAPHD_BIN)" "$(BIN_DIST)/yai-graphd"; fi
	@if [ -f "$(YAI_NETD_BIN)" ]; then cp "$(YAI_NETD_BIN)" "$(BIN_DIST)/yai-netd"; fi
	@if [ -f "$(YAI_ORCHESTRATORD_BIN)" ]; then cp "$(YAI_ORCHESTRATORD_BIN)" "$(BIN_DIST)/yai-orchestratord"; fi
	@if [ -f "$(YAI_POLICYD_BIN)" ]; then cp "$(YAI_POLICYD_BIN)" "$(BIN_DIST)/yai-policyd"; fi
	@if [ -f "$(YAI_GOVERNANCED_BIN)" ]; then cp "$(YAI_GOVERNANCED_BIN)" "$(BIN_DIST)/yai-governanced"; fi
	@if [ -f "$(YAI_METRICSD_BIN)" ]; then cp "$(YAI_METRICSD_BIN)" "$(BIN_DIST)/yai-metricsd"; fi
	@if [ -f "$(YAI_AUDITD_BIN)" ]; then cp "$(YAI_AUDITD_BIN)" "$(BIN_DIST)/yai-auditd"; fi
	@if [ -f "$(YAI_SUPERVISORD_BIN)" ]; then cp "$(YAI_SUPERVISORD_BIN)" "$(BIN_DIST)/yai-supervisord"; fi
	@echo "[YAI] dist staged in $(BIN_DIST)"

dist-all: dist
	@echo "[YAI] dist-all complete"

docs:
	@mkdir -p $(DOXY_OUT)
	@$(DOXYGEN) $(DOXYFILE)
	@echo "✔ Doxygen: $(DOXY_OUT)/html/index.html"

docs-clean:
	@rm -rf $(DOXY_OUT)

clean:
	rm -rf $(BUILD_DIR)

clean-dist:
	rm -rf $(DIST_ROOT)

clean-all: clean clean-dist

help:
	@echo "Primary targets:"
	@echo "  all               build all current binaries"
	@echo "  yai               canonical user entrypoint"
	@echo "  yai-ctl           control entrypoint"
	@echo "  yai-sh            interactive shell entrypoint"
	@echo "  yai-daemond       daemon runtime manager"
	@echo "  yai-daemon        compat alias of yai-daemond"
	@echo "  yai-containerd    container manager"
	@echo "  yai-datad         data service"
	@echo "  yai-graphd        graph service"
	@echo "  yai-netd          network service"
	@echo "  yai-orchestratord orchestration service"
	@echo "  yai-policyd       policy service"
	@echo "  yai-governanced   governance service"
	@echo "  yai-metricsd      metrics service"
	@echo "  yai-auditd        audit service"
	@echo "  yai-supervisord   supervisor service"
	@echo "  foundations       support/hal/protocol/kernel"
	@echo "  kernel-check      sub-build syntax check"
	@echo "  kernel-smoke      sub-build smoke"
	@echo "  build             full build"
	@echo "  dist              stage binaries into dist/bin"
	@echo "  clean             remove build/"