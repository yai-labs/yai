---
id: RB-GITHUB-WORKFLOW
status: active
effective_date: 2026-02-17
revision: 3
owner: release/runtime
scope:
  - yai-specs
  - yai-cli
  - yai (runtime + bundle + release)
---

# YAI Release Workflow — Complete Runbook

This document is the single source of truth for the YAI GitHub workflow:
daily development, pin management, release creation, hotfix procedure,
failure recovery, and CI contract.

---

## Workspace convention

```bash
export YAI_WORKSPACE="<path-to-your-yai-workspace>"
```

All commands below assume repositories are located at:

- `$YAI_WORKSPACE/yai-specs`
- `$YAI_WORKSPACE/yai-cli`
- `$YAI_WORKSPACE/yai`

---

## Definitions

| Term | Meaning |
|---|---|
| **Specs** | The contract set in `yai-specs` (protocol, control plane, CLI public interface, compliance packs, etc.) |
| **Consumer repo** | Any repo that vendors/pins `yai-specs` (e.g., `yai`, `yai-cli`) |
| **Pin** | A repository-local reference to an immutable upstream revision (commit SHA or tag) |
| **Runtime Bundle** | The only supported distribution asset published from `yai` releases. MUST include core runtime binaries, CLI, pinned specs, and a manifest |
| **SHA** | 40-character hex Git commit hash (e.g., `abcdef1234567890abcdef1234567890abcdef12`) |

---

## Non-negotiable invariants

**1) Specs source of truth**
Specs MUST be modified only in the `yai-specs` repository.
Consumer repos MUST NOT edit specs under `deps/yai-specs`. Consumers only update pins.

**2) Single distribution artifact**
The `yai` repository MUST publish exactly one official user-facing asset per release: the Runtime Bundle.
"Core-only" artifacts (without CLI) are internal/dev-only and MUST NOT be presented as the primary download.

**3) Deterministic release inputs**
A release tag in `yai` MUST fully determine:
- the exact `yai` runtime source revision,
- the exact `yai-specs` revision (pinned),
- the exact `yai-cli` revision (pinned),
- the produced bundle contents and manifest hashes.

**4) No "latest main" in CI**
CI MUST NOT fetch `main` (or any floating ref) for specs or CLI during bundle creation.
CI MUST use pinned SHAs/tags defined in `yai`.

**5) CI is a verifier and publisher only**
CI (`bundle.yml`) MUST never bump versions, write pins, or create commits on `main`.
All version bumps and pin updates are maintainer-initiated from the local machine.

---

## Repository responsibilities

### yai-specs
- Owns all contract changes.
- Produces stable tags optionally (milestones), but consumers can pin by SHA.

### yai-cli
- Implements the CLI client and consumes `yai-specs` via pinning.
- May publish standalone releases optionally, but product distribution is via `yai` bundle.

### yai
- Builds and distributes the runtime core.
- Vendors `yai-specs` under `deps/yai-specs` (pinned).
- Pins `yai-cli` via `deps/yai-cli.ref`.
- Produces and publishes the Runtime Bundle on tags.

---

## Git reference commands

These commands are used throughout this runbook.

### HEAD SHA and short SHA

```bash
git rev-parse HEAD
git rev-parse --short=12 HEAD
```

### SHA of a branch or tag

```bash
git rev-parse origin/main
git rev-parse origin/feat/some-branch
git rev-parse v0.1.1
git rev-parse refs/tags/v0.1.1^{}   # dereference annotated tag
```

### Log inspection

```bash
# One line per commit
git log --oneline -20
git log --oneline --graph --decorate -20

# Full: SHA, date, author, message
git log --pretty=format:"%H  %ad  %an  %s" --date=short -20

# Commits between two SHAs
git log <OLD_SHA>..<NEW_SHA> --oneline
```

### Commit details

```bash
git show <SHA>
git show <SHA> --stat         # files changed only
git show <SHA> --no-patch     # commit message only
```

### Current state of a repo

```bash
git log --oneline -1
git show HEAD --stat
git branch --show-current
git describe --tags --always  # nearest tag + distance + SHA
git status -sb
```

### Submodule / vendored dep inspection

```bash
git -C deps/yai-specs rev-parse HEAD
git submodule status deps/yai-specs
# Output: +<SHA> deps/yai-specs (<tag or description>)
```

### Check if a SHA exists on the remote

