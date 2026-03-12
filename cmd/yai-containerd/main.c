#define _POSIX_C_SOURCE 200809L

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include <yai/api/version.h>
#include <yai/container/container.h>

static void print_help(void) {
  puts("yai-containerd - canonical container manager service surface");
  printf("version: %s\n", YAI_VERSION_STRING);
  puts("");
  puts("usage:");
  puts("  yai-containerd create <container-id> [interactive|managed|service|system|recovery]");
  puts("  yai-containerd initialize <container-id>");
  puts("  yai-containerd open <container-id>");
  puts("  yai-containerd attach <container-id> <session-id>");
  puts("  yai-containerd bind <container-id> <session-id> [normal|privileged|recovery|diagnostic] [cap-mask]");
  puts("  yai-containerd unbind <container-id> <session-id>");
  puts("  yai-containerd rebind <container-id> <old-session-id> <new-session-id> [mode] [cap-mask]");
  puts("  yai-containerd enter <container-id> <session-id>");
  puts("  yai-containerd leave <container-id> <session-id>");
  puts("  yai-containerd escape <container-id> <session-id> [none|admin|recovery|debug]");
  puts("  yai-containerd recovery-enter <container-id> <session-id>");
  puts("  yai-containerd mount <container-id> <source> <target> <ro|rw|privileged> [internal|attached|read-only|read-write|hidden|privileged-only]");
  puts("  yai-containerd resolve <container-id> <container-path>");
  puts("  yai-containerd visible <container-id> <container-path> [0|1]");
  puts("  yai-containerd recover <container-id>");
  puts("  yai-containerd seal <container-id>");
  puts("  yai-containerd destroy <container-id>");
  puts("  yai-containerd show <container-id>");
}

static yai_container_class_t parse_class(const char *value) {
  if (!value || !value[0] || strcmp(value, "interactive") == 0) return YAI_CONTAINER_CLASS_INTERACTIVE;
  if (strcmp(value, "managed") == 0) return YAI_CONTAINER_CLASS_MANAGED;
  if (strcmp(value, "service") == 0) return YAI_CONTAINER_CLASS_SERVICE;
  if (strcmp(value, "system") == 0) return YAI_CONTAINER_CLASS_SYSTEM;
  if (strcmp(value, "recovery") == 0) return YAI_CONTAINER_CLASS_RECOVERY;
  return YAI_CONTAINER_CLASS_INTERACTIVE;
}

static yai_container_session_mode_t parse_session_mode(const char *value) {
  if (!value || strcmp(value, "normal") == 0) return YAI_CONTAINER_SESSION_MODE_NORMAL;
  if (strcmp(value, "privileged") == 0) return YAI_CONTAINER_SESSION_MODE_PRIVILEGED;
  if (strcmp(value, "recovery") == 0) return YAI_CONTAINER_SESSION_MODE_RECOVERY;
  if (strcmp(value, "diagnostic") == 0) return YAI_CONTAINER_SESSION_MODE_DIAGNOSTIC;
  return YAI_CONTAINER_SESSION_MODE_NORMAL;
}

static const char *session_mode_name(yai_container_session_mode_t mode) {
  switch (mode) {
    case YAI_CONTAINER_SESSION_MODE_GLOBAL: return "global";
    case YAI_CONTAINER_SESSION_MODE_NORMAL: return "normal";
    case YAI_CONTAINER_SESSION_MODE_PRIVILEGED: return "privileged";
    case YAI_CONTAINER_SESSION_MODE_RECOVERY: return "recovery";
    case YAI_CONTAINER_SESSION_MODE_DIAGNOSTIC: return "diagnostic";
    default: return "unknown";
  }
}

static yai_container_escape_policy_class_t parse_escape_class(const char *value) {
  if (!value || strcmp(value, "none") == 0) return YAI_CONTAINER_ESCAPE_NONE;
  if (strcmp(value, "admin") == 0) return YAI_CONTAINER_ESCAPE_CONTROLLED_ADMIN;
  if (strcmp(value, "recovery") == 0) return YAI_CONTAINER_ESCAPE_RECOVERY;
  if (strcmp(value, "debug") == 0) return YAI_CONTAINER_ESCAPE_DEBUG;
  return YAI_CONTAINER_ESCAPE_NONE;
}

static yai_container_mount_policy_t parse_mount_policy(const char *value) {
  if (!value || strcmp(value, "ro") == 0) return YAI_CONTAINER_MOUNT_RO;
  if (strcmp(value, "rw") == 0) return YAI_CONTAINER_MOUNT_RW;
  return YAI_CONTAINER_MOUNT_PRIVILEGED;
}

static yai_container_visibility_class_t parse_visibility(const char *value) {
  if (!value || strcmp(value, "attached") == 0) return YAI_CONTAINER_VISIBILITY_ATTACHED;
  if (strcmp(value, "internal") == 0) return YAI_CONTAINER_VISIBILITY_INTERNAL;
  if (strcmp(value, "read-only") == 0) return YAI_CONTAINER_VISIBILITY_READ_ONLY;
  if (strcmp(value, "read-write") == 0) return YAI_CONTAINER_VISIBILITY_READ_WRITE;
  if (strcmp(value, "hidden") == 0) return YAI_CONTAINER_VISIBILITY_HIDDEN;
  if (strcmp(value, "privileged-only") == 0) return YAI_CONTAINER_VISIBILITY_PRIVILEGED_ONLY;
  return YAI_CONTAINER_VISIBILITY_ATTACHED;
}

