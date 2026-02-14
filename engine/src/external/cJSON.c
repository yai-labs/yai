#include "cJSON.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

cJSON *cJSON_Parse(const char *value) {
    // Nota: In un sistema reale useresti la lib completa. 
    // Qui l'Engine si aspetta JSON validi dal Mind.
    return NULL; // TODO: Inserire qui parser minimale o linkare lib completa
}

char *cJSON_PrintUnformatted(const cJSON *item) {
    if (!item) return NULL;
    char *out = malloc(256); // Placeholder
    snprintf(out, 256, "{\"status\":\"ok\"}"); 
    return out;
}

void cJSON_Delete(cJSON *c) {
    if (!c) return;
    if (c->child) cJSON_Delete(c->child);
    if (c->next) cJSON_Delete(c->next);
    free(c->string);
    free(c->valuestring);
    free(c);
}

cJSON *cJSON_CreateObject(void) {
    cJSON *n = (cJSON*)malloc(sizeof(cJSON));
    if (n) memset(n, 0, sizeof(cJSON));
    if (n) n->type = cJSON_Object;
    return n;
}

void cJSON_AddItemToObject(cJSON *object, const char *string, cJSON *item) {
    if (!item || !string) return;
    item->string = strdup(string);
    item->next = object->child;
    object->child = item;
}

cJSON *cJSON_AddStringToObject(cJSON *object, const char *name, const char *string) {
    cJSON *s = (cJSON*)malloc(sizeof(cJSON));
    memset(s, 0, sizeof(cJSON));
    s->type = cJSON_String;
    s->valuestring = strdup(string);
    cJSON_AddItemToObject(object, name, s);
    return s;
}

cJSON *cJSON_GetObjectItem(const cJSON * const object, const char * const string) {
    cJSON *c = object->child;
    while (c && strcmp(c->string, string)) c = c->next;
    return c;
}