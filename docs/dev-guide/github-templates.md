# GitHub Templates & Governance Kit (v1)

This repository uses selectable PR templates and enforced PR metadata.

## PR templates
Location: `.github/PULL_REQUEST_TEMPLATE/`

Rules:
- Only this folder must exist (no single-template files).
- Every PR must contain `## PR-METADATA` with a YAML fenced block.
- Placeholders are forbidden. Use `N/A`.

## CLI usage (agents/humans)
Create PR with explicit template:

```bash
gh pr create --template 20-core-change.md
```

Update body from template (recommended automation):

```bash
BASE_SHA="$(git rev-parse HEAD)"
tmp="$(mktemp)"
cp .github/PULL_REQUEST_TEMPLATE/20-core-change.md "$tmp"
perl -pi -e "s/<40-char-sha>/$BASE_SHA/g" "$tmp"
perl -pi -e "s/#<issue-number>/N\/A/g" "$tmp"
gh pr edit --body-file "$tmp"
```

## Enforcement

Workflow: `.github/workflows/validate-pr-metadata.yml`

Fails when:

- missing PR-METADATA
- placeholders exist
- missing Commands run section
