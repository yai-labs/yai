#include <time.h>

#include <yai/platform/clock.h>

long yai_clock_unix_seconds(void) {
  return (long)time(NULL);
}
