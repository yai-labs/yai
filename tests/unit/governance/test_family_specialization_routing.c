#include <stdio.h>
#include <string.h>

#include <yai/law/classifier.h>
#include <yai/law/discovery.h>

int main(void) {
  yai_law_classification_ctx_t ctx;
  yai_law_discovery_result_t res;

  if (yai_law_classify_event("ws-r1", "{\"command\":\"transfer.authorize\",\"resource\":\"ledger\",\"provider\":\"payment-gateway\"}", &ctx) != 0) return 1;
  if (yai_law_discover_domain(&ctx, &res) != 0) return 1;
  if (strcmp(res.family_id, "economic") != 0) return 1;
  if (strcmp(res.specialization_id, "transfers") != 0) return 1;
  if (strcmp(res.family_candidates[0], "economic") != 0) return 1;

  if (yai_law_classify_event("ws-r2", "{\"command\":\"github.issues.comment.create\",\"resource\":\"external_repository\",\"provider\":\"github-api\",\"contract\":\"true\"}", &ctx) != 0) return 1;
  if (yai_law_discover_domain(&ctx, &res) != 0) return 1;
  if (strcmp(res.family_id, "digital") != 0) return 1;
  if (strcmp(res.specialization_id, "external-commentary") != 0) return 1;

  if (yai_law_classify_event("ws-r3", "{\"command\":\"experiment.black-box-evaluation\",\"params_hash\":\"x\",\"dataset\":\"d\"}", &ctx) != 0) return 1;
  if (yai_law_discover_domain(&ctx, &res) != 0) return 1;
  if (strcmp(res.family_id, "scientific") != 0) return 1;
  if (strcmp(res.specialization_id, "black-box-evaluation") != 0) return 1;

  puts("family_specialization_routing: ok");
  return 0;
}
