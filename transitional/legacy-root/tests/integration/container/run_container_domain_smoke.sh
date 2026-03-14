#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
"$ROOT/tests/sys/container/containerd_smoke.sh"
echo "container_domain_smoke: ok"
