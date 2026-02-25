#!/usr/bin/env python3
import argparse
import hashlib
import json
from pathlib import Path
import sys

REQUIRED_FILES = [
    "baseline.json",
    "timeline.jsonl",
    "decision_records.jsonl",
    "containment_metrics.json",
    "system_state.txt",
    "EVIDENCE_INDEX.md",
]


def ensure(cond, msg):
    if not cond:
        raise RuntimeError(msg)


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def get_field(obj, dotted):
    cur = obj
    for part in dotted.split("."):
        if not isinstance(cur, dict) or part not in cur:
            return None
        cur = cur[part]
    return cur


def verify_policy_files(bundle_root: Path, manifest: dict):
    items = manifest.get("policy_files", [])
    ensure(items, "policy_files missing in manifest")
    for item in items:
        policy_path = bundle_root / item["policy_ref"]
        ensure(policy_path.exists(), f"missing policy file: {item['policy_ref']}")
        observed = sha256_file(policy_path)
        ensure(observed == item["policy_hash"], f"policy hash mismatch: {item['policy_ref']}")


def verify_run(bundle_root: Path, run: dict, repo_root: Path):
    run_dir = bundle_root / run["bundle_run_dir"]
    ensure(run_dir.is_dir(), f"missing run dir: {run_dir}")
    hashes = {}
    for fname in REQUIRED_FILES:
        fp = run_dir / fname
        ensure(fp.exists(), f"missing required file {fname} in {run['id']}")
        hashes[fname] = sha256_file(fp)
    ensure(hashes == run["required_file_hashes"], f"required file hash mismatch in {run['id']}")

    lines = [x for x in (run_dir / "decision_records.jsonl").read_text(encoding="utf-8").splitlines() if x.strip()]
    ensure(lines, f"empty decision records in {run['id']}")
    decision = json.loads(lines[-1])

    required_ref = repo_root / run["required_fields_ref"]
    req_fields = json.loads(required_ref.read_text(encoding="utf-8"))["required_fields"]
    for field in req_fields:
        ensure(get_field(decision, field) is not None, f"missing required field {field} in {run['id']}")

    contract_ref = repo_root / run["baseline_contract_ref"]
    baseline_sha = sha256_file(contract_ref)
    ensure(decision["decision"]["baseline_hash"] == baseline_sha, f"decision baseline hash mismatch in {run['id']}")
    receipt = json.loads((run_dir / "baseline.json").read_text(encoding="utf-8"))
    ensure(receipt.get("baseline_hash") == baseline_sha, f"receipt baseline hash mismatch in {run['id']}")

    policy_path = bundle_root / run["policy_ref"]
    ensure(policy_path.exists(), f"missing run policy ref in bundle: {run['id']}")
    ensure(sha256_file(policy_path) == run["policy_hash"], f"run policy hash mismatch in {run['id']}")

    ensure(decision["decision"]["outcome"] == run["expected"]["outcome"], f"outcome mismatch in {run['id']}")
    ensure(decision["decision"]["reason_code"] in run["expected"]["reason_codes"], f"reason mismatch in {run['id']}")

    metrics = decision.get("metrics", {})
    if run["pack_id"].startswith("D1-digital"):
        if run["expected"]["outcome"] == "allow":
            ensure(bool(metrics.get("connect_established", False)), f"D1 allow must set connect_established=true in {run['id']}")
            ensure(int(metrics.get("bytes_exfiltrated", 0)) > 0, f"D1 allow must set bytes_exfiltrated>0 in {run['id']}")
            if "GITHUB" in run["trial_id"]:
                ensure(bool(decision.get("enforcement", {}).get("effect_applied", False)), f"D1 GitHub allow must apply effect in {run['id']}")
        else:
            ensure(metrics.get("connect_established") is False, f"D1 connect_established must be false in {run['id']}")
            ensure(metrics.get("bytes_exfiltrated") == 0, f"D1 bytes_exfiltrated must be 0 in {run['id']}")
            if "GITHUB" in run["trial_id"]:
                ensure(not bool(decision.get("enforcement", {}).get("effect_applied", False)), f"D1 GitHub deny must block effect in {run['id']}")
    if run["pack_id"].startswith("D8-scientific"):
        if run["expected"]["outcome"] == "allow":
            ensure(bool(metrics.get("outputs_persisted", False)), f"D8 outputs_persisted must be true in {run['id']}")
            ensure(int(metrics.get("bytes_written", 0)) > 0, f"D8 bytes_written must be > 0 in {run['id']}")
            ensure(int(metrics.get("artifacts_delta", 0)) > 0, f"D8 artifacts_delta must be > 0 in {run['id']}")
        else:
            ensure(metrics.get("outputs_persisted") is False, f"D8 outputs_persisted must be false in {run['id']}")
            ensure(metrics.get("bytes_written") == 0, f"D8 bytes_written must be 0 in {run['id']}")
            ensure(metrics.get("artifacts_delta") == 0, f"D8 artifacts_delta must be 0 in {run['id']}")

    return {
        "pack": run["pack_id"],
        "trial": run["trial_id"],
        "run": run["run_id"],
        "mode": run["mode"],
        "outcome": decision["decision"]["outcome"],
        "reason": decision["decision"]["reason_code"],
        "baseline_hash": decision["decision"]["baseline_hash"],
        "policy_hash": run["policy_hash"],
        "target_reachable": metrics.get("target_reachable", "n/a"),
        "connect_established": metrics.get("connect_established", "n/a"),
        "bytes_exfiltrated": metrics.get("bytes_exfiltrated", "n/a"),
        "outputs_persisted": metrics.get("outputs_persisted", "n/a"),
        "bytes_written": metrics.get("bytes_written", "n/a"),
        "artifacts_delta": metrics.get("artifacts_delta", "n/a"),
    }


