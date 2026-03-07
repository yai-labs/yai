/* SPDX-License-Identifier: Apache-2.0 */

#define _POSIX_C_SOURCE 200809L

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

int main(void)
{
  const char *socket_path = "/tmp/yai-mind-test.sock";
  pid_t pid = fork();

  assert(pid >= 0);
  if (pid == 0) {
    execl("./dist/bin/yai-mind", "./dist/bin/yai-mind", "--serve-once", "--socket", socket_path, (char *)NULL);
    _exit(127);
  }

  {
    struct timespec ts = {.tv_sec = 0, .tv_nsec = 120000000L};
    nanosleep(&ts, NULL);
  }

  {
    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    struct sockaddr_un addr;
    char out[1024] = {0};
    ssize_t n;

    assert(fd >= 0);
    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;
    snprintf(addr.sun_path, sizeof(addr.sun_path), "%s", socket_path);
    assert(connect(fd, (struct sockaddr *)&addr, sizeof(addr)) == 0);

    assert(write(fd, "COMPLETE daemon smoke\n", 22) == 22);
    n = read(fd, out, sizeof(out) - 1);
    assert(n > 0);
    out[n] = '\0';
    assert(strstr(out, "STATUS 200") != NULL);
    close(fd);
  }

  {
    int status = 0;
    waitpid(pid, &status, 0);
    assert(WIFEXITED(status));
    assert(WEXITSTATUS(status) == 0);
  }

  puts("test_mind_daemon_smoke: ok");
  return 0;
}
