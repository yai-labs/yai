# Repo Workflow

## Branching
- One objective per branch.
- Deterministic commit history.
- No unrelated file churn.

## Execution Sequence
1. Apply structural changes.
2. Repair links and README pointers.
3. Run docs validators.
4. Attach evidence and residual risks.

## Repository Anchors
- CLI paths: `cmd/`
- Runtime and subsystem implementation: `lib/`
- Public headers/contracts: `include/yai/`
- Test execution: `tests/`