static int cmd_create(const char *container_id, const char *class_name) {
  yai_container_record_t record;
  time_t now = time(NULL);

  if (!container_id || !container_id[0]) return 1;

  memset(&record, 0, sizeof(record));
  (void)snprintf(record.identity.container_id, sizeof(record.identity.container_id), "%s", container_id);
  record.identity.container_class = parse_class(class_name);
  (void)snprintf(record.identity.container_profile,
                 sizeof(record.identity.container_profile),
                 "%s",
                 yai_container_class_name(record.identity.container_class));
  (void)snprintf(record.identity.creation_source, sizeof(record.identity.creation_source), "%s", "yai-containerd");
  record.identity.owner_handle = 1;
  record.identity.state_handle = (uint64_t)now;

  yai_container_config_defaults(&record.config);
  record.lifecycle.current = YAI_CONTAINER_LIFECYCLE_CREATED;
  record.lifecycle.previous = YAI_CONTAINER_LIFECYCLE_CREATED;
  record.lifecycle.created_at = (int64_t)now;
  record.lifecycle.updated_at = (int64_t)now;
  (void)snprintf(record.root.root_path, sizeof(record.root.root_path), "/container/%s", record.identity.container_id);
  record.root.container_root_handle = record.identity.state_handle;
  record.session_domain.container_session_scope = record.identity.state_handle;
  yai_container_state_defaults(&record.state);

  if (yai_container_create(&record) != 0) {
    fprintf(stderr, "create failed for container '%s'\n", container_id);
    return 1;
  }

  printf("created container %s class=%s\n",
         record.identity.container_id,
         yai_container_class_name(record.identity.container_class));
  return 0;
}

static int cmd_show(const char *container_id) {
  yai_container_identity_t identity;
  yai_container_state_t state;
  yai_container_root_t root;
  yai_container_session_domain_t session;
  yai_container_policy_view_t policy;
  yai_container_grants_view_t grants;

  if (yai_container_get_identity(container_id, &identity) != 0 ||
      yai_container_get_state(container_id, &state) != 0 ||
      yai_container_get_root_view(container_id, &root) != 0 ||
      yai_container_get_session_view(container_id, &session) != 0 ||
      yai_container_get_policy_view(container_id, &policy) != 0 ||
      yai_container_get_grants_view(container_id, &grants) != 0) {
    fprintf(stderr, "show failed for container '%s'\n", container_id);
    return 1;
  }

  printf("container_id=%s\n", identity.container_id);
  printf("class=%s profile=%s source=%s\n",
         yai_container_class_name(identity.container_class),
         identity.container_profile,
         identity.creation_source);
  printf("lifecycle=%s\n", yai_container_lifecycle_name(state.lifecycle_state));
  printf("state-updated-at=%lld recovery-flags=%llu\n",
         (long long)state.updated_at,
         (unsigned long long)state.recovery_reason_flags);
  printf("root_handle=%llu projection=%s path=%s\n",
         (unsigned long long)root.container_root_handle,
         root.projection_ready ? "ready" : "not-ready",
         root.root_path);
  printf("projected-root=%s backing-store=%s\n",
         root.projected_root_host_path,
         root.backing_store_path);
  printf("session_bound=%u count=%llu last=%llu\n",
         (unsigned)session.bound,
         (unsigned long long)session.bound_session_count,
         (unsigned long long)session.last_bound_session_id);
  printf("active_session=%llu mode=%s cap-mask=%llu escape-class=%u iflags=%llu\n",
         (unsigned long long)session.active_session_id,
         session_mode_name(session.session_mode),
         (unsigned long long)session.capability_mask,
         (unsigned)session.escape_policy_class,
         (unsigned long long)session.interactive_flags);
  printf("policy_view=%llu grants_view=%llu\n",
         (unsigned long long)policy.policy_view_handle,
         (unsigned long long)grants.grants_view_handle);
  return 0;
}

