#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include "yai_kernel.h"

void yai_rpc_write_error_v1(int fd, const char* ws_id, const char* trace, const char* code, const char* msg, const char* actor) {
    char buf[1024];
    snprintf(buf, sizeof(buf), 
        "{\"v\":1,\"type\":\"error\",\"ws_id\":\"%s\",\"trace\":\"%s\",\"error\":{\"code\":\"%s\",\"msg\":\"%s\",\"actor\":\"%s\"}}\n",
        ws_id ? ws_id : "null", trace ? trace : "null", code, msg, actor ? actor : "kernel");
    write(fd, buf, strlen(buf));
}