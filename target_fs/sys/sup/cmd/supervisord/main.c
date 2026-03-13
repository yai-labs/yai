/* SPDX-License-Identifier: Apache-2.0 */
#include <yai/sup/admission.h>
#include <yai/sup/recovery.h>
#include <yai/sup/registry.h>

#include <stdio.h>

int main(int argc, char **argv)
{
  const char *scope = (argc > 1) ? argv[1] : "user";
  char admission[128], recovery[128], registry[128];

  if (yai_supervisor_admission_check(scope, admission, sizeof(admission)) != 0) return 1;
  if (yai_supervisor_recovery_plan(scope, recovery, sizeof(recovery)) != 0) return 1;
  if (yai_supervisor_registry_snapshot(registry, sizeof(registry)) != 0) return 1;

  puts("yai-supervisord - supervisor L2 entrypoint");
  puts(admission);
  puts(recovery);
  puts(registry);
  return 0;
}
