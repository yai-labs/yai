# Policy Resolution

## Resolution order

`yai` resolves normative decisions from embedded law using:

1. operational classification
2. family selection
3. specialization selection
4. foundation baseline
5. domain specialization policy pack
6. regulatory overlays
7. sector overlays
8. contextual overlays (when present)
9. authority contributor aggregation
10. evidence contributor aggregation
11. precedence + final effect mapping

## Compositional expectations

Overlays are active normative layers (not metadata):
- they can restrict or deny
- they can force review/escalation
- they can raise authority scope burden
- they can harden evidence/retention/provenance obligations

## Overlay-driven examples

- `payment.authorize` + `sector.finance` + `retention-governance` -> review/escalation + evidence hardening
- `github.publish.personal-data` + `gdpr-eu` -> review/deny path depending on lawful-basis context
- `experiment.run.high-risk` + `ai-act` -> high-risk review + oversight-oriented rationale
- provider trust downgrade + `security-supply-chain` -> review/quarantine shift

## Runtime effect output

Resolver outputs:
- resolved family/domain/subdomain context
- family candidates and selected specialization context in trace payload
- overlay attachment split by class: `regulatory`, `sector`, `contextual`
- applied rules and precedence notes
- final effect (`allow|deny|quarantine|review_required|degrade|require_justification`)
- aggregated evidence obligations (with additive overlay contributors)
- authority requirement summary (baseline + overlay uplifts)
- contributor-aware profiles (`authority_profile`, `evidence_profile`)

Workspace model integration:
- declared context acts as strong initial hint
- inferred context is updated from real classification/discovery outcomes
- effective summaries are persisted as workspace inspectable state
- workspace execution path (`yai ws run ...`) resolves through the same law-driven stack and updates workspace summaries

## Determinism

Resolution is deterministic for same input + same embedded law version.

## WS-3 Inspectability Surfaces

Workspace-facing inspection endpoints now expose this state:
- `yai.workspace.policy.effective`: effective family/specialization + stack/overlay/effect summaries
- `yai.workspace.debug.resolution`: compact declared/inferred/effective debug summary
- `yai.workspace.inspect`: full workspace-level snapshot including last resolution summary
