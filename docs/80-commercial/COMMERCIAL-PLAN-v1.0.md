# YAI Commercial Plan v1.0

## 1. Executive Summary

YAI is positioned as a governance and execution control plane for production AI systems. The commercial objective is to become the default enterprise layer that allows organizations to deploy autonomous or semi-autonomous AI workflows with measurable safety, enforceable policy, and audit-grade evidence.

The primary market wedge is a high-friction operational problem that most organizations cannot solve quickly:

- AI workflows are technically feasible but operationally blocked by security, compliance, and change-risk concerns.
- Existing controls are fragmented across network controls, ad hoc scripts, manual reviews, and post-facto logging.
- Time-to-production is long, reliability is inconsistent, and auditability is weak.

YAI addresses this by enforcing a deterministic runtime grammar:

- contract -> decision -> enforcement -> evidence

The initial commercial release focuses on SC102 Wave 1 capabilities, using D1 (egress containment) and D8 (reproducibility parameter lock) as proof that governance controls can be applied rapidly, repeatedly, and across domains.

## 2. Commercial Thesis

### 2.1 Core Thesis

Organizations will pay for a platform that converts AI deployment risk into controlled, measurable, and auditable operations without slowing product delivery.

### 2.2 Why Now

- AI agents are moving from experimentation to production operations.
- Boards, CISOs, and regulators are increasing scrutiny on AI behavior and controls.
- Most teams lack a coherent control-plane architecture for AI runtime enforcement.

### 2.3 Why YAI

YAI provides an integrated architecture rather than point controls:

- Runtime planes: Boot, Root, Kernel, Engine
- Policy substrate: yai-law domain packs and baselines
- Qualification substrate: QT/RT wave runner, evidence bundling, verification
- Operator substrate: CLI status/doctor, runbooks, demos, reports

This creates a credible enterprise story: not just "AI works," but "AI operates under enforceable governance."

## 3. Star Case (Trojan Horse Use Case)

### 3.1 Star Case Name

AI Production Change Guard

### 3.2 Problem Statement

Enterprises struggle to safely approve AI-driven operational changes (data movement, outbound access, parameterized jobs, external calls) because controls are slow, fragmented, and difficult to audit.

### 3.3 YAI Promise

Deploy a governance-enforced runtime path in days, not quarters, with fail-closed behavior and replayable evidence.

### 3.4 Why This Wins Commercially

- Immediate value for security/compliance stakeholders
- Immediate velocity gain for engineering/operations
- Measurable reduction in incident exposure and review overhead
- Cross-domain extensibility beyond initial domains

## 4. Product Definition

### 4.1 Product Scope (Current-to-Next)

Current foundation (presentable now):

- D1 containment qualification and evidence
- D8 parameter-lock qualification and evidence
- SC102 wave run -> bundle -> verify workflow
- Operator demo scaffolding

Near-term productization (next 1-2 releases):

- Stronger Engine attachment visibility and diagnostics in CLI
- Hardened wave automation for reproducible customer replay
- Extended domain portfolio beyond D1/D8
- Enhanced reporting outputs for executive and audit audiences

### 4.2 Product Packaging (Commercial SKUs)

#### Starter (Design Partner)

- Scope: SC102 baseline governance package (D1 + D8)
- Outcome: deploy first governed workflows in controlled environments
- Delivery model: assisted onboarding and weekly governance reviews

#### Growth

- Scope: Starter + additional domain packs + integration support
- Outcome: multi-workspace production governance with recurring wave verification
- Delivery model: structured enablement with quarterly hardening plan

#### Enterprise

- Scope: Growth + custom policy packs + formal assurance artifacts
- Outcome: organization-wide AI governance fabric and audit readiness
- Delivery model: strategic engagement, SLA-backed operations, roadmap co-design

## 5. ICP, Buyers, and Entry Strategy

### 5.1 Ideal Customer Profile

- Mid-to-large organizations deploying AI in operational workflows
- Regulated or high-trust sectors (finance, healthcare, public sector, infrastructure)
- Teams already experiencing friction between AI teams and governance teams

### 5.2 Economic Buyer

- CIO / CTO / CISO depending on account structure

### 5.3 Champion Personas

- Platform Engineering lead
- Security Engineering lead
- AI Platform / MLOps lead
- Compliance or Risk program owner

### 5.4 Land-and-Expand Motion

1. Land with one high-value workflow (Star Case).
2. Prove controls and speed with wave evidence and operator demo.
3. Expand to additional workflows/domains.
4. Standardize enterprise governance pattern across business units.

