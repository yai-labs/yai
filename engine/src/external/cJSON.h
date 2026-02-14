#ifndef cJSON__h
#define cJSON__h

#include <stddef.h>

#define cJSON_Invalid (0)
#define cJSON_False  (1 << 0)
#define cJSON_True   (1 << 1)
#define cJSON_NULL   (1 << 2)
#define cJSON_Number (1 << 3)
#define cJSON_String (1 << 4)
#define cJSON_Array  (1 << 5)
#define cJSON_Object (1 << 6)
#define cJSON_Raw    (1 << 7)

typedef struct cJSON {
    struct cJSON *next;
    struct cJSON *prev;
    struct cJSON *child;
    int type;
    char *valuestring;
    int valueint;
    double valuedouble;
    char *string;
} cJSON;

cJSON *cJSON_Parse(const char *value);
char *cJSON_PrintUnformatted(const cJSON *item);
void cJSON_Delete(cJSON *c);
cJSON *cJSON_CreateObject(void);
void cJSON_AddItemToObject(cJSON *object, const char *string, cJSON *item);
cJSON *cJSON_AddStringToObject(cJSON *object, const char *name, const char *string);
cJSON *cJSON_GetObjectItem(const cJSON * const object, const char * const string);

#endif