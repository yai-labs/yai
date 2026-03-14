#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
OUT_DIR="$ROOT/build/test/unit_mesh"
mkdir -p "$OUT_DIR"

cc -Wall -Wextra -std=c11 -O2 \
  -I"$ROOT/include" \
  "$ROOT/tests/legacy/unit/network/mesh/test_mesh_minimal.c" \
  "$ROOT/sys/network/topology/identity.c" \
  "$ROOT/sys/network/topology/peer_registry.c" \
  "$ROOT/sys/network/topology/membership.c" \
  "$ROOT/sys/network/discovery/discovery.c" \
  "$ROOT/sys/network/mesh/mesh_topology.c" \
  "$ROOT/sys/network/mesh/mesh_peering.c" \
  "$ROOT/sys/network/routing/coordination.c" \
  "$ROOT/sys/network/transport/transport_client.c" \
  "$ROOT/sys/network/routing/replay_state.c" \
  "$ROOT/sys/network/routing/conflict_state.c" \
  "$ROOT/sys/network/mesh/containment.c" \
  "$ROOT/sys/network/discovery/enrollment.c" \
  -o "$OUT_DIR/mesh_unit_tests"

"$OUT_DIR/mesh_unit_tests"
echo "mesh_unit_tests: ok"