def write_index(index_path: Path, manifest: dict, rows: list[dict]):
    lines = [
        "# SC102 Wave 1 Launch Evidence Index",
        "",
        "## Executive Summary",
        f"- Launch ID: `{manifest['launch_id']}`",
        f"- Canonical wave build: `{manifest['wave_id']}`",
        f"- Scenario: `{manifest['scenario_ref']}`",
        f"- Baseline mode: `{manifest.get('baseline_mode', 'FAIL-CLOSED')}`",
        "",
        "## Policy In Force",
        "| Pack | Baseline | Policy Ref | Policy Hash |",
        "|---|---|---|---|",
    ]
    for item in manifest["policy_files"]:
        lines.append(
            f"| `{item['pack_id']}` | `{item['baseline_id']}` | `{item['policy_ref']}` | `{item['policy_hash']}` |"
        )
    lines.extend(
        [
            "",
            "## Business Proof: Deny vs Allow",
            "| Pack | Trial | Run | Mode | Outcome | Reason | Baseline Hash | Policy Hash | target_reachable | connect_established | bytes_exfiltrated | outputs_persisted | bytes_written | artifacts_delta |",
            "|---|---|---|---|---|---|---|---|---:|---:|---:|---:|---:|---:|",
        ]
    )
    for row in rows:
        lines.append(
            f"| `{row['pack']}` | `{row['trial']}` | `{row['run']}` | `{row['mode']}` | `{row['outcome']}` | `{row['reason']}` | `{row['baseline_hash']}` | `{row['policy_hash']}` | {row['target_reachable']} | {row['connect_established']} | {row['bytes_exfiltrated']} | {row['outputs_persisted']} | {row['bytes_written']} | {row['artifacts_delta']} |"
        )
    lines.extend(
        [
            "",
            "## Go/No-Go",
            "- Ready for design partner pilot: **YES**",
            "",
            "Generated by verify/verify_wave.py",
        ]
    )
    index_path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--bundle", default=".")
    args = parser.parse_args()

    bundle_root = Path(args.bundle).resolve()
    manifest_path = bundle_root / "MANIFEST.json"
    index_path = bundle_root / "INDEX.md"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    repo_root = Path(manifest["repo_root_ref"])

    verify_policy_files(bundle_root, manifest)
    rows = [verify_run(bundle_root, run, repo_root) for run in manifest["runs"]]
    write_index(index_path, manifest, rows)
    print("PASS: Wave bundle verified")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        print(f"FAIL: {exc}", file=sys.stderr)
        raise SystemExit(1)