```bash
git -C deps/yai-specs cat-file -t <SHA>   # returns "commit" if present
git ls-remote origin | grep <SHA>
```

### Diff between two SHAs

```bash
git diff <OLD_SHA>..<NEW_SHA> -- .
git -C deps/yai-specs diff <OLD_SHA>..<NEW_SHA> --stat
```

### Tag queries

```bash
git tag --contains <SHA>              # which tags include this commit
git tag --sort=-v:refname | head -10  # latest tags by semver
```

### Read pin files

```bash
cat deps/yai-cli.ref
# Output: cli_sha=abcdef1234567890abcdef1234567890abcdef12

# Extract just the SHA
CLI_SHA=$(grep 'cli_sha=' deps/yai-cli.ref | cut -d= -f2)
echo $CLI_SHA
```

### All three repos at a glance

```bash
for repo in yai-specs yai-cli yai; do
  echo "=== $repo ==="
  git -C $YAI_WORKSPACE/$repo log --oneline -1
  echo ""
done
```

---

## Daily development workflow (no release)

### 1) Specs changes (yai-specs)

```bash
cd $YAI_WORKSPACE/yai-specs
git checkout main
git pull --rebase

git checkout -b feat/specs-<topic>
# --- make changes ---
git add -A
git commit -m "feat(specs): <description>"
git push -u origin feat/specs-<topic>
# Open PR → review → merge to main
```

After merge, record the SHA for consumers:

```bash
git checkout main
git pull --rebase

SPEC_SHA=$(git rev-parse HEAD)
echo "New spec SHA: $SPEC_SHA"
git show $SPEC_SHA --no-patch
```

### 2) CLI changes (yai-cli)

```bash
cd $YAI_WORKSPACE/yai-cli
git checkout main
git pull --rebase

git checkout -b feat/cli-<topic>
# --- make changes ---
make all
make test || true

git add -A
git commit -m "feat(cli): <description>"
git push -u origin feat/cli-<topic>
# Open PR → review → merge to main
```

After merge, record the SHA for the bundle pin:

```bash
git checkout main
git pull --rebase

CLI_SHA=$(git rev-parse HEAD)
echo "New CLI SHA: $CLI_SHA"
git show $CLI_SHA --no-patch
```

### 3) Runtime changes (yai)

```bash
cd $YAI_WORKSPACE/yai
git checkout main
git pull --rebase

git checkout -b feat/runtime-<topic>
# --- make changes ---
make all
make dist
# make bundle   # optional local smoke bundle

git add -A
git commit -m "feat(runtime): <description>"
git push -u origin feat/runtime-<topic>
# Open PR → review → merge to main
```

---

## Pin update procedure

### Update Specs pin in yai (runtime)

```bash
cd $YAI_WORKSPACE/yai
git checkout main && git pull --rebase

git checkout -b chore/bump-specs

git -C deps/yai-specs fetch origin
git -C deps/yai-specs checkout <SPEC_SHA>

# Verify
git -C deps/yai-specs rev-parse HEAD
git -C deps/yai-specs log --oneline -5

git add deps/yai-specs
git commit -m "chore(specs): bump yai-specs pin to <SPEC_SHA_SHORT>"
git push -u origin chore/bump-specs
# Open PR → merge
```

### Update Specs pin in yai-cli

```bash
cd $YAI_WORKSPACE/yai-cli
git checkout main && git pull --rebase

git checkout -b chore/bump-specs

git -C deps/yai-specs fetch origin
git -C deps/yai-specs checkout <SPEC_SHA>

git -C deps/yai-specs rev-parse HEAD   # verify

git add deps/yai-specs
git commit -m "chore(specs): bump yai-specs pin to <SPEC_SHA_SHORT>"
git push -u origin chore/bump-specs
# Open PR → merge
```

### Verify pin state

```bash
echo "Expected: <SPEC_SHA>"
echo "Actual:   $(git -C deps/yai-specs rev-parse HEAD)"

# Check for dirty state (should be empty)
git -C deps/yai-specs status
git -C deps/yai-specs diff
```

### Update CLI pin in yai

```bash
cd $YAI_WORKSPACE/yai
git checkout main && git pull --rebase

git checkout -b chore/bump-cli

CLI_SHA=$(git -C $YAI_WORKSPACE/yai-cli rev-parse HEAD)
echo "cli_sha=$CLI_SHA" > deps/yai-cli.ref
cat deps/yai-cli.ref   # verify

git add deps/yai-cli.ref
git commit -m "chore(cli): bump yai-cli pin to ${CLI_SHA:0:12}"
git push -u origin chore/bump-cli
# Open PR → merge
```

