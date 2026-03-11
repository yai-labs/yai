#!/usr/bin/env python3
from pathlib import Path

REQUIRED = {
    "README.md": ["single canonical repository", "governance/", "foundation/", "formal/"],
    "FOUNDATION.md": ["single canonical system root", "governance/", "foundation/", "formal/"],
    "GOVERNANCE.md": ["No external governance repository is required"],
    "COMPATIBILITY.md": ["single-repository", "governance/contracts/"],
    "VERSIONING.md": ["single-repository", "v1.0.0"],
}

FORBIDDEN = [
    "embedded/law",
    "../law",
    "dual-repo",
    "runtime-embedded",
    "YAI_LAW_",
]


def main() -> int:
    repo = Path(__file__).resolve().parents[2]
    errors = []

    for rel, needles in REQUIRED.items():
        p = repo / rel
        if not p.exists():
            errors.append(f"missing file: {rel}")
            continue
        text = p.read_text(encoding="utf-8", errors="ignore")
        for needle in needles:
            if needle not in text:
                errors.append(f"{rel}: missing required phrase '{needle}'")

    for rel in REQUIRED:
        text = (repo / rel).read_text(encoding="utf-8", errors="ignore")
        for token in FORBIDDEN:
            if token in text:
                errors.append(f"{rel}: forbidden legacy token '{token}'")

    if errors:
        print("unified_repo_root_framing: FAIL")
        for e in errors:
            print(" -", e)
        return 1

    print("unified_repo_root_framing: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