int main(int argc, char **argv) {
  if (argc < 2 || strcmp(argv[1], "--help") == 0 || strcmp(argv[1], "help") == 0) {
    print_help();
    return 0;
  }

  if (strcmp(argv[1], "create") == 0) return cmd_create(argc > 2 ? argv[2] : NULL, argc > 3 ? argv[3] : NULL);
  if (strcmp(argv[1], "initialize") == 0 && argc > 2) return yai_container_initialize(argv[2]) == 0 ? 0 : 1;
  if (strcmp(argv[1], "open") == 0 && argc > 2) return yai_container_open(argv[2]) == 0 ? 0 : 1;
  if (strcmp(argv[1], "attach") == 0 && argc > 3) return yai_container_attach(argv[2], (uint64_t)strtoull(argv[3], NULL, 10)) == 0 ? 0 : 1;
  if (strcmp(argv[1], "bind") == 0 && argc > 3) {
    uint64_t sid = (uint64_t)strtoull(argv[3], NULL, 10);
    yai_container_session_mode_t mode = parse_session_mode(argc > 4 ? argv[4] : NULL);
    uint64_t cap = argc > 5 ? (uint64_t)strtoull(argv[5], NULL, 10) : UINT64_MAX;
    return yai_container_bind_session(argv[2], sid, mode, cap, 0) == 0 ? 0 : 1;
  }
  if (strcmp(argv[1], "unbind") == 0 && argc > 3) return yai_container_unbind_session(argv[2], (uint64_t)strtoull(argv[3], NULL, 10)) == 0 ? 0 : 1;
  if (strcmp(argv[1], "rebind") == 0 && argc > 4) {
    uint64_t old_sid = (uint64_t)strtoull(argv[3], NULL, 10);
    uint64_t new_sid = (uint64_t)strtoull(argv[4], NULL, 10);
    yai_container_session_mode_t mode = parse_session_mode(argc > 5 ? argv[5] : NULL);
    uint64_t cap = argc > 6 ? (uint64_t)strtoull(argv[6], NULL, 10) : UINT64_MAX;
    return yai_container_rebind_session(argv[2], old_sid, new_sid, mode, cap, 0) == 0 ? 0 : 1;
  }
  if (strcmp(argv[1], "enter") == 0 && argc > 3) {
    yai_container_bound_session_t s;
    uint64_t sid = (uint64_t)strtoull(argv[3], NULL, 10);
    if (yai_container_session_enter(argv[2], sid, &s) != 0) return 1;
    printf("session=%llu container=%s mode=%s root=%llu runtime-view=%llu\n",
           (unsigned long long)s.session_id,
           s.bound_container_id,
           session_mode_name(s.session_mode),
           (unsigned long long)s.root_handle,
           (unsigned long long)s.runtime_view_handle);
    printf("projected-root=%s\n", s.path_context.projected_root);
    return 0;
  }
  if (strcmp(argv[1], "leave") == 0 && argc > 3) return yai_container_session_leave(argv[2], (uint64_t)strtoull(argv[3], NULL, 10)) == 0 ? 0 : 1;
  if (strcmp(argv[1], "escape") == 0 && argc > 3) {
    uint64_t sid = (uint64_t)strtoull(argv[3], NULL, 10);
    return yai_container_session_request_escape(argv[2], sid, parse_escape_class(argc > 4 ? argv[4] : NULL)) == 0 ? 0 : 1;
  }
  if (strcmp(argv[1], "recovery-enter") == 0 && argc > 3) {
    return yai_container_session_enter_recovery(argv[2], (uint64_t)strtoull(argv[3], NULL, 10), 1u) == 0 ? 0 : 1;
  }
  if (strcmp(argv[1], "mount") == 0 && argc > 5) {
    yai_container_mount_t mount;
    memset(&mount, 0, sizeof(mount));
    (void)snprintf(mount.source, sizeof(mount.source), "%s", argv[3]);
    (void)snprintf(mount.target, sizeof(mount.target), "%s", argv[4]);
    mount.policy = parse_mount_policy(argv[5]);
    mount.mount_class = YAI_CONTAINER_MOUNT_ATTACHED_EXTERNAL;
    mount.visibility_class = parse_visibility(argc > 6 ? argv[6] : NULL);
    mount.attachability_class = YAI_CONTAINER_ATTACHABILITY_CONTROLLED;
    return yai_container_attach_mount(argv[2], &mount) == 0 ? 0 : 1;
  }
  if (strcmp(argv[1], "resolve") == 0 && argc > 3) {
    yai_container_path_context_t context;
    char resolved[2048];
    if (yai_container_path_context_load(argv[2], &context) != 0) return 1;
    if (yai_container_resolve_path(&context, argv[3], resolved, sizeof(resolved)) != 0) return 1;
    puts(resolved);
    return 0;
  }
  if (strcmp(argv[1], "visible") == 0 && argc > 3) {
    int privileged = (argc > 4 && strcmp(argv[4], "1") == 0) ? 1 : 0;
    int visible = yai_container_is_path_visible(argv[2], argv[3], privileged);
    if (visible < 0) return 1;
    puts(visible ? "visible" : "hidden");
    return visible ? 0 : 1;
  }
  if (strcmp(argv[1], "recover") == 0 && argc > 2) return yai_container_recover(argv[2], 1u) == 0 ? 0 : 1;
  if (strcmp(argv[1], "seal") == 0 && argc > 2) return yai_container_seal_runtime(argv[2], (int64_t)time(NULL)) == 0 ? 0 : 1;
  if (strcmp(argv[1], "destroy") == 0 && argc > 2) return yai_container_destroy(argv[2], (int64_t)time(NULL)) == 0 ? 0 : 1;
  if (strcmp(argv[1], "show") == 0 && argc > 2) return cmd_show(argv[2]);

  print_help();
  return 1;
}