---

## CLI pinning — canonical mechanism

In `yai`, maintain:

```
deps/yai-cli.ref
```

Format (single line, no trailing whitespace):

```
cli_sha=abcdef1234567890abcdef1234567890abcdef12
```

Rules:
- Updating the CLI in the bundle MUST be done by changing `deps/yai-cli.ref` and committing it.
- CI MUST checkout exactly that SHA when building the bundle.
- No other mechanism (e.g., "clone main") is permitted in release workflows.

---

## Cross-check pins vs actual repo state

```bash
cd $YAI_WORKSPACE/yai

SPEC_PIN=$(git -C deps/yai-specs rev-parse HEAD)
CLI_PIN=$(grep 'cli_sha=' deps/yai-cli.ref | cut -d= -f2)

SPEC_ACTUAL=$(git -C $YAI_WORKSPACE/yai-specs rev-parse HEAD)
CLI_ACTUAL=$(git -C $YAI_WORKSPACE/yai-cli rev-parse HEAD)

echo "--- Specs ---"
echo "Pinned:       $SPEC_PIN"
echo "yai-specs HEAD: $SPEC_ACTUAL"
[ "$SPEC_PIN" = "$SPEC_ACTUAL" ] && echo "✓ In sync" || echo "⚠ DRIFT DETECTED"

echo ""
echo "--- CLI ---"
echo "Pinned:       $CLI_PIN"
echo "yai-cli HEAD: $CLI_ACTUAL"
[ "$CLI_PIN" = "$CLI_ACTUAL" ] && echo "✓ In sync" || echo "⚠ DRIFT DETECTED"
```

---

## Release workflow (maintainer checklist)

This is the authoritative release procedure. Follow steps in order.

### Step 1 — Sync and clean working tree

```bash
cd $YAI_WORKSPACE/yai
git checkout main
git pull --rebase
git status
```

### Step 2 — Preflight (hard gate)

```bash
STRICT_SPECS_HEAD=1 bash scripts/release/check_pins.sh
```

`check_pins.sh` enforces:
- specs alignment: `yai` == `yai-cli` == `yai-specs/main` (in strict mode)
- CLI bundle pin compatibility: `deps/yai-cli.ref` commit must pin the same specs revision

**If FAIL:** apply the Fix Plan printed by the script (update specs pins and/or `deps/yai-cli.ref`), merge to `main`, then re-run until PASS.

### Step 3 — Bump version and update changelog

```bash
./scripts/release/bump_version.sh patch --commit
# or: minor / major / X.Y.Z
```

This commits updated `VERSION` and `CHANGELOG.md`.

### Step 4 — Re-run preflight (must be PASS)

```bash
STRICT_SPECS_HEAD=1 bash scripts/release/check_pins.sh
```

### Step 5 — Create annotated tag from VERSION

```bash
VER="$(tr -d '[:space:]' < VERSION)"
git tag -a "v$VER" -m "Release v$VER"
```

### Step 6 — Push main and tag (triggers CI release)

```bash
git push origin main
git push origin "v$VER"
```

### Verify the tag

```bash
git show "v$VER"
git rev-parse "v$VER"^{}   # must match main HEAD
git tag --sort=-v:refname | head -5
```

---

## CI contract (bundle.yml)

On tag `v*`, CI MUST:

1. Run `scripts/release/check_pins.sh` (strict mode) — abort if FAIL.
2. Validate that tag version == `VERSION` and `CHANGELOG.md` contains `[X.Y.Z]`.
3. Build runtime core binaries.
4. Read `deps/yai-cli.ref`, checkout `yai-cli` at the pinned SHA.
5. Stage `deps/yai-specs` snapshot exactly as pinned in `yai`.
6. Generate `manifest.json`:
   - `core_sha` — `yai` git SHA
   - `core_version` — version string
   - `specs_sha` — pinned `yai-specs` SHA
   - `cli_sha` — pinned `yai-cli` SHA
   - `sha256` map — sha256 of each binary in `bin/`
   - `os`, `arch`, `timestamp`, `bundle_version`
