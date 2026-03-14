#!/usr/bin/env bash
set -euo pipefail

# Resolve canonical governance root consumed by runtime/tooling wrappers.
yai_resolve_governance_root() {
  local root="${1:-}"

  if [[ -n "${YAI_GOVERNANCE_ROOT:-}" && -d "${YAI_GOVERNANCE_ROOT:-}" ]]; then
    echo "$YAI_GOVERNANCE_ROOT"
    return 0
  fi

  if [[ -n "$root" && -d "$root/governance" ]]; then
    echo "$root/governance"
    return 0
  fi

  return 1
}
