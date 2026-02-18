# YAI Cross-Repo Release Train Workflow (Core ↔ CLI)

**Status:** Canonical  
**Audience:** Maintainers, collaborators, code agents  
**Scope:** `yai` (core runtime) + `yai-cli` (client/tooling)  
**Baseline contract repo:** `yai-specs` (anchoring reference)

---

## 1. Purpose

This document defines the **canonical cross-repo workflow** for delivering features and hardening phases that span:
- **`yai`** (Root/Kernel/Engine/Runtime enforcement and behavior)
- **`yai-cli`** (conformant client, operator UX, and integration harness)

The goal is to ensure:
1. **Single Definition of Done** across repos (no “core done, CLI later” drift).
2. **Main is always releasable** in both repos.
3. Every milestone is **provable** via reproducible evidence (auditable).
4. Contract changes remain anchored to **`yai-specs`**.

---

## 2. Principles (Non-Negotiables)

### P1 — Main Always Releasable
- `main` in each repo MUST remain in a releasable state.
- Temporary breakages are allowed only inside feature branches / PRs, never in `main`.

### P2 — Vertical Slices by Runbook Phase
Work MUST be planned and delivered as **runbook phases** (vertical slices), not as “repo weeks”.
A phase is complete only when core behavior + CLI proof both exist.

### P3 — Single Definition of Done (DoD)
A milestone is “DONE” only if:
- core enforces the invariant deterministically, AND
- CLI can prove both positive and negative cases reproducibly, AND
- evidence is captured in an auditable form.

### P4 — Specs Are the Contract Anchor
If a milestone changes the contract (wire/envelope/roles/errors), the change MUST be reflected in `yai-specs` first or in lockstep.
This document does not redefine the contract; it defines delivery mechanics.

---

## 3. Repositories and Responsibilities

### 3.1 `yai` (Core Runtime)
Owns:
- Enforcement (authority gates, session rules, routing determinism)
- Logging/auditability (reject reasons, trace correlation)
- Deterministic error responses (stable codes + semantics)

### 3.2 `yai-cli` (Client + Harness)
Owns:
- Client conformance to contract (envelope correctness, handshake semantics)
- Operator UX (clear errors, safe defaults)
- Proof harness (smoke + negative tests, gates, repeatable checks)

### 3.3 `yai-specs` (Contract Baseline)
Owns:
- Protocol schema, roles, errors, invariants, vectors
- Compatibility statements and versioning rules

---

## 4. Unit of Work: “Milestone Pack”

Every delivery unit is a **Milestone Pack** (MP).  
A Milestone Pack represents ONE runbook phase (or a small set of tightly coupled phases).

### 4.1 Milestone Pack MUST include
1. **Identifier** (e.g., `MP-ROOT-HARDENING-0.1.2`)
2. **Objective** (what invariant becomes true)
3. **Contract Delta** (what changes on the wire / semantics)
4. **Repo Split** (what changes belong to `yai` vs `yai-cli`)
5. **Evidence Plan** (how we prove it: positive + negative cases)
6. **Compatibility Rule** (A/B classification, see below)
7. **Definition of Done** (explicit checks)

### 4.2 Milestone Pack template
```text
Milestone Pack: <ID>
Runbook link: <docs/runbooks/...>
Owner: <team/role>

Objective:
- <single sentence>

Contract Delta:
- Envelope: <none | fields changed>
- Authority: <none | new rule>
- Errors: <codes/semantics changes>
- Logging: <new required fields>

Repo Split:
- yai: <enforcement / routing / logging>
- yai-cli: <conformance / UX / harness>

Evidence Plan (minimum):
- Positive cases:
  - <case 1>
  - <case 2>
- Negative cases:
  - <case 1>
  - <case 2>

Compatibility Classification:
- Type: A | B
- Rationale: <why>
- Upgrade path: <how old clients behave>

Definition of Done:
- [ ] Core invariant enforced deterministically
- [ ] CLI proves invariant (pos/neg)
- [ ] Evidence captured and reviewable
- [ ] Compatibility updated (if needed)
```

## 5. Compatibility Classification (A/B)

Every Milestone Pack MUST be classified:

### Type A — Core-Only / Non-Breaking
- Core adds enforcement/behavior that does NOT require immediate CLI change.
- CLI may be improved, but core merge must not break existing CLI behavior.
- Merge implication: `yai` PR can merge independently.

### Type B — Contract-Touching / Sync Required
- Core enforcement WILL reject old client behavior OR client must adopt new fields/flags.
- CLI change is REQUIRED to validate and operate the milestone.
- Merge implication: PRs MUST be paired (see §6) and releases MUST be coordinated (see §7).

## 6. PR Model: Paired PRs (“Twin PRs”)

For Type B milestones, work MUST be represented as Twin PRs:
- PR-Core in `yai`
- PR-CLI in `yai-cli`

