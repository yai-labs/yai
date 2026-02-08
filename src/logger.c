#include <stdio.h>
#include <time.h>
#include "kernel.h"

void ice_log_static(ice_event_type_t type, const char *msg) {
    time_t now = time(NULL);
    // Formato ultra-veloce ispirato a stdout.py
    printf("[%ld] [EV:%d] %s\n", now, type, msg);
}
