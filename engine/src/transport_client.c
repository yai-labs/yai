#include "transport.h"
#include <stdio.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>
#include <string.h>
#include <stdlib.h>

static int _sock_fd = -1;
const char* SOCK_PATH = "/tmp/yai_runtime.sock";

static const char* yai_socket_path(void) {
    const char* env_path = getenv("YAI_RUNTIME_SOCKET");
    if (env_path && env_path[0] != '\0') {
        return env_path;
    }
    return SOCK_PATH;
}

int yai_transport_init() {
    struct sockaddr_un addr;

    if ((_sock_fd = socket(AF_UNIX, SOCK_STREAM, 0)) == -1) {
        return -1;
    }

    memset(&addr, 0, sizeof(struct sockaddr_un));
    addr.sun_family = AF_UNIX;
    const char* sock_path = yai_socket_path();
    strncpy(addr.sun_path, sock_path, sizeof(addr.sun_path) - 1);

    if (connect(_sock_fd, (struct sockaddr *)&addr, sizeof(struct sockaddr_un)) == -1) {
        close(_sock_fd);
        _sock_fd = -1;
        return -2;
    }

    return 0;
}

int yai_send_command(IceCmdType type, const void* data, size_t len) {
    if (_sock_fd == -1) return -1;

    IcePacket pkt;
    pkt.type = type;
    pkt.payload_size = (len > 1024) ? 1024 : len;
    if (data && len > 0) {
        memcpy(pkt.payload, data, pkt.payload_size);
    }

    if (send(_sock_fd, &pkt, sizeof(IcePacket), 0) == -1) {
        return -3;
    }

    return 0;
}