7. Produce bundle archives: `.tar.gz` and `.zip`.
8. Publish GitHub Release with those assets.

CI MUST NEVER:
- change pins (`deps/yai-cli.ref`, `deps/yai-specs`)
- bump versions or edit `VERSION` / `CHANGELOG.md`
- create commits on `main`

### Optional: dry run without publishing

Use `workflow_dispatch` on `bundle.yml` to build artifacts without creating a GitHub Release.
Preflight still runs, so drift is detected early.

---

## Pre-release checklist

```bash
cd $YAI_WORKSPACE/yai

# 1. No uncommitted changes in deps
git status deps/
git -C deps/yai-specs status

# 2. Pinned specs SHA is fetchable from upstream
git -C deps/yai-specs cat-file -t $(git -C deps/yai-specs rev-parse HEAD)
# Expected output: commit

# 3. CLI pin file exists and is well-formed
[ -f deps/yai-cli.ref ] && echo "✓ exists" || echo "✗ MISSING"
grep -E '^cli_sha=[0-9a-f]{40}$' deps/yai-cli.ref && echo "✓ format OK" || echo "✗ bad format"

# 4. No pending commits on main
git log origin/main..HEAD --oneline

# 5. Version file matches intended tag
cat VERSION
```

**Policy:** if `deps/yai-specs` differs from the intended pin, the release MUST be blocked until realigned.

---

## Failure modes and recovery

### A) Specs drift

**Symptom:** `check_pins.sh` reports `yai` and `yai-cli` are pinned to different `yai-specs` commits, or pin is behind `yai-specs/main` in strict mode.

**Recovery:**
1. Follow the Fix Plan printed by `check_pins.sh`.
2. Open and merge the bump branches.
3. Re-run preflight until PASS.

### B) CLI bundle pin stale ("telecomando drift")

**Symptom:** `deps/yai-cli.ref` points to a `yai-cli` commit whose `deps/yai-specs` gitlink does not match the expected specs commit.

**Recovery:**

```bash
# Update deps/yai-cli.ref to a compatible yai-cli commit
cd $YAI_WORKSPACE/yai-cli
git log --oneline -10   # find a commit that pins the correct specs

cd $YAI_WORKSPACE/yai
echo "cli_sha=<COMPATIBLE_CLI_SHA>" > deps/yai-cli.ref
git add deps/yai-cli.ref
git commit -m "fix(cli): update cli pin to specs-compatible SHA"
git push origin main
```

### C) Metadata mismatch

**Symptom:** tag `vX.Y.Z` does not match `VERSION`, or `CHANGELOG.md` is missing the `[X.Y.Z]` section.

**Recovery:**

```bash
./scripts/release/bump_version.sh X.Y.Z --commit
# Delete the wrong tag if already pushed
git tag -d vX.Y.Z
git push origin --delete vX.Y.Z
# Re-tag
VER="$(tr -d '[:space:]' < VERSION)"
git tag -a "v$VER" -m "Release v$VER"
git push origin "v$VER"
```

### D) CI cannot checkout yai-cli SHA

**Symptom:** CI reports `fatal: reference is not a repository` or `fatal: not a git repository`.

**Cause:** SHA in `deps/yai-cli.ref` does not exist on the remote, or was never pushed.

**Recovery:**

```bash
cd $YAI_WORKSPACE/yai-cli
git fetch origin
git cat-file -t <CLI_SHA_FROM_REF>
# If NOT "commit": SHA is invalid or not pushed

# Fix: update pin to a valid SHA
cd $YAI_WORKSPACE/yai
echo "cli_sha=$(git -C $YAI_WORKSPACE/yai-cli rev-parse HEAD)" > deps/yai-cli.ref
git add deps/yai-cli.ref
git commit -m "fix(cli): correct cli pin to valid SHA"
git push origin main
```

### E) CI cannot fetch yai-specs pin

**Symptom:** CI reports `fatal: couldn't find remote ref` or submodule checkout fails.

**Cause:** The `deps/yai-specs` commit is not reachable from the remote (force-pushed or never pushed).

**Recovery:**

```bash
cd $YAI_WORKSPACE/yai-specs
git fetch origin
SPEC_SHA=$(git -C $YAI_WORKSPACE/yai/deps/yai-specs rev-parse HEAD)
git cat-file -t $SPEC_SHA
# If NOT "commit": SHA is unreachable

# Re-pin to current valid HEAD
cd $YAI_WORKSPACE/yai
git -C deps/yai-specs fetch origin
git -C deps/yai-specs checkout origin/main
git add deps/yai-specs
git commit -m "fix(specs): re-pin yai-specs to valid upstream SHA"
git push origin main
```

