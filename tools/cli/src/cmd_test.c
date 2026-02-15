#include "../include/yai_cli.h"
#include "../include/yai_rpc.h"
#include <stdio.h>
#include <string.h>

/*
 * yai test
 * Placeholder intentionally empty.
 *
 * Future:
 *  - wire to runbooks under docs/runbooks/
 *  - execute protocol vectors from law/specs/vectors/
 */

static void usage(void) {
    fprintf(stderr,
        "YAI Test Harness (placeholder)\n"
        "Usage:\n"
        "  yai test\n"
        "  yai test --help\n"
    );
}

int yai_cmd_test(int argc, char **argv, const yai_cli_opts_t *opt) {
    (void)opt;

    if (argc > 0 && (strcmp(argv[0], "--help") == 0 || strcmp(argv[0], "-h") == 0)) {
        usage();
        return 0;
    }

    // Deliberately empty for now (no side effects).
    printf("{\"status\":\"ok\",\"code\":\"TEST_PLACEHOLDER\",\"detail\":\"no tests wired\"}\n");
    return 0;
}