### 6.1 Twin PR requirements
- Each PR MUST link the other PR as a dependency.
- PR descriptions MUST reference the Milestone Pack ID and Evidence Plan.
- Review MUST validate the DoD as a single unit (not repo-by-repo “looks fine”).

### 6.2 PR description minimum fields (both PRs)
- Milestone Pack ID
- Classification (A/B)
- What changed (core or CLI perspective)
- Evidence (what was run / what proves correctness)
- Compatibility note (what breaks / what remains compatible)

## 7. Merge + Release Sequencing (The “Release Train”)

### 7.1 Key rule: Tag/Release happens AFTER cross-repo evidence
A final tag/release MUST be created only after:
- both PRs are merged (for Type B), AND
- evidence is complete and reviewed.

### 7.2 Sequencing for Type A
- Merge `yai` PR once core checks + evidence are satisfied.
- Optional: follow-up CLI PR can land later (but DoD remains satisfied without it).

### 7.3 Sequencing for Type B (Coordinated Merge)
Type B milestones MUST avoid long “broken compatibility windows”.

Preferred approach:
1. Ensure core can support a short compatibility buffer if feasible (best).
2. Merge `yai` PR and `yai-cli` PR as close together as possible.
3. Create the final release tags only when:
- both merged, AND
- evidence is captured.

### 7.4 Release Candidate vs Final Release
- A “candidate” reference MAY exist internally for coordination.
- A “final” release is the only externally meaningful state, and MUST satisfy DoD.

## 8. Evidence and Auditability

Evidence is not “someone said it works”.
Evidence MUST be:
- Reproducible (another person/agent can obtain the same outcome)
- Deterministic (same inputs -> same result; failures are stable)
- Reviewable (captured in PR text and/or repository artifacts)

### 8.1 Minimum evidence for every milestone
- At least 2 positive cases and 2 negative cases.
- Negative cases MUST demonstrate deterministic rejects with correct error codes/semantics.
- Logs MUST include enough context to be audit-grade (trace correlation, reject reason).

### 8.2 Where evidence lives
- Primary: PR description + CI results (as authoritative record)
- Secondary: existing test plans/runbooks referenced by path under `docs/`
- Optional: vectors in specs when contract-level test vectors change

## 9. Runbook Integration

Runbooks define operational intent; Milestone Packs define delivery units.

Rule:
- Each runbook phase MUST map to one Milestone Pack.

Practical mapping:
- Runbook: `docs/runbooks/<topic>.md`
- Milestone Packs: referenced from the runbook phases as IDs (`MP-*`).
- Test Plans: referenced from the Milestone Pack Evidence Plan when relevant.

## 10. Versioning + Pins (Cross-Repo Coherence)

When `yai` requires a specific `yai-cli` level (Type B milestones), the repo SHOULD maintain an explicit pin/reference to the compatible CLI version/revision (exact mechanism is repository-specific; the point is that it must be explicit and reviewable).

Compatibility expectations MUST remain consistent with:
- `COMPATIBILITY.md`
- `VERSIONING.md`
- any dependency/pin metadata tracked in the repository

## 11. Roles and Governance (Lightweight)

### 11.1 Owner
Each Milestone Pack MUST have a single Owner accountable for:
- coordination across repos
- ensuring DoD is satisfied end-to-end

### 11.2 Review policy
- Core enforcement changes: require reviewers who understand boundaries/invariants.
- CLI changes: require reviewers who validate UX + harness correctness.
- Type B: reviewers MUST treat Twin PRs as one unit.

### 11.3 Code agents
Agents MUST:
- start from the Milestone Pack template
- produce Twin PRs for Type B work
- include evidence and compatibility notes as first-class outputs

## 12. Anti-Patterns (What we do NOT do)

- “Finish core now, fix CLI later” for Type B milestones.
- Tagging core releases without a conformant CLI that proves the milestone.
- Changing contract semantics without anchoring to `yai-specs`.
- Relying on ad-hoc manual testing without deterministic evidence.
- Long-lived compatibility limbo where old CLI cannot operate but new CLI is not released.

## Appendix A — Worked Example (Authority Gate)

Milestone Pack: `MP-ROOT-HARDENING-0.1.2`  
Objective: privileged commands require `arming=true` + `role=operator`

Contract Delta:
- Authority: new reject rule for privileged commands
- Errors: deterministic error codes for missing arming / insufficient role
- Logging: reject reason MUST be audit-visible

Repo Split:
- `yai`: enforce gate at boundary + log reject reason
- `yai-cli`: expose operator intent safely + prove negative cases

Classification:
- Type B (sync required)

DoD:
- Core rejects non-conforming calls deterministically
- CLI can reproduce both allowed and rejected scenarios
- Evidence is captured in PR and CI

---
