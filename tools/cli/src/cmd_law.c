#include "../include/yai_cmd_law.h"
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <errno.h>

#define LAW_PATH_MAX 512

static int file_exists(const char *p) {
    struct stat st;
    return (p && p[0] && stat(p, &st) == 0 && S_ISREG(st.st_mode));
}

static int dir_exists(const char *p) {
    struct stat st;
    return (p && p[0] && stat(p, &st) == 0 && S_ISDIR(st.st_mode));
}

/*
 * Resolve repository-relative prefix for accessing `law/...`
 */
static const char* find_law_prefix(void) {
    static const char *CANDIDATES[] = {
        "",
        "../",
        "../../",
        "../../../",
        NULL
    };

    for (int i = 0; CANDIDATES[i]; i++) {
        char path[LAW_PATH_MAX];
        int n = snprintf(path, sizeof(path), "%slaw", CANDIDATES[i]);
        if (n > 0 && (size_t)n < sizeof(path) && dir_exists(path)) {
            return CANDIDATES[i];
        }
    }

    return NULL;
}

typedef struct {
    const char *rel_path;
    int must_be_dir;
} law_req_t;

static const law_req_t REQ[] = {
    { "law/specs/control/control_plane.v1.json", 0 },
    { "law/specs/control/authority.json",        0 },
    { "law/specs/protocol/protocol.h",           0 },
    { "law/specs/protocol/transport.h",          0 },
    { "law/specs/protocol/yai_protocol_ids.h",   0 },
    { "law/formal/YAI_KERNEL.tla",               0 },
    { "law/formal/spec_map.md",                  0 },
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
        fprintf(stderr, "[law][FATAL] 'law/' not found. Run from repo root.\n");
        return 2;
    }

    int ok = 1;
    printf("[law] Integrity check (prefix: %s)\n",
           prefix[0] ? prefix : "./");

    for (int i = 0; REQ[i].rel_path; i++) {
        char full[LAW_PATH_MAX];
        int n = snprintf(full, sizeof(full), "%s%s",
                         prefix, REQ[i].rel_path);

        if (n <= 0 || (size_t)n >= sizeof(full)) {
            fprintf(stderr, "  [FAIL] %s (path overflow)\n",
                    REQ[i].rel_path);
            ok = 0;
            continue;
        }

        int exists = REQ[i].must_be_dir
            ? dir_exists(full)
            : file_exists(full);

        if (exists) {
            printf("  [OK]   %s\n", REQ[i].rel_path);
        } else {
            fprintf(stderr, "  [FAIL] %s (missing)\n",
                    REQ[i].rel_path);
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
    puts("├── axioms/");
    puts("├── boundaries/");
    puts("├── formal/");
    puts("└── specs/");
    puts("    ├── control/");
    puts("    ├── protocol/");
    puts("    └── ...");
    return 0;
}

static int cmd_status(void) {
    const char *prefix = find_law_prefix();
    if (prefix) {
        printf("[law] FOUND (prefix: %s)\n",
               prefix[0] ? prefix : "./");
        return 0;
    }

    fprintf(stderr, "[law] NOT_FOUND\n");
    return 2;
}

int yai_cmd_law(int argc, char **argv, const yai_cli_opts_t *opt) {
    (void)opt;

    if (argc < 1) {
        law_usage();
        return 1;
    }

    const char *sub = argv[0];

    if (strcmp(sub, "check") == 0)  return cmd_check();
    if (strcmp(sub, "tree") == 0)   return cmd_tree();
    if (strcmp(sub, "status") == 0) return cmd_status();

    fprintf(stderr, "ERR: unknown law subcommand: %s\n", sub);
    law_usage();
    return 1;
}
