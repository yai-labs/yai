# Workspace Hostile-Path Baseline (WS-11)

This baseline defines adversarial-but-realistic misuse classes covered in current runtime hardening.

## Covered Classes

- stale binding abuse
- cross-workspace scope misuse
- path traversal in workspace root options
- degraded-mode misuse visibility (requested vs effective)
- attach/run lifecycle misuse with invalid active context

## Baseline Goal

Reject or clearly signal ambiguous/unsafe runtime states before continuing execution.

## Out of Scope (for this tranche)

- full OS sandbox escape prevention
- kernel/container isolation primitives
- deep exploit research/pentest workflows
