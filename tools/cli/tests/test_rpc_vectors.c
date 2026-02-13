// tools/cli/tests/test_rpc_vectors.c
#include "../include/yai_envelope.h"
#include <stdio.h>
#include <string.h>

int main(void) {
    char out[4096];
    int rc = yai_build_request_jsonl("dev","tr-1",0,"user","ping","null",out,sizeof(out));
    if (rc != 0) return 1;
    if (strstr(out, "\"type\":\"ping\"") == NULL) return 2;
    puts("ok");
    return 0;
}
