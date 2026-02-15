// tools/cli/src/cmd_root.c
#include "../include/yai_cmd.h"
#include "../include/yai_rpc.h"

#include <stdio.h>
#include <string.h>

static void usage_global(void) {
    fprintf(stderr,
        "YAI SOVEREIGN CLI\n"
        "USAGE:\n"
        "  yai kernel <status|ping|stop|ws>\n"
        "  yai engine --ws <id> <storage|network|resource|provider> <method> [params_json]\n"
        "  yai mind   --ws <id> <chat|think|query> <prompt>\n"
        "  yai law    <check|tree|status>\n"
        "\nTry:\n"
        "  yai kernel help\n"
        "  yai engine help\n"
        "  yai mind help\n"
        "  yai law help\n"
    );
}

static void usage_kernel(void) {
    fprintf(stderr,
        "Kernel Control (L1)\n"
        "Usage:\n"
        "  yai kernel <status|ping|stop>\n"
        "  yai kernel ws <create|list|destroy> [id]\n"
        "\nNotes:\n"
        "  - L1 commands are global (no --ws required)\n"
        "  - ws destroy requires --arming\n"
    );
}

static void usage_engine(void) {
    fprintf(stderr,
        "Engine Gates (L2)\n"
        "Usage:\n"
        "  yai engine --ws <id> <storage|network|resource|provider> <method> [params_json]\n"
        "\nExamples:\n"
        "  yai engine --ws dev storage put_node '{\"id\":\"n1\"}'\n"
        "  yai engine --ws dev provider infer '{\"model\":\"...\",\"prompt\":\"...\"}'\n"
        "\nNotes:\n"
        "  - --ws is mandatory (ADR-001)\n"
    );
}

static void usage_mind(void) {
    fprintf(stderr,
        "Mind Interface (L3)\n"
        "Usage:\n"
        "  yai mind --ws <id> <chat|think|query> <prompt>\n"
        "\nExamples:\n"
        "  yai mind --ws dev chat \"ciao\"\n"
        "  yai mind --ws dev think \"riassumi questo\"\n"
        "\nNotes:\n"
        "  - --ws is mandatory (ADR-001)\n"
    );
}

static void usage_law(void) {
    fprintf(stderr,
        "Law (local)\n"
        "Usage:\n"
        "  yai law <check|tree|status>\n"
        "\nExamples:\n"
        "  yai law check\n"
    );
}

static int is_help_cmd(int argc, char **argv) {
    return (argc >= 1 && argv && argv[0] && strcmp(argv[0], "help") == 0);
}

/**
 * Dispatcher Centrale (ADR-002)
 * Nota: cmd_root decide local-vs-rpc (law locale, resto via root socket).
 */
int yai_cmd_dispatch(const char *binary, int argc, char **argv, const yai_cli_opts_t *opt) {
    if (!binary || !binary[0]) { usage_global(); return 2; }

    // HELP per-bin (no RPC)
    if (is_help_cmd(argc, argv)) {
        if (strcmp(binary, "kernel") == 0) { usage_kernel(); return 0; }
        if (strcmp(binary, "engine") == 0) { usage_engine(); return 0; }
        if (strcmp(binary, "mind")   == 0) { usage_mind();   return 0; }
        if (strcmp(binary, "law")    == 0) { usage_law();    return 0; }
        usage_global();
        return 0;
    }

    // LAW: sempre locale
    if (strcmp(binary, "law") == 0) {
        if (argc < 1) { usage_law(); return 1; }
        return yai_cmd_law(argc, argv, opt);
    }

    // Validazione preventiva: engine/mind richiedono ws (ADR-001)
    if (strcmp(binary, "engine") == 0 || strcmp(binary, "mind") == 0) {
        if (!opt || !opt->ws_id || !opt->ws_id[0]) {
            fprintf(stderr, "FATAL: '%s' requires --ws <id>\n", binary);
            return 2;
        }
    }

    // Routing ai moduli
    if (strcmp(binary, "kernel") == 0) {
        if (argc > 0 && strcmp(argv[0], "ws") == 0) return yai_cmd_ws(argc - 1, argv + 1, opt);
        return yai_cmd_kernel(argc, argv, opt);
    }
    if (strcmp(binary, "engine") == 0) return yai_cmd_engine(argc, argv, opt);
    if (strcmp(binary, "mind")   == 0) return yai_cmd_mind(argc, argv, opt);

    fprintf(stderr, "ERR: Unknown binary target: %s\n", binary);
    usage_global();
    return 2;
}
