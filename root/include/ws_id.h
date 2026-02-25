#pragma once

#include <ctype.h>
#include <stddef.h>
#include <string.h>

#define YAI_WS_ID_MAX 35u

static inline int yai_ws_id_is_valid(const char *ws_id)
{
    if (!ws_id)
        return 0;

    size_t n = 0;
    while (n <= YAI_WS_ID_MAX && ws_id[n] != '\0')
        n++;
    if (n == 0 || n > YAI_WS_ID_MAX)
        return 0;

    for (size_t i = 0; i < n; i++) {
        unsigned char c = (unsigned char)ws_id[i];
        if (!(isalnum(c) || c == '-' || c == '_'))
            return 0;
    }

    return 1;
}
