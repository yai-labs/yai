// tools/cli/src/cmd_law.c
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

static void law_usage(void) {
    fprintf(stderr,
        "USAGE:\n"
        "  yai law check\n"
        "  yai law tree\n"
        "  yai law paths\n"
        "\n"
        "NOTES:\n"
        "  - 'law' is local (no RPC).\n"
        "  - Used to verify law/specs are present + stable before multi-tenant.\n"
    );
}

// repo-relative paths (from repo root)
static const char *LAW_DIR = "law";
static const char *LAW_SPECS_DIR = "law/specs";
static const char *LAW_PROTOCOL_DIR = "law/specs/protocol";
static const char *LAW_CONTROL_DIR = "law/specs/control";
static const char *LAW_FORMAL_DIR = "law/formal";

static const char *REQ_FILES[] = {
    "law/specs/control/control_plane.v1.json",
    "law/specs/control/authority.json",
    "law/specs/protocol/protocol.h",
    "law/specs/protocol/transport.h",
    "law/specs/protocol/commands.h",
    "law/specs/protocol/yai_protocol_ids.h",
    "law/formal/YAI_KERNEL.tla",
    "law/formal/YAI_KERNEL.cfg",
    "law/formal/spec_map.md",
    NULL
};

static int cmd_paths(void) {
    printf("[law] root=%s\n", LAW_DIR);
    printf("[law] specs=%s\n", LAW_SPECS_DIR);
    printf("[law] protocol=%s\n", LAW_PROTOCOL_DIR);
    printf("[law] control=%s\n", LAW_CONTROL_DIR);
    printf("[law] formal=%s\n", LAW_FORMAL_DIR);
    return 0;
}

static int cmd_tree(void) {
    // Minimal “tree” without spawning external tools.
    // Prints a stable shortlist that matters for boundary+tests.
    puts("law/");
    puts("  axioms/ ...");
    puts("  invariants/ ...");
    puts("  boundaries/ ...");
    puts("  specs/");
    puts("    control/");
    puts("      control_plane.v1.json");
    puts("      authority.json");
    puts("    protocol/");
    puts("      protocol.h");
    puts("      transport.h");
    puts("      commands.h");
    puts("      yai_protocol_ids.h");
    puts("    vault/ ...");
    puts("    vectors/ ...");
    puts("  formal/");
    puts("    YAI_KERNEL.tla");
    puts("    YAI_KERNEL.cfg");
    puts("    spec_map.md");
    return 0;
}

static int cmd_check(void) {
    int ok = 1;

    if (!dir_exists(LAW_DIR)) {
        fprintf(stderr, "[law][FAIL] missing directory: %s\n", LAW_DIR);
        ok = 0;
    }
    if (!dir_exists(LAW_SPECS_DIR)) {
        fprintf(stderr, "[law][FAIL] missing directory: %s\n", LAW_SPECS_DIR);
        ok = 0;
    }
    if (!dir_exists(LAW_FORMAL_DIR)) {
        fprintf(stderr, "[law][WARN] missing directory: %s (TLA/TLC will fail)\n", LAW_FORMAL_DIR);
        ok = 0;
    }

    for (int i = 0; REQ_FILES[i] != NULL; i++) {
        const char *p = REQ_FILES[i];
        if (!file_exists(p)) {
            fprintf(stderr, "[law][FAIL] missing file: %s (errno=%d)\n", p, errno);
            ok = 0;
        }
    }

    if (!ok) {
        fprintf(stderr, "[law] status=NOT_OK\n");
        return 2;
    }

    fprintf(stdout, "[law] status=OK\n");
    return 0;
}

int yai_cmd_law(int argc, char **argv, const yai_cli_opts_t *opt) {
    (void)opt;

    if (argc < 1) {
        law_usage();
        return 2;
    }

    const char *sub = argv[0];

    if (!strcmp(sub, "help") || !strcmp(sub, "-h") || !strcmp(sub, "--help")) {
        law_usage();
        return 0;
    }
    if (!strcmp(sub, "paths")) return cmd_paths();
    if (!strcmp(sub, "tree"))  return cmd_tree();
    if (!strcmp(sub, "check")) return cmd_check();

    fprintf(stderr, "ERR: unknown law subcommand: %s\n", sub);
    law_usage();
    return 2;
}