## 6. Value Proposition and KPI Framework

### 6.1 Value Pillars

- Risk reduction: fail-closed enforcement on non-contracted effects
- Delivery acceleration: shorter path from prototype to governed production
- Audit readiness: reproducible evidence bundles and verifiable manifests
- Operational scale: reusable runbooks, domain packs, and wave orchestration

### 6.2 KPI Set (Commercial)

Primary commercial KPIs:

- Time-to-governed-deployment (days)
- Policy violation block rate (% prohibited effects blocked)
- Evidence completeness score (%)
- Change lead-time reduction (%)
- Manual governance review hours saved per month

Secondary reliability KPIs:

- Wave reproducibility success rate
- Mean time to containment
- Drift detection and remediation cycle time

## 7. 14-Day Pilot Program (Commercial Offer)

### 7.1 Pilot Objective

Demonstrate measurable governance and delivery improvements on one production-relevant workflow.

### 7.2 Pilot Deliverables

- One configured Star Case workflow under YAI governance
- Baseline deny/allow scenario coverage where applicable
- Wave evidence bundle with manifest and verification output
- Operator demo runbook and recorded walkthrough
- Pilot report with KPI delta and production-readiness recommendation

### 7.3 Pilot Success Criteria

- Controlled denied effects demonstrably blocked (0 forbidden effect success)
- Evidence artifacts complete and replay-verifiable
- Stakeholder sign-off from engineering + security/compliance
- Clear expansion backlog defined (next 2-3 domain/workflow targets)

### 7.4 Pilot Commercial Outcome

- Convert to annual platform subscription and implementation package
- Use pilot KPI delta as ROI baseline for executive decision

## 8. Pricing and Monetization Model

### 8.1 Pricing Architecture

Use a hybrid model:

- Platform subscription (annual)
- Implementation package (one-time or phased)
- Optional premium services (custom packs, assurance reporting, enablement)

### 8.2 Pricing Drivers

- Number of governed workflows / domains
- Number of environments and workspaces
- Required assurance level and reporting depth
- Support and response expectations

### 8.3 Commercial Discipline

- Tie pricing to measurable control coverage and operational outcomes
- Keep pilot pricing low-friction but outcome-bound
- Protect enterprise margin through standardization of runbooks and wave tooling

## 9. Go-To-Market Plan

### 9.1 Phase 1: Design Partner Cohort

- Target 3-5 design partners with acute governance pain
- Co-develop proof narratives and case assets
- Standardize implementation templates

### 9.2 Phase 2: Structured Pipeline

- Publish repeatable Star Case narrative with quantified outcomes
- Build partner-led delivery capability
- Expand channel through security and platform integrators

### 9.3 Phase 3: Scale

- Productize domain pack onboarding
- Expand industry-specific governance templates
- Establish formal customer advisory board and release cadence alignment

## 10. Competitive Positioning

### 10.1 Category Position

YAI should be positioned as AI runtime governance infrastructure, not as an LLM model layer and not as a generic observability tool.

### 10.2 Against Alternatives

- Against ad hoc controls: YAI is consistent and replayable
- Against pure monitoring tools: YAI enforces, not only observes
- Against static policy documents: YAI executes policy at runtime and emits evidence

### 10.3 Defensible Moat

- Runtime plane architecture (Root/Kernel/Engine governance model)
- Domain-pack abstraction (cross-domain scalability)
- Qualification wave system (run -> bundle -> verify)
- Evidence integrity and release linkage discipline

## 11. Operating Model and Org Plan

### 11.1 Core Functions

- Product & Architecture
- Runtime Engineering
- Domain Pack Engineering
- Solutions Engineering / Customer Success
- Governance & Assurance

### 11.2 Cadence

- Weekly: product/runtime sync + risk review
- Bi-weekly: pilot/account review
- Monthly: release wave review and commercial KPI review

## 12. Risks and Mitigations

### 12.1 Technical Risks

- Incomplete runtime attachment diagnostics
- Domain coverage gaps for target customers
- Environmental variability in demos/pilots

Mitigations:

- Strengthen CLI status/doctor semantics and attachment probes
- Prioritize domain roadmap by pipeline demand
- Use standard run environments and strict replay scripts

### 12.2 Commercial Risks

- Long enterprise cycles
- Misclassification as "another AI tool"
- Scope creep during pilots

Mitigations:

