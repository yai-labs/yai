# YAI PR Templates (Governance Kit v1)

This folder contains selectable PR templates.

IMPORTANT:
- Do not add `PULL_REQUEST_TEMPLATE.md` at repo root.
- Do not add `.github/PULL_REQUEST_TEMPLATE.md`.
Only this folder must exist.

How to choose a template:

GitHub UI:
- When opening a PR, append `?template=<file>` to the compare URL:
  `.../compare/main...branch?template=20-core-change.md`

GitHub CLI:
- `gh pr create --template 20-core-change.md`

Rules (enforced by CI):
- `## PR-METADATA` YAML block is mandatory.
- Placeholders are forbidden. Use `N/A` if not applicable.
- `## Commands run` is mandatory and must include the exact commands executed.
