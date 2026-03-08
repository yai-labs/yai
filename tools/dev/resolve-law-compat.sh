#!/usr/bin/env bash
set -euo pipefail

# Resolve compatibility material root consumed by tooling.
# Authority remains law; callers should consume exports/snapshots via this resolver.

yai_resolve_law_compat_root() {
  local root="${1:-}"

  if [[ -n "${YAI_LAW_COMPAT_ROOT:-}" && -d "${YAI_LAW_COMPAT_ROOT:-}" ]]; then
    echo "$YAI_LAW_COMPAT_ROOT"
    return 0
  fi

  if [[ -n "${YAI_SPECS_ROOT:-}" && -d "${YAI_SPECS_ROOT:-}" ]]; then
    echo "$YAI_SPECS_ROOT"
    return 0
  fi

  if [[ -n "$root" && -d "$root/deps/law" ]]; then
    echo "$root/deps/law"
    return 0
  fi

  if [[ -n "$root" && -d "$root/deps/law" ]]; then
    echo "$root/deps/law"
    return 0
  fi

  return 1
}
