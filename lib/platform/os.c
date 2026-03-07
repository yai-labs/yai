#include <unistd.h>

#include <yai/platform/os.h>

pid_t yai_os_getpid(void) {
  return getpid();
}
