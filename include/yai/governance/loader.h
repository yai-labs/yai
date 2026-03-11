#pragma once

#include <stddef.h>

#include <yai/governance/manifest.h>

typedef struct yai_law_runtime {
  char root[512];
  yai_law_manifest_t manifest;
  yai_law_runtime_entrypoint_t entrypoint;
  yai_law_runtime_view_t runtime_view;
  yai_law_compatibility_t compatibility;
} yai_law_runtime_t;

int yai_law_load_runtime(yai_law_runtime_t *out, char *err, size_t err_cap);
int yai_law_load_domain_manifest(const yai_law_runtime_t *rt,
                                 const char *domain_id,
                                 char *out_json,
                                 size_t out_cap);
int yai_law_load_compliance_index(const yai_law_runtime_t *rt,
                                  char *out_json,
                                  size_t out_cap);
int yai_law_read_surface_json(const yai_law_runtime_t *rt,
                              const char *rel_path,
                              char *out_json,
                              size_t out_cap);
