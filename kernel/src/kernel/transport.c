#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>
#include <stdio.h>
#include <string.h>
#include "transport.h"

#define SOCKET_PATH "/tmp/yai_runtime.sock"

static int server_fd = -1;

void handle_client_command(int client_fd) {
    IcePacket pkt;
    ssize_t bytes = read(client_fd, &pkt, sizeof(IcePacket));

    if (bytes > 0) {
        printf("[RUNTIME] Received Command: %d (Size: %u)\n", pkt.type, pkt.payload_size);

        switch (pkt.type) {
            case CMD_PING:
                printf("[RUNTIME] 🏓 PING received. Runtime is ALIVE.\n");
                break;
            case CMD_AGENT_SPAWN:
                printf("[RUNTIME] 🤖 Spawning agent: %s\n", pkt.payload);
                break;
            case CMD_SYS_HALT:
                printf("[RUNTIME] 🛑 System Halt requested.\n");
                break;
            default:
                printf("[RUNTIME] Unknown command type: %d\n", pkt.type);
        }
    }
}

int yai_transport_init(void) {
    struct sockaddr_un addr;

    // Crea la socket
    server_fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (server_fd < 0) return -1;
    
    memset(&addr, 0, sizeof(struct sockaddr_un));
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, SOCKET_PATH, sizeof(addr.sun_path) - 1);

    unlink(SOCKET_PATH); // Pulisce se esiste già
    if (bind(server_fd, (struct sockaddr *)&addr, sizeof(struct sockaddr_un)) < 0) return -2;
    if (listen(server_fd, 5) < 0) return -3;

    printf("[TRANSPORT] UDS Socket listening at %s\n", SOCKET_PATH);
    // Qui il main loop accetterà le connessioni dell'Engine
    return 0;
}

void yai_transport_serve_once(void) {
    if (server_fd < 0) return;

    int client_fd = accept(server_fd, NULL, NULL);
    if (client_fd < 0) return;

    handle_client_command(client_fd);
    close(client_fd);
}
