# Domain Discovery

## Runtime role

`yai` maps runtime operations to family/domain/subdomain candidates before policy resolution.

## Inputs

- action class
- provider class
- protocol class
- resource class
- workspace declared hints (`declared_control_family`, `declared_specialization`)
- workspace inferred hints (last inference checkpoint)
- authority hints

## Discovery output

Discovery emits:
- `family_id`
- `domain_id` compatibility id (internal bridge output)
- specialization candidate set
- selected specialization
- family candidate ranking
- confidence and rationale

This output is consumed by resolver, which composes regulatory/sector/contextual overlays over the selected domain context.
Resolver may confirm or refine workspace declared hints and writes back effective summaries for workspace inspection surfaces.

## Runtime genericity pass I

Discovery now routes in this order:
1. classification signal extraction
2. family candidate scoring/ranking
3. specialization candidate selection within selected family
4. compatibility `domain_id` mapping for bridge continuity

Overlay-sensitive signals considered in runtime-facing paths:
- provider trust hints (for security-supply-chain)
- personal-data publication hints (for GDPR)
- high-risk experiment hints (for AI Act)

## WS-3 workspace observability bridge

Workspace inspection now exposes discovery-aligned context snapshots through:
- `yai.workspace.domain.get` (declared + inferred + effective routing view)
- `yai.workspace.inspect` (full declared/inferred/effective state snapshot)
- `yai.workspace.run` (runtime action execution that consumes declared hints and refreshes inferred/effective snapshots)
