# YAI ROI Model v1.0

## Purpose

Provide a practical framework to estimate economic impact from deploying YAI governance controls.

## Core formula

Annual ROI = (Risk reduction value + Efficiency gain value + Audit acceleration value) - Annual YAI cost

## Input dimensions

### A. Risk reduction

- Number of high-risk AI workflow changes per year
- Estimated incident probability without YAI
- Estimated incident probability with YAI
- Average incident cost

Risk reduction value = (P_without - P_with) x Incident_cost x Changes_per_year

### B. Efficiency gain

- Current average governance review hours per change
- Future review hours per change with YAI
- Fully loaded hourly cost

Efficiency gain value = (Hours_without - Hours_with) x Hourly_cost x Changes_per_year

### C. Audit acceleration

- Number of audit cycles per year
- Hours saved per cycle from pre-structured evidence bundles
- Fully loaded hourly cost

Audit acceleration value = Hours_saved_per_cycle x Hourly_cost x Audit_cycles

## KPI mapping to wave evidence

Use SC102 wave outputs to anchor assumptions:

- Containment effectiveness from `INDEX.md` key metrics
- Evidence completeness from bundle verification outputs
- Operational repeatability from run success rate across wave runs

## Reporting template

For each customer account include:

1. Baseline assumptions table
2. Target-state assumptions table
3. Conservative / expected / aggressive ROI scenarios
4. Risks and confidence notes
5. Recommendation

## Notes

- Use conservative assumptions for first commercial proposal.
- Recompute after pilot completion with measured values.
- Always reference concrete `wave_id` in ROI appendices.


## Golden Wave Seed Metrics (SC102-WAVE1-LAUNCH)

From `WAVE-1-2026-02-25-0e7af41` (`INDEX.md`):

- D1 denied egress attempts blocked: 3/3 selected runs (target_reachable=true, connect_established=false, bytes_exfiltrated=0)
- D8 denied publish attempt blocked: outputs_persisted=false, bytes_written=0, artifacts_delta=0

Use these as conservative baseline proof points until pilot-specific measured numbers replace assumptions.
