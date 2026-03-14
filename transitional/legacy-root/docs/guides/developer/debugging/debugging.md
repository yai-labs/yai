---
role: support
status: active
audience: developer
owner_domain: guides
---

# Debugging

# Purpose
Provides actionable guidance for operators or developers.

# Scope
Covers practical guidance and usage patterns for the section audience.

# Relationships
- Architecture source docs
- Runbooks for executable procedures

# Canonical Role
Supporting guide aligned to canonical architecture and runbooks.

# Main Body
For docs/governance pipelines, debug in this order:

1. Path contract mismatch (old vs new canonical paths)
2. Frontmatter schema violations
3. Traceability graph/linkage gaps
4. Generated artifact drift

Always capture failing command output in evidence files.

# Related Docs
- `docs/guides/README.md`
- Section README and related guide pages
