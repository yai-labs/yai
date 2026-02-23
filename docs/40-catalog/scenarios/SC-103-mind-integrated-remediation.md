---
id: SC-103
title: SC-103 - Mind-Integrated Containment and Remediation (Governed)
status: draft
owner: catalog
effective_date: 2026-02-23
revision: 1
depends_on:
  - SC-102
---

# SC-103 - Mind-Integrated Containment and Remediation (Governed)

## 1) Purpose
Demonstrate a full governed loop where an intelligent layer (Mind + LLM agents + cockpit) operates **inside a workspace** and can:
1) observe an incident signal,
2) propose a remediation plan,
3) generate a patch/config change,
4) apply changes only through governed execution,
5) produce complete evidence for audit and replay.

This scenario must preserve the runtime grammar:
**trigger -> context -> authority/contract -> decision -> enforcement -> evidence**

## 2) Scope
In-scope:
- Mind operates in a workspace and builds context (graphs, memory, reasoning).
- A code-capable agent proposes and prepares a remediation change (patch or configuration).
- Enforcement remains fail-closed at runtime boundaries (Root/Kernel/Engine).

Out-of-scope:
- Unrestricted autonomous actions outside policy.
- Real destructive operations against production assets.
- Any dependence on probabilistic LLM output for containment decisions.

## 3) Preconditions
- SC-102 is green for at least one pack (recommended first pack: `D1-digital/egress-v1`).
- Workspace lifecycle and path jail are active.
- Root handshake + authority envelope are required and enforced.
- Evidence pipeline emits structured outcomes and trace artifacts.

## 4) System Under Test
Components:
- Boot -> Root control plane -> Kernel -> Engine (gated effects)
- Mind inside workspace
- Cockpit client (equivalent to CLI as a Root client)
- Mock providers/adapters (safe-only)

## 5) Scenario Narrative (Safe Simulation)
A workspace contains a service ("workload") that begins attempting an outbound egress action that is not contracted.
The runtime detects the forbidden effect and blocks it immediately.
The Mind is notified through the event/evidence stream, builds context, and proposes remediation:
- identify the responsible component,
- propose a safe change (e.g., disable the egress path, tighten config, rotate a token in mock mode),
- generate a patch/config PR,
- request arming for application of the change,
- apply only after governed approval,
- verify resolution via repeated qualification run.

## 6) Domain Pack Binding (recommended)
Primary pack: `D1-digital/egress-v1`
Optional extensions:
- `D6-operational/incident-response-action-v1` (simulate containment action workflow)
- `D8-scientific/reproducibility-parameter-lock-v1` (parameter lock for remediation plan)

## 7) Steps (Operational)
### Step A - Trigger
- A safe stimulus is executed (vector) that causes an egress attempt to a non-allowlisted destination.

### Step B - Context
- Workspace context is established (`ws_id`, `trace_id`, principal role, arming state).
- Mind subscribes to workspace evidence stream.

### Step C - Authority/Contract
- Baseline and contracts are pinned and recorded (baseline hash).
- Mind reads only what policy permits (minimal disclosure).

### Step D - Decision
- The runtime decides `deny` or `quarantine` for the forbidden effect with stable reason code.
- The decision must not depend on Mind/LLM output.

### Step E - Enforcement
- Engine/Kernel blocks the forbidden effect (fail-closed).
- Optional: quarantine workflow is activated (simulation-only).

### Step F - Evidence
- Evidence record includes decision + enforcement + baseline hash + metrics.

### Step G - Remediation Proposal (Mind)
- Mind builds context graph:
  - event timeline
  - suspected component mapping
  - configuration/code path hypothesis
- Mind produces a remediation plan with:
  - proposed change type (config/patch)
  - expected impact and rollback plan
  - required authority/arming level

### Step H - Change Generation (Code Agent)
- Code agent generates a patch or configuration update in a sandboxed repo clone.
- Output is a commit/PR artifact (simulation or local git).

### Step I - Governed Apply
- Cockpit/CLI requests arming/approval.
- After approval, the runtime applies the change through governed channels.

### Step J - Verification
- Re-run the same vector (3 consecutive runs).
- Confirm:
  - forbidden effect remains prevented
  - evidence completeness is 100%
  - remediation artifacts are linked in evidence

## 8) Evidence Outputs
Required outputs:
- Evidence run bundle (ER): logs + structured evidence records
- Linkage between:
  - baseline hash
  - vector id
  - decision reason codes
  - remediation artifact id (commit/PR)
  - verification rerun results

## 9) KPIs
- time_to_contain_ms (must remain within SC-102 envelope)
- remediation_time_to_patch_s (measured, no strict threshold in v0.1)
- evidence_completeness = 100%
- repeatability_runs >= 3 (post-fix)

## 10) Acceptance Criteria
- Containment is fail-closed and independent of Mind output.
- Mind produces a remediation plan with traceable justification.
- Code change is generated and applied only via governed approval.
- Post-fix qualification reruns pass consistently and produce complete evidence.

## 11) Safety Controls
- Mock providers only (no real external effects).
- No secrets in evidence.
- Minimal disclosure enforced for all Mind inputs.
