#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
OUT_DIR="$ROOT/build/test/unit_governance"
mkdir -p "$OUT_DIR"

LAW_SRCS=$(find "$ROOT/lib/governance" -type f -name '*.c' | sort)
CFLAGS='-Wall -Wextra -std=c11 -O2'
INCLUDES="-I$ROOT/include -I$ROOT/include/yai"

for t in \
  test_no_legacy_primary_path \
  test_manifest_loader \
  test_contract_surface \
  test_domain_loader \
  test_compliance_loader \
  test_discovery \
  test_family_specialization_routing \
  test_resolution \
  test_overlay_authority_evidence \
  test_precedence \
  test_effective_stack
 do
  cc $CFLAGS $INCLUDES "$ROOT/tests/unit/governance/${t}.c" $LAW_SRCS -o "$OUT_DIR/$t"
  "$OUT_DIR/$t"
 done

python3 "$ROOT/tests/unit/governance/test_embedded_surface_matches_publish_index.py"

echo "governance_unit_tests: ok"
