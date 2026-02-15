#include "../include/yai_cmd_law.h"
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <errno.h>

static int file_exists(const char *p) {
    struct stat st;
    return (p && p[0] && stat(p, &st) == 0 && S_ISREG(st.st_mode));
}

static int dir_exists(const char *p) {
    struct stat st;
    return (p && p[0] && stat(p, &st) == 0 && S_ISDIR(st.st_mode));
}

/**
 * Resolve repository-relative prefix for accessing `law/...`
 * Returns:
 *   ""      -> current working dir contains `law/`
 *   "../"   -> parent contains `law/`
 *   "../../" ...
 *   NULL    -> not found
 */
static const char* find_law_prefix(void) {
    if (dir_exists("law")) return "";
    if (dir_exists("../law")) return "../";
    if (dir_exists("../../law")) return "../../";
    if (dir_exists("../../../law")) return "../../../";
    return NULL;
}

typedef struct {
    const char *rel_path;
    int must_be_dir; // 0 = file, 1 = dir
} law_req_t;

static const law_req_t REQ[] = {
    { "law/specs/control/control_plane.v1.json", 0 },
    { "law/specs/control/authority.json",        0 },
    { "law/specs/protocol/protocol.h",          0 },
    { "law/specs/protocol/transport.h",         0 },
    { "law/specs/protocol/yai_protocol_ids.h",  0 },
    { "law/formal/YAI_KERNEL.tla",              0 },
    { "law/formal/spec_map.md",                 0 },
    { NULL, 0 }
};

static void law_usage(void) {
    fprintf(stderr,
        "YAI Law (local)\n"
        "Usage:\n"
        "  yai law check    # verify core specs exist\n"
        "  yai law tree     # print logical structure\n"
        "  yai law status   # exit 0 if law/ found, else 2\n"
    );
}

static int cmd_check(void) {
    const char *prefix = find_law_prefix();
    if (!prefix) {
        fprintf(stderr, "[law][FATAL] Could not locate 'law/' directory. Run from repo root or tools/cli.\n");
        return 2;
    }

    int ok = 1;
    printf("[law] Integrity check (prefix: %s)\n", prefix[0] ? prefix : "./");

    for (int i = 0; REQ[i].rel_path != NULL; i++) {
        char full[512];
        snprintf(full, sizeof(full), "%s%s", prefix, REQ[i].rel_path);

        int exists = REQ[i].must_be_dir ? dir_exists(full) : file_exists(full);
        if (exists) {
            printf("  [OK]   %s\n", REQ[i].rel_path);
        } else {
            fprintf(stderr, "  [FAIL] %s (missing)\n", REQ[i].rel_path);
            ok = 0;
        }
    }

    if (ok) {
        printf("\n[law] Status: PROVISIONS_MET\n");
        return 0;
    }

    fprintf(stderr, "\n[law] Status: VIOLATION_DETECTED\n");
    return 2;
}

static int cmd_tree(void) {
    puts("law/ (Sovereignty Root)");
    puts("├── axioms/        # Fundamental truths");
    puts("├── boundaries/    # L0-Lx isolation rules");
    puts("├── formal/        # TLA+ specifications");
    puts("└── specs/         # Machine-readable laws (json/h)");
    puts("    ├── control/   # Root plane authority");
    puts("    ├── protocol/  # Wire + IDs + auth/audit");
    puts("    └── ...");
    return 0;
}

static int cmd_status(void) {
    const char *prefix = find_law_prefix();
    if (prefix) {
        printf("[law] FOUND (prefix: %s)\n", prefix[0] ? prefix : "./");
        return 0;
    }
    fprintf(stderr, "[law] NOT_FOUND (run inside repo)\n");
    return 2;
}

int yai_cmd_law(int argc, char **argv, const yai_cli_opts_t *opt) {
    (void)opt;
    if (argc < 1) { law_usage(); return 1; }

    const char *sub = argv[0];

    if (strcmp(sub, "check") == 0)  return cmd_check();
    if (strcmp(sub, "tree") == 0)   return cmd_tree();
    if (strcmp(sub, "status") == 0) return cmd_status();

    fprintf(stderr, "ERR: unknown law subcommand: %s\n", sub);
    law_usage();
    return 1;
}
