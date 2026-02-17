---
id: RB-GITHUB-WORKFLOW
status: active
effective_date: 2026-02-17
revision: 2
owner: release/runtime
scope:
  - yai-specs
  - yai-cli
  - yai (runtime + bundle + release)
---

# YAI GitHub Workflow Runbook (Daily Dev + Release)

## Purpose

Define the canonical GitHub workflow for the YAI ecosystem to ensure:
- deterministic builds and releases,
- strict dependency pinning (specs + CLI),
- a single supported distribution artifact (the Runtime Bundle),
- prevention of version mismatches between Runtime, CLI, and Specs.

This runbook applies to:
- `yai-specs` — source of truth for contracts/specifications,
- `yai-cli` — command/control client,
- `yai` — runtime core + bundling + distribution.

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
- Pins `yai-cli` via `deps/yai-cli.ref` (see [CLI pinning](#cli-pinning-required-for-deterministic-bundles)).
- Produces and publishes the Runtime Bundle on tags.

---

## Git reference commands (quick lookup)

These commands are used throughout this runbook. Keep them handy.

### Get the current HEAD SHA

```bash
git rev-parse HEAD
```

### Get a short SHA (first 12 chars)

```bash
git rev-parse --short=12 HEAD
```

### Get the SHA of a specific branch

```bash
git rev-parse origin/main
git rev-parse origin/feat/some-branch
```

### Get the SHA of a tag

```bash
git rev-parse v0.1.1
git rev-parse refs/tags/v0.1.1^{}   # dereferenced tag (for annotated tags)
```

### Show the log with SHAs (one line per commit)

```bash
git log --oneline -20
git log --oneline --graph --decorate -20
```

### Show full log with author, date, SHA, and message

```bash
git log --pretty=format:"%H  %ad  %an  %s" --date=short -20
```

### Find the SHA of a submodule/dep at a given path

```bash
git -C deps/yai-specs rev-parse HEAD
git -C deps/yai-cli rev-parse HEAD   # if vendored as subdir
```

### Show what a pinned submodule currently resolves to

```bash
git submodule status deps/yai-specs
# Output: +<SHA> deps/yai-specs (<tag or description>)
```

### Check if a SHA exists on the remote

```bash
git -C deps/yai-specs cat-file -t <SHA>    # returns "commit" if present
git ls-remote origin | grep <SHA>
```

### Show the diff between two SHAs

```bash
git diff <OLD_SHA>..<NEW_SHA> -- .
git log <OLD_SHA>..<NEW_SHA> --oneline
```

### Find which tag contains a given SHA

```bash
git tag --contains <SHA>
```

### Show commit info for a specific SHA

```bash
git show <SHA>
git show <SHA> --stat           # files changed only
git show <SHA> --no-patch       # commit message only
```

### Show the SHA recorded by the parent repo for a vendored dep

```bash
cat deps/yai-cli.ref            # for yai-cli pin
git -C deps/yai-specs rev-parse HEAD   # for specs submodule
```

---

## Daily development workflow (no release)

### 1) Specs changes (yai-specs)

```bash
cd ~/Developer/YAI/yai-specs
git checkout main
git pull --rebase

git checkout -b feat/specs-<topic>

# --- make changes ---

git add -A
git commit -m "feat(specs): <description of change>"
git push -u origin feat/specs-<topic>
# Open PR → review → merge to main
```

After merge, record the new spec SHA for use in consumer repos:

```bash
git checkout main
git pull --rebase

# Full SHA (use this in deps/yai-specs)
SPEC_SHA=$(git rev-parse HEAD)
echo "New spec SHA: $SPEC_SHA"

# Confirm commit details
git show $SPEC_SHA --no-patch
```

### 2) CLI changes (yai-cli)

```bash
cd ~/Developer/YAI/yai-cli
git checkout main
git pull --rebase

git checkout -b feat/cli-<topic>

# --- make changes ---

make all
make test || true

git add -A
git commit -m "feat(cli): <description of change>"
git push -u origin feat/cli-<topic>
# Open PR → review → merge to main
```

After merge, record the CLI SHA for the bundle pin:

```bash
git checkout main
git pull --rebase

CLI_SHA=$(git rev-parse HEAD)
echo "New CLI SHA: $CLI_SHA"

git show $CLI_SHA --no-patch
```

### 3) Runtime changes (yai)

```bash
cd ~/Developer/YAI/yai
git checkout main
git pull --rebase

git checkout -b feat/runtime-<topic>

# --- make changes ---

make all
make dist
# make bundle   # optional local smoke bundle

git add -A
git commit -m "feat(runtime): <description of change>"
git push -u origin feat/runtime-<topic>
# Open PR → review → merge to main
```

---

## Pin update procedure (consumers)

### Update Specs pin in yai (runtime)

```bash
cd ~/Developer/YAI/yai
git checkout main
git pull --rebase

git checkout -b chore/bump-specs

# Fetch latest from upstream
git -C deps/yai-specs fetch origin

# Checkout the desired SHA (from yai-specs step above)
git -C deps/yai-specs checkout <SPEC_SHA>

# Verify the pin is correct
git -C deps/yai-specs rev-parse HEAD
git -C deps/yai-specs log --oneline -5

git add deps/yai-specs
git commit -m "chore(specs): bump yai-specs pin to <SPEC_SHA_SHORT>"
git push -u origin chore/bump-specs
# Open PR → merge
```

### Update Specs pin in yai-cli

```bash
cd ~/Developer/YAI/yai-cli
git checkout main
git pull --rebase

git checkout -b chore/bump-specs

git -C deps/yai-specs fetch origin
git -C deps/yai-specs checkout <SPEC_SHA>

# Verify
git -C deps/yai-specs rev-parse HEAD

git add deps/yai-specs
git commit -m "chore(specs): bump yai-specs pin to <SPEC_SHA_SHORT>"
git push -u origin chore/bump-specs
# Open PR → merge
```

### Verify pin state before continuing

```bash
# Check what SHA deps/yai-specs currently resolves to
git -C deps/yai-specs rev-parse HEAD

# Check it matches what you expect
echo "Expected: <SPEC_SHA>"
echo "Actual:   $(git -C deps/yai-specs rev-parse HEAD)"

# Check for dirty state (should be empty)
git -C deps/yai-specs status
git -C deps/yai-specs diff
```

---

## CLI pinning (required for deterministic bundles)

`yai` MUST pin the exact CLI revision included in the Runtime Bundle.

### Canonical mechanism

In `yai`, maintain a file:

```
deps/yai-cli.ref
```

Format (single line, no trailing whitespace):

```
cli_sha=abcdef1234567890abcdef1234567890abcdef12
```

Rules:
- Updating the CLI included in the bundle MUST be done by changing `deps/yai-cli.ref` and committing it to `yai`.
- CI MUST checkout exactly that SHA when building the bundle.
- No other mechanism (e.g., "clone main") is permitted in release workflows.

### Update CLI pin in yai

```bash
cd ~/Developer/YAI/yai
git checkout main
git pull --rebase

git checkout -b chore/bump-cli

# Get the SHA from yai-cli
CLI_SHA=$(git -C ~/Developer/YAI/yai-cli rev-parse HEAD)
echo "cli_sha=$CLI_SHA" > deps/yai-cli.ref

# Verify file content
cat deps/yai-cli.ref

git add deps/yai-cli.ref
git commit -m "chore(cli): bump yai-cli pin to ${CLI_SHA:0:12}"
git push -u origin chore/bump-cli
# Open PR → merge
```

### Read current CLI pin

```bash
cat deps/yai-cli.ref
# Output: cli_sha=abcdef1234567890abcdef1234567890abcdef12

# Extract just the SHA
CLI_SHA=$(grep 'cli_sha=' deps/yai-cli.ref | cut -d= -f2)
echo $CLI_SHA
```

---

## Product release workflow (bundle publish)

### Release principle

Only `yai` tags create user-facing releases.
A `yai` tag (e.g., `v0.1.1`) triggers CI to build and publish the Runtime Bundle asset.

### Preconditions (verify before tagging)

```bash
cd ~/Developer/YAI/yai
git checkout main
git pull --rebase

# 1. Confirm specs pin is at the intended SHA
echo "Specs pin: $(git -C deps/yai-specs rev-parse HEAD)"
git -C deps/yai-specs log --oneline -3

# 2. Confirm CLI pin is set
cat deps/yai-cli.ref

# 3. Confirm no dirty state in deps
git -C deps/yai-specs status
git status deps/

# 4. Confirm current main is what you expect
git log --oneline -5

# 5. Run local smoke checks
make clean-all
make all
make dist
make bundle
```

### Release commands

```bash
cd ~/Developer/YAI/yai
git checkout main
git pull --rebase

# Option A: use release script (recommended)
./scripts/release/bump_version.sh patch --commit --tag

# Option B: manual tag
git tag -a v0.1.1 -m "Release v0.1.1"

# Push both main and the tag
git push origin main
git push origin v0.1.1   # or: git push origin --tags
```

### Verify the tag was created correctly

```bash
# Show tag details
git show v0.1.1

# Confirm the tag SHA matches main HEAD
git rev-parse v0.1.1^{}
git rev-parse main

# List tags sorted by version
git tag --sort=-v:refname | head -10
```

### CI expectations (on tag `v*` in `yai`)

On tag push, CI MUST:

1. Validate `VERSION` / `CHANGELOG` if required.
2. Build runtime core binaries.
3. Read `deps/yai-cli.ref`, checkout `yai-cli` at the pinned SHA.
4. Stage `deps/yai-specs` snapshot exactly as pinned in `yai`.
5. Generate `manifest.json` containing:
   - `core_sha` — `yai` git SHA
   - `core_version` — version string
   - `specs_sha` — pinned `yai-specs` SHA
   - `cli_sha` — pinned `yai-cli` SHA
   - `sha256` map — sha256 of each binary in `bin/`
   - `os`, `arch`, `timestamp`, `bundle_version`
6. Produce bundle archives: `.tar.gz` and/or `.zip`.
7. Publish GitHub Release with those assets.

---

## State inspection commands (full reference)

Use these at any time to understand the current state of any repo or pin.

### Inspect current repo state

```bash
# Where am I and what SHA am I at?
git log --oneline -1

# Full details of HEAD commit
git show HEAD --stat

# What branch/tag am I on?
git branch --show-current
git describe --tags --always   # nearest tag + distance + SHA

# What's the remote tracking state?
git status -sb
git fetch --dry-run
```

### Inspect all three repos at once

```bash
for repo in yai-specs yai-cli yai; do
  echo "=== $repo ==="
  git -C ~/Developer/YAI/$repo log --oneline -1
  echo ""
done
```

### Cross-check pins vs actual repo state

```bash
cd ~/Developer/YAI/yai

SPEC_PIN=$(git -C deps/yai-specs rev-parse HEAD)
CLI_PIN=$(grep 'cli_sha=' deps/yai-cli.ref | cut -d= -f2)

SPEC_ACTUAL=$(git -C ~/Developer/YAI/yai-specs rev-parse HEAD)
CLI_ACTUAL=$(git -C ~/Developer/YAI/yai-cli rev-parse HEAD)

echo "--- Specs ---"
echo "Pinned:  $SPEC_PIN"
echo "yai-specs HEAD: $SPEC_ACTUAL"
[ "$SPEC_PIN" = "$SPEC_ACTUAL" ] && echo "✓ In sync" || echo "⚠ DRIFT DETECTED"

echo ""
echo "--- CLI ---"
echo "Pinned:  $CLI_PIN"
echo "yai-cli HEAD: $CLI_ACTUAL"
[ "$CLI_PIN" = "$CLI_ACTUAL" ] && echo "✓ In sync" || echo "⚠ DRIFT DETECTED"
```

### Show what changed between two pins

```bash
# What changed in specs between two SHAs?
git -C deps/yai-specs log <OLD_SHA>..<NEW_SHA> --oneline

# What files changed?
git -C deps/yai-specs diff <OLD_SHA>..<NEW_SHA> --stat
```

---

## Operational checks (pre-release checklist)

Before every release, run the following checks:

```bash
cd ~/Developer/YAI/yai

# 1. No uncommitted changes in deps
git status deps/
git -C deps/yai-specs status

# 2. Pinned specs SHA is fetchable from upstream
git -C deps/yai-specs cat-file -t $(git -C deps/yai-specs rev-parse HEAD)
# Expected output: commit

# 3. CLI pin file exists and is well-formed
[ -f deps/yai-cli.ref ] && echo "✓ exists" || echo "✗ MISSING"
grep -E '^cli_sha=[0-9a-f]{40}$' deps/yai-cli.ref && echo "✓ format OK" || echo "✗ bad format"

# 4. No pending commits needed in yai/main
git log origin/main..HEAD --oneline

# 5. Confirm version file matches intended tag
cat VERSION   # or wherever the version is stored
```

**Policy: consumer repos MUST NOT drift.**
If `deps/yai-specs` differs from the intended pin, the release MUST be blocked until realigned.

---

## Failure modes and recovery

### CI fails: cannot checkout yai-cli SHA

**Symptom:** CI reports `fatal: reference is not a repository` or `fatal: not a git repository` when checking out the CLI pin.

**Cause:** The SHA in `deps/yai-cli.ref` does not exist in `yai-cli` remote, or the repo URL is misconfigured.

**Recovery:**

```bash
# On your local machine, verify the SHA exists in yai-cli
cd ~/Developer/YAI/yai-cli
git fetch origin
git cat-file -t <CLI_SHA_FROM_REF>
# If output is NOT "commit", the SHA is invalid or not pushed

# Check what the latest main SHA is
git log --oneline -5

# Update the pin to a valid SHA and re-push
cd ~/Developer/YAI/yai
echo "cli_sha=$(git -C ~/Developer/YAI/yai-cli rev-parse HEAD)" > deps/yai-cli.ref
git add deps/yai-cli.ref
git commit -m "fix(cli): correct cli pin to valid SHA"
git push origin main
```

### CI fails: cannot fetch yai-specs pin

**Symptom:** CI reports `fatal: couldn't find remote ref` or submodule checkout fails.

**Cause:** The `deps/yai-specs` submodule commit is not reachable from the remote (e.g., was force-pushed over or never pushed).

**Recovery:**

```bash
# Verify the SHA exists on yai-specs remote
cd ~/Developer/YAI/yai-specs
git fetch origin
SPEC_SHA=$(cat ../yai/deps/yai-specs/.git 2>/dev/null || git -C ../yai/deps/yai-specs rev-parse HEAD)
git cat-file -t $SPEC_SHA
# If NOT "commit", SHA is unreachable

# Push the missing commit if it exists locally
git push origin $SPEC_SHA:refs/heads/main   # only if it's a legit commit

# Or re-pin to the current valid HEAD
cd ~/Developer/YAI/yai
git -C deps/yai-specs fetch origin
git -C deps/yai-specs checkout origin/main
git add deps/yai-specs
git commit -m "fix(specs): re-pin yai-specs to valid upstream SHA"
git push origin main
```

### CI fails: manifest SHA mismatch

**Symptom:** Bundle manifest `specs_sha` or `cli_sha` does not match expected values.

**Cause:** CI is not reading the pin files correctly, or the pin files have incorrect content.

**Recovery:**

```bash
# Verify pin files locally
cat deps/yai-cli.ref
git -C deps/yai-specs rev-parse HEAD

# Compare with manifest from last successful release
# (check GitHub Releases for the manifest.json artifact)

# If pin files are correct, check CI script logic:
# CI must do: CLI_SHA=$(grep 'cli_sha=' deps/yai-cli.ref | cut -d= -f2)
# not:        CLI_SHA=$(git -C yai-cli rev-parse HEAD)  ← WRONG, reads floating HEAD
```

### Local bundle smoke test fails

**Symptom:** `make bundle` fails locally before release.

**Debug steps:**

```bash
cd ~/Developer/YAI/yai

# Check build artifacts
make clean-all
make all 2>&1 | tail -30

# Verify deps are in expected state
ls deps/yai-specs/
cat deps/yai-cli.ref

# Try building dist separately
make dist 2>&1

# Verbose bundle
make bundle VERBOSE=1 2>&1
```

---

## Hotfix release (patch on release branch)

Use when a critical bug must be fixed in a shipped version without pulling in unrelated main changes.

### Setup hotfix branch

```bash
cd ~/Developer/YAI/yai

# Find the tag SHA you want to hotfix
git log --oneline --decorate | grep "tag:"
git rev-parse v0.1.1^{}   # confirm the target tag SHA

# Create a hotfix branch from the release tag
git checkout -b hotfix/v0.1.2 v0.1.1
```

### Apply the fix

```bash
# Option A: cherry-pick a specific commit from main
git log --oneline main | head -20    # find the fix commit SHA
git cherry-pick <FIX_SHA>

# Option B: apply fix directly on hotfix branch
# edit files
git add -A
git commit -m "fix: <description of critical fix>"

# Verify the fix
make all
make test
```

### Release the hotfix

```bash
# Tag directly from the hotfix branch
git tag -a v0.1.2 -m "Hotfix release v0.1.2 — <short description>"

# Push branch and tag
git push origin hotfix/v0.1.2
git push origin v0.1.2

# Confirm
git show v0.1.2 --stat
```

### Backport to main (mandatory)

After the hotfix is released, the fix MUST be backported to `main` to avoid regression.

```bash
cd ~/Developer/YAI/yai
git checkout main
git pull --rebase

# Cherry-pick the hotfix commit(s)
git log --oneline hotfix/v0.1.2 | head -5   # find the fix SHA(s)
git cherry-pick <FIX_SHA>

git push origin main
```

### Cleanup

```bash
# Delete hotfix branch after backport is confirmed
git branch -d hotfix/v0.1.2
git push origin --delete hotfix/v0.1.2
```

---

## FAQ

### Do we need tags in yai-cli or yai-specs?

Not required for the product release.
The product release is determined by the `yai` tag plus pinned SHAs.
Standalone tags in `yai-cli`/`yai-specs` are optional (milestones/audit trail).

### Why pin by SHA instead of branch?

SHA pinning is immutable. A branch ref can be force-pushed or overwritten, making the build non-reproducible. SHA guarantees the exact same source state every time.

### How do I audit what went into a specific release?

```bash
cd ~/Developer/YAI/yai

# Checkout the release tag
git checkout v0.1.1

# Read the pins
cat deps/yai-cli.ref
git -C deps/yai-specs rev-parse HEAD

# Show specs at that pin
git -C deps/yai-specs log $(git -C deps/yai-specs rev-parse HEAD) --oneline -5

# Or read the published manifest.json from GitHub Releases (most reliable)
```

### How do I see all SHAs that went into a bundle without the binary?

```bash
# Read manifest.json from the GitHub Release asset for that version
# Or reconstruct locally:
echo "=== Bundle inputs for HEAD ==="
echo "runtime SHA : $(git rev-parse HEAD)"
echo "specs SHA   : $(git -C deps/yai-specs rev-parse HEAD)"
echo "cli SHA     : $(grep 'cli_sha=' deps/yai-cli.ref | cut -d= -f2)"
```

### What if yai-specs and yai-cli get out of sync with each other?

Both pin `yai-specs` independently. They MUST be updated separately. There is no required sync between the `yai-specs` pin in `yai-cli` and the one in `yai`; they reflect what each consumer was built/tested against.

If you need them aligned (e.g., for a coordinated release), bump both pins to the same `SPEC_SHA` and verify independently.

---

## Appendix: one-liner state report

Paste this into any terminal to get a full snapshot of all three repos and current pins:

```bash
echo "===== YAI State Report $(date -u +%Y-%m-%dT%H:%M:%SZ) =====" && \
echo "" && \
echo "--- yai-specs ---" && \
git -C ~/Developer/YAI/yai-specs log --oneline -3 && \
echo "" && \
echo "--- yai-cli ---" && \
git -C ~/Developer/YAI/yai-cli log --oneline -3 && \
echo "" && \
echo "--- yai (runtime) ---" && \
git -C ~/Developer/YAI/yai log --oneline -3 && \
echo "" && \
echo "--- Pins in yai ---" && \
echo "specs pin : $(git -C ~/Developer/YAI/yai/deps/yai-specs rev-parse HEAD 2>/dev/null || echo 'NOT FOUND')" && \
echo "cli pin   : $(grep 'cli_sha=' ~/Developer/YAI/yai/deps/yai-cli.ref 2>/dev/null || echo 'NOT FOUND')" && \
echo ""
```