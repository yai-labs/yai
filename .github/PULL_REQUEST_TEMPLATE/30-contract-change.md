## PR-METADATA
```yaml
yai_pr: 1
type: contract-change
issue: "#<issue-number>|N/A"
mp: "MP-<TRACK>-<X.Y.Z>|N/A"
runbook: "docs/runbooks/<name>.md|N/A"
base_commit: "<40-char-sha>"
compatibility: "A|B|C"
scope: "law/specs + bindings"
repos_impacted: ["yai", "yai-specs"]
twin_prs:
  yai_cli: N/A
  yai_specs: "<link>|N/A"
```

## Objective

What contract is changing and why.

## Law / Spec refs

- law/specs/…:
- invariants impacted:

  - …

## Breaking surface

- Who breaks?
- What must be bumped/pinned?

## Evidence

- Vectors/tests updated:

  - …

## Commands run

```bash
# paste exact commands
```

## Checklist

- [ ] Spec + implementation in sync
- [ ] Vectors updated when semantics changed
- [ ] Compatibility declared and justified
