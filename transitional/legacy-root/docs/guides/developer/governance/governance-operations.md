# Governance Operations

## Process Guardrails
- Keep scope explicit and limited per change.
- Do not bypass mandatory checks with skip flags for closure.
- Keep evidence links to runbooks/reports deterministic.

## GitHub Governance Inputs
- Issue and milestone templates are governed by `infra` standards.
- Local usage must stay aligned with program policies under `docs/program/policies/`.

## Verification in Repo
- Governance compatibility gate: `./tools/bin/yai-governance-compat-check`
- Convergence smoke: `./tools/release/unified_repo_convergence_smoke.sh`