- Keep wedge sharp (Star Case first)
- Sell measurable outcomes, not feature breadth
- Pilot contracts with explicit boundaries and success metrics

## 13. 90-Day Execution Plan

### 13.1 Days 0-30

- Finalize Star Case package and collateral
- Qualify design partner shortlist
- Lock pilot template and operational playbook

### 13.2 Days 31-60

- Execute first design partner pilots
- Capture evidence bundles and KPI baselines
- Refine deployment and onboarding experience

### 13.3 Days 61-90

- Convert first pilots to annual contracts
- Publish anonymized case narratives
- Formalize expansion plan across additional domains

## 14. Definition of Commercial Readiness

YAI is commercially ready for broad launch when all are true:

- At least 2 successful pilots converted to paid recurring agreements
- Wave verification is repeatable without manual heroics
- Operator demos are reproducible across customer environments
- KPI improvements are measurable and customer-verifiable

## 15. Immediate Next Actions (This Branch)

1. Create launch collateral package:
   - One-page executive brief
   - 10-slide solution deck
   - Pilot offer sheet
2. Align SC102 wave outputs with commercial report template.
3. Add pricing assumptions worksheet and ROI calculator template.
4. Define customer-facing implementation timeline from pre-sales to go-live.

## 16. Traceability to Catalog and Qualification

Commercial claims are valid only when linked to catalog and qualification artifacts.

Canonical chain:

1. Commercial narrative and offer:
   - `docs/80-commercial/COMMERCIAL-PLAN-v1.0.md`
2. Scenario policy grammar:
   - `docs/30-catalog/scenarios/SC-102.md`
3. Domain pack contracts and evidence requirements:
   - `docs/30-catalog/domains/packs/D1-digital/egress-v1/`
   - `docs/30-catalog/domains/packs/D8-scientific/reproducibility-parameter-lock-v1/`
4. Trial definitions:
   - `docs/30-catalog/domains/trials/D1-digital/egress-v1/`
   - `docs/30-catalog/domains/trials/D8-scientific/reproducibility-parameter-lock-v1/`
5. Qualification execution:
   - `docs/40-qualification/QT-0.1-003-SC102-WAVE1/`
6. Evidence bundle and verification:
   - `docs/40-qualification/WAVES/<wave_id>/`
   - `docs/40-qualification/WAVES/<wave_id>/verify/verify.sh`

Commercial reporting (ROI, pilot outcomes, launch narrative) must always reference a concrete `wave_id`, its `MANIFEST.json`, and `INDEX.md`.

## 17. Star Case Execution Pack (Operational)

### 17.1 Purpose

Standardize one repeatable execution and proof package that Sales, Solutions, and Customer Success can run without custom engineering.

### 17.2 Canonical Entry Points

- Scenario contract: `docs/30-catalog/scenarios/SC-102.md`
- Wave runner: `docs/40-qualification/QT-0.1-003-SC102-WAVE1/run/run-wave.sh`
- Bundle verifier: `docs/40-qualification/WAVES/<wave_id>/verify/verify.sh`

### 17.3 Command Sequence

```bash
cd ~/Developer/YAI/yai

# runtime health
yai down --ws dev --force || true
yai up --ws dev --detach --allow-degraded
yai status --json
yai doctor --json

# star case execution (D1 + D8)
./docs/40-qualification/QT-0.1-003-SC102-WAVE1/run/run-wave.sh

# verify generated bundle
LATEST="$(cat docs/40-qualification/WAVES/LATEST)"
cd "docs/40-qualification/WAVES/$LATEST"
./verify/verify.sh
```

### 17.4 Star Case Output Contract

Required proof assets per run:

- `MANIFEST.json` with release identity (`yai_git_sha`, `yai_cli_git_sha`, `specs_pin_sha`)
- `INDEX.md` with D1 and D8 outcome rows
- selected run evidence directories with decision, metrics, baseline receipt, timeline

### 17.5 Commercial Acceptance Gate

A customer-facing run is valid only if all are true:

- `run-wave.sh` exits `0`
- `verify/verify.sh` exits `0`
- D1 rows show deny + blocked effect (`connect_established=false`, `bytes_exfiltrated=0`)
- D8 row shows deny + blocked persistence (`outputs_persisted=false`, `bytes_written=0`, `artifacts_delta=0`)

### 17.6 Packaging Rule

Customer decks, pilot reports, and ROI claims must include:

- exact `wave_id`
- path to `MANIFEST.json`
- path to `INDEX.md`
- verify result stamp (pass/fail + timestamp)