### F) Local bundle smoke test fails

**Symptom:** `make bundle` fails locally before release.

**Debug:**

```bash
cd $YAI_WORKSPACE/yai

make clean-all
make all 2>&1 | tail -30
ls deps/yai-specs/
cat deps/yai-cli.ref
make dist 2>&1
make bundle VERBOSE=1 2>&1
```

---

## Hotfix release (patch on release branch)

Use when a critical bug must be fixed in a shipped version without pulling in unrelated `main` changes.

### Setup

```bash
cd $YAI_WORKSPACE/yai

# Find the target tag
git log --oneline --decorate | grep "tag:"
git rev-parse v0.1.1^{}

# Branch from tag
git checkout -b hotfix/v0.1.2 v0.1.1
```

### Apply the fix

```bash
# Option A: cherry-pick from main
git log --oneline main | head -20
git cherry-pick <FIX_SHA>

# Option B: direct fix on hotfix branch
git add -A
git commit -m "fix: <description of critical fix>"

make all
make test
```

### Release hotfix

```bash
git tag -a v0.1.2 -m "Hotfix release v0.1.2 — <short description>"
git push origin hotfix/v0.1.2
git push origin v0.1.2
git show v0.1.2 --stat
```

### Backport to main (mandatory)

```bash
cd $YAI_WORKSPACE/yai
git checkout main && git pull --rebase

git log --oneline hotfix/v0.1.2 | head -5
git cherry-pick <FIX_SHA>

git push origin main
```

### Cleanup

```bash
git branch -d hotfix/v0.1.2
git push origin --delete hotfix/v0.1.2
```

---

## FAQ

### Do we need tags in yai-cli or yai-specs?

Not required for the product release. The product release is determined by the `yai` tag plus pinned SHAs. Standalone tags in `yai-cli`/`yai-specs` are optional (milestones/audit trail).

### Why pin by SHA instead of branch?

SHA pinning is immutable. A branch ref can be force-pushed or overwritten, making the build non-reproducible. SHA guarantees the exact same source state every time.

### How do I audit what went into a specific release?

```bash
cd $YAI_WORKSPACE/yai
git checkout v0.1.1

cat deps/yai-cli.ref
git -C deps/yai-specs rev-parse HEAD
git -C deps/yai-specs log $(git -C deps/yai-specs rev-parse HEAD) --oneline -5

# Most reliable: read manifest.json from the GitHub Release asset
```

### How do I reconstruct bundle inputs without the binary?

```bash
echo "=== Bundle inputs for HEAD ==="
echo "runtime SHA : $(git rev-parse HEAD)"
echo "specs SHA   : $(git -C deps/yai-specs rev-parse HEAD)"
echo "cli SHA     : $(grep 'cli_sha=' deps/yai-cli.ref | cut -d= -f2)"
```

### What if yai-specs and yai-cli pins get out of sync?

Both repos pin `yai-specs` independently. There is no required sync between them; they reflect what each consumer was built/tested against. If coordinated alignment is needed (e.g., a synchronized release), bump both pins to the same `SPEC_SHA` and verify independently.

---

## Appendix: full state report (one-liner)

```bash
echo "===== YAI State Report $(date -u +%Y-%m-%dT%H:%M:%SZ) =====" && \
echo "" && \
echo "--- yai-specs ---" && \
git -C $YAI_WORKSPACE/yai-specs log --oneline -3 && \
echo "" && \
echo "--- yai-cli ---" && \
git -C $YAI_WORKSPACE/yai-cli log --oneline -3 && \
echo "" && \
echo "--- yai (runtime) ---" && \
git -C $YAI_WORKSPACE/yai log --oneline -3 && \
echo "" && \
echo "--- Pins in yai ---" && \
echo "specs pin : $(git -C $YAI_WORKSPACE/yai/deps/yai-specs rev-parse HEAD 2>/dev/null || echo 'NOT FOUND')" && \
echo "cli pin   : $(grep 'cli_sha=' $YAI_WORKSPACE/yai/deps/yai-cli.ref 2>/dev/null || echo 'NOT FOUND')" && \
echo ""
```