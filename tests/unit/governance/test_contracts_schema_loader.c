#include <stdio.h>
#include <string.h>

#include <yai/governance/loader.h>

static int contains(const char *json, const char *needle) {
  return json && needle && strstr(json, needle) != NULL;
}

int main(void) {
  yai_governance_runtime_t rt;
  char err[256] = {0};
  char json[16384];

  if (yai_governance_load_runtime(&rt, err, sizeof(err)) != 0) {
    fprintf(stderr, "contracts_schema_loader: runtime load failed: %s\n", err);
    return 1;
  }

  if (yai_governance_read_surface_json(&rt, "contracts/control/schema/control_plane.v1.json", json, sizeof(json)) != 0) {
    fprintf(stderr, "contracts_schema_loader: missing control plane contract\n");
    return 1;
  }
  if (!contains(json, "\"protocol_name\"") || !contains(json, "\"knowledge\"")) {
    fprintf(stderr, "contracts_schema_loader: invalid control plane contract payload\n");
    return 1;
  }

  if (yai_governance_read_surface_json(&rt, "contracts/providers/schema/providers.v1.json", json, sizeof(json)) != 0) {
    fprintf(stderr, "contracts_schema_loader: missing providers contract\n");
    return 1;
  }
  if (!contains(json, "\"primary_plane\": \"knowledge\"")) {
    fprintf(stderr, "contracts_schema_loader: providers contract not canonicalized\n");
    return 1;
  }

  if (yai_governance_read_surface_json(&rt, "schema/workspace_governance_attachment.v1.schema.json", json, sizeof(json)) != 0) {
    fprintf(stderr, "contracts_schema_loader: missing workspace attachment schema\n");
    return 1;
  }
  if (!contains(json, "\"runtime_binding_model\"") || !contains(json, "\"topology_target\"")) {
    fprintf(stderr, "contracts_schema_loader: invalid workspace attachment schema\n");
    return 1;
  }

  if (yai_governance_read_surface_json(&rt, "schema/governance_review_state.v1.schema.json", json, sizeof(json)) != 0) {
    fprintf(stderr, "contracts_schema_loader: missing review state schema\n");
    return 1;
  }
  if (!contains(json, "\"next_allowed_actions\"") || !contains(json, "\"runtime_family_targets\"")) {
    fprintf(stderr, "contracts_schema_loader: invalid review state schema\n");
    return 1;
  }

  puts("contracts_schema_loader: ok");
  return 0;
}
