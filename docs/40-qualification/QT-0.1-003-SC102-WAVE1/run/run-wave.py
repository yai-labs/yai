#!/usr/bin/env python3
import argparse
import datetime as dt
import hashlib
import json
import os
import platform
import shutil
import subprocess
import sys
import textwrap
from pathlib import Path

try:
    import yaml  # type: ignore
except Exception:
    yaml = None

REQUIRED_EVIDENCE_FILES = [
    "baseline.json",
    "timeline.jsonl",
    "decision_records.jsonl",
    "containment_metrics.json",
    "system_state.txt",
    "EVIDENCE_INDEX.md",
]


def ensure(cond: bool, msg: str):
    if not cond:
        raise RuntimeError(msg)


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    return sha256_bytes(path.read_bytes())


def run_cmd(
    cmd: list[str],
    cwd: Path,
    env: dict | None = None,
    log_path: Path | None = None,
    timeout_s: int | None = None,
) -> tuple[int, str]:
    final_env = os.environ.copy()
    if env:
        final_env.update(env)

    if log_path:
        log_path.parent.mkdir(parents=True, exist_ok=True)
        with log_path.open("w", encoding="utf-8") as fh:
            try:
                proc = subprocess.run(
                    cmd,
                    cwd=str(cwd),
                    env=final_env,
                    stdout=fh,
                    stderr=subprocess.STDOUT,
                    text=True,
                    timeout=timeout_s,
                )
            except subprocess.TimeoutExpired:
                fh.write(f"\n[wave] TIMEOUT: command exceeded {timeout_s}s\n")
                fh.flush()
                return 124, log_path.read_text(encoding="utf-8", errors="replace")
        return proc.returncode, log_path.read_text(encoding="utf-8", errors="replace")

    try:
        proc = subprocess.run(
            cmd,
            cwd=str(cwd),
            env=final_env,
            capture_output=True,
            text=True,
            timeout=timeout_s,
        )
    except subprocess.TimeoutExpired:
        return 124, f"[wave] TIMEOUT: command exceeded {timeout_s}s\n"
    return proc.returncode, proc.stdout + proc.stderr


def git_value(repo: Path, args: list[str], default: str = "unknown") -> str:
    try:
        out = subprocess.check_output(["git", *args], cwd=str(repo), stderr=subprocess.DEVNULL, text=True).strip()
        return out if out else default
    except Exception:
        return default


def load_wave_config(path: Path) -> dict:
    raw = path.read_text(encoding="utf-8")
    if yaml is not None:
        data = yaml.safe_load(raw)
        ensure(isinstance(data, dict), "wave config must be a mapping")
        return data
    data = json.loads(raw)
    ensure(isinstance(data, dict), "wave config must be a mapping")
    return data


def build_release_identity(repo_root: Path) -> dict:
    yai_sha = git_value(repo_root, ["rev-parse", "HEAD"])
    yai_short = yai_sha[:7] if yai_sha != "unknown" else "unknown"
    yai_cli_repo = (repo_root / ".." / "yai-cli").resolve()
    yai_cli_sha = (
        git_value(yai_cli_repo, ["rev-parse", "HEAD"])
        if yai_cli_repo.exists() and (yai_cli_repo / ".git").exists()
        else "unknown"
    )
    yai_law_repo = repo_root / "deps" / "yai-law"
    specs_pin_sha = git_value(yai_law_repo, ["rev-parse", "HEAD"]) if yai_law_repo.exists() else "unknown"
    tag = git_value(repo_root, ["describe", "--tags", "--exact-match"], default="")
    version_label = tag if tag else f"dev-{yai_short}"
    return {
        "yai_git_sha": yai_sha,
        "yai_cli_git_sha": yai_cli_sha,
        "specs_pin_sha": specs_pin_sha,
        "version_label": version_label,
        "build_mode": "build",
        "reproduce": [
            "cd docs/40-qualification/QT-0.1-003-SC102-WAVE1",
            "./run/run-wave.sh",
            "cd docs/40-qualification/WAVES/SC102-WAVE1-LAUNCH",
            "./verify/verify.sh",
        ],
    }


def canonical_wave_id(cfg: dict, arg_wave_id: str, now: dt.datetime, repo_root: Path) -> str:
    if arg_wave_id:
        return arg_wave_id
    base_wave_id = cfg.get("id", "WAVE")
    if cfg.get("release_ref_mode", "auto") == "manual":
        return str(base_wave_id)
    git_short = git_value(repo_root, ["rev-parse", "--short", "HEAD"], default="nogit")
    return f"{base_wave_id}-{now.date().isoformat()}-{git_short}"


def expected_reason_codes(pack_id: str, decision_reason: str) -> list[str]:
    if pack_id.startswith("D8-scientific"):
        return [
            "PARAM_LOCK_MISSING",
            "PARAMS_LOCK_MISSING",
            "PARAMS_LOCK_MISMATCH",
            "PARAMS_LOCK_HASH_MISMATCH",
            "PARAMS_LOCK_INVALID",
            "PARAMS_LOCK_INVALID_SIGNATURE",
            "PARAMS_LOCK_VALID",
        ]
    return [decision_reason] if decision_reason else ["EGRESS_DEST_NOT_CONTRACTED"]


def write_verify_scripts(bundle_dir: Path):
    verify_dir = bundle_dir / "verify"
    verify_dir.mkdir(parents=True, exist_ok=True)

    verify_py = verify_dir / "verify_wave.py"
    verify_sh = verify_dir / "verify.sh"

    verify_py.write_text(
        textwrap.dedent(
            """\
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
    index_path.write_text("\\n".join(lines) + "\\n", encoding="utf-8")


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
"""
        ),
        encoding="utf-8",
    )
    verify_py.chmod(0o755)

    verify_sh.write_text(
        textwrap.dedent(
            """\
#!/usr/bin/env bash
set -euo pipefail
DIR="$(cd "$(dirname "$0")" && pwd)"
python3 "$DIR/verify_wave.py" --bundle "$DIR/.."
"""
        ),
        encoding="utf-8",
    )
    verify_sh.chmod(0o755)

    root_verify = bundle_dir / "verify.sh"
    root_verify.write_text(
        textwrap.dedent(
            """\
#!/usr/bin/env bash
set -euo pipefail
DIR="$(cd "$(dirname "$0")" && pwd)"
"$DIR/verify/verify.sh"
"""
        ),
        encoding="utf-8",
    )
    root_verify.chmod(0o755)


def generate_bundle_readme(bundle_dir: Path, manifest: dict):
    lines = [
        "---",
        f"id: {manifest['launch_id']}",
        "title: SC102 Wave 1 Launch Bundle",
        "status: draft",
        "owner: runtime",
        f"effective_date: {dt.date.today().isoformat()}",
        "revision: 1",
        "---",
        "",
        f"# {manifest['launch_id']}",
        "",
        "SC102 star-case bundle with deterministic verify and policy pinning.",
        "",
        "## Canonical Links",
        "- docs/30-catalog/scenarios/SC-102.md",
        "- docs/40-qualification/QT-0.1-003-SC102-WAVE1/README.md",
        "- docs/30-catalog/domains/packs/D1-digital/egress-v1/README.md",
        "- docs/30-catalog/domains/packs/D8-scientific/reproducibility-parameter-lock-v1/README.md",
        "",
        "## Verify",
        "```bash",
        "./verify/verify.sh",
        "```",
        "",
        "## Structure",
        "- MANIFEST.json",
        "- INDEX.md",
        "- POLICY/",
        "- RUNS/deny/",
        "- RUNS/allow/",
        "",
        "## Release Identity",
        "```json",
        json.dumps(manifest["product_release"], indent=2),
        "```",
    ]
    (bundle_dir / "README.md").write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--wave-id", default="")
    args = parser.parse_args()

    script_dir = Path(__file__).resolve().parent
    qt_dir = script_dir.parent
    repo_root = qt_dir.parents[2]
    cfg_path = qt_dir / "wave" / "wave.yaml"
    lock_path = qt_dir / "wave" / "wave.lock.json"
    cfg = load_wave_config(cfg_path)

    default_timeout_s = int(os.getenv("YAI_WAVE_CMD_TIMEOUT", "1200"))
    now = dt.datetime.now(dt.timezone.utc)
    stamp = now.strftime("%Y%m%dT%H%M%SZ")
    wave_id = canonical_wave_id(cfg, args.wave_id, now, repo_root)
    launch_id = str(cfg.get("launch_id", "SC102-WAVE1-LAUNCH"))
    scenario_id = str(cfg.get("scenario", "SC-102"))

    runtime_root = Path.home() / ".yai" / "qualifications" / scenario_id.replace("-", "") / wave_id / stamp
    runtime_logs = runtime_root / "logs"
    runtime_evidence = runtime_root / "evidence"
    runtime_state = runtime_root / "state"
    runtime_logs.mkdir(parents=True, exist_ok=True)
    release_identity = build_release_identity(repo_root)

    run_records: list[dict] = []
    for idx, item in enumerate(cfg.get("items", []), start=1):
        trial_id = str(item["trial_id"])
        trial_evidence_root = runtime_evidence / trial_id
        trial_state_root = runtime_state / trial_id

        for cmd_idx, run_cfg in enumerate(item.get("run_cmds", []), start=1):
            cmd_path = repo_root / run_cfg["path"]
            ensure(cmd_path.exists(), f"missing run command: {cmd_path}")
            env = {"EVIDENCE_ROOT": str(trial_evidence_root), "STATE_ROOT": str(trial_state_root)}
            env.update({k: str(v) for k, v in run_cfg.get("env", {}).items()})
            log_path = runtime_logs / f"{idx:02d}_{trial_id}_{cmd_idx}.log"
            timeout_s = int(run_cfg.get("timeout_s", default_timeout_s))
            print(f"[wave] run start: {trial_id} cmd#{cmd_idx} timeout={timeout_s}s")
            rc, output = run_cmd([str(cmd_path)], cwd=repo_root, env=env, log_path=log_path, timeout_s=timeout_s)
            print(f"[wave] run end: {trial_id} cmd#{cmd_idx} rc={rc} log={log_path}")
            if rc != 0:
                print(output)
                raise RuntimeError(f"run failed for {trial_id} cmd#{cmd_idx}: {run_cfg['path']}")

        baseline_id = "baseline-deny"
        if item.get("run_cmds"):
            baseline_id = str(item["run_cmds"][0].get("env", {}).get("BASELINE_ID", "baseline-deny"))
        mode = "allow" if baseline_id == "baseline-allow" else "deny"
        for run_id in item.get("select", {}).get("keep", []):
            src_run = trial_evidence_root / item["pack_id"] / run_id
            ensure(src_run.exists(), f"missing selected runtime evidence: {src_run}")
            run_records.append(
                {
                    "trial_id": trial_id,
                    "pack_id": str(item["pack_id"]),
                    "run_id": str(run_id),
                    "src_run": src_run,
                    "trial_ref": str(item["trial_ref"]),
                    "baseline_contract_ref": str(item["baseline_contract_ref"]),
                    "required_fields_ref": str(item["required_fields_ref"]),
                    "baseline_id": baseline_id,
                    "mode": mode,
                }
            )

    waves_root = repo_root / "docs" / "40-qualification" / "WAVES"
    bundle_dir = waves_root / wave_id
    if bundle_dir.exists():
        shutil.rmtree(bundle_dir)
    (bundle_dir / "RUNS" / "deny").mkdir(parents=True, exist_ok=True)
    (bundle_dir / "RUNS" / "allow").mkdir(parents=True, exist_ok=True)
    (bundle_dir / "POLICY").mkdir(parents=True, exist_ok=True)

    policy_index: dict[tuple[str, str], dict] = {}
    for rr in run_records:
        key = (rr["pack_id"], rr["baseline_id"])
        if key in policy_index:
            continue
        src_policy = repo_root / rr["baseline_contract_ref"]
        ensure(src_policy.exists(), f"missing baseline contract: {src_policy}")
        policy_name = f"{rr['pack_id'].replace('/', '--')}--{rr['baseline_id']}.json"
        policy_rel = Path("POLICY") / policy_name
        policy_dst = bundle_dir / policy_rel
        shutil.copy2(src_policy, policy_dst)
        policy_index[key] = {
            "pack_id": rr["pack_id"],
            "baseline_id": rr["baseline_id"],
            "policy_ref": str(policy_rel),
            "policy_hash": sha256_file(policy_dst),
            "baseline_contract_ref": rr["baseline_contract_ref"],
        }

    manifest_runs = []
    for rr in run_records:
        run_key = f"{rr['trial_id']}-{rr['run_id']}"
        dst_rel = Path("RUNS") / rr["mode"] / run_key
        dst_abs = bundle_dir / dst_rel
        shutil.copytree(rr["src_run"], dst_abs)
        decision_file = dst_abs / "decision_records.jsonl"
        lines = [x for x in decision_file.read_text(encoding="utf-8").splitlines() if x.strip()]
        ensure(lines, f"empty decision records in {run_key}")
        decision = json.loads(lines[-1])

        required_hashes = {}
        for fname in REQUIRED_EVIDENCE_FILES:
            fp = dst_abs / fname
            ensure(fp.exists(), f"missing required file in bundle run {run_key}: {fname}")
            required_hashes[fname] = sha256_file(fp)

        policy_meta = policy_index[(rr["pack_id"], rr["baseline_id"])]
        manifest_runs.append(
            {
                "id": run_key,
                "trial_id": rr["trial_id"],
                "pack_id": rr["pack_id"],
                "run_id": rr["run_id"],
                "mode": rr["mode"],
                "baseline_id": rr["baseline_id"],
                "trial_ref": rr["trial_ref"],
                "bundle_run_dir": str(dst_rel),
                "source_runtime_ref": str(rr["src_run"]),
                "baseline_contract_ref": rr["baseline_contract_ref"],
                "required_fields_ref": rr["required_fields_ref"],
                "policy_ref": policy_meta["policy_ref"],
                "policy_hash": policy_meta["policy_hash"],
                "required_file_hashes": required_hashes,
                "expected": {
                    "outcome": "allow" if rr["mode"] == "allow" else "deny",
                    "reason_codes": expected_reason_codes(rr["pack_id"], decision["decision"].get("reason_code", "")),
                },
            }
        )

    policy_files = list(policy_index.values())
    policy_digest = sha256_bytes(
        "\n".join(f"{x['policy_ref']}:{x['policy_hash']}" for x in sorted(policy_files, key=lambda z: z["policy_ref"])).encode("utf-8")
    )
    manifest = {
        "launch_id": launch_id,
        "wave_id": wave_id,
        "scenario_ref": "docs/30-catalog/scenarios/SC-102.md",
        "qt_ref": cfg.get("qt_ref"),
        "selection_policy": cfg.get("selection_policy", "selected-runs-only"),
        "baseline_mode": "FAIL-CLOSED",
        "generated_at": now.isoformat(),
        "repo_root_ref": str(repo_root),
        "runtime_output_root": str(runtime_root),
        "policy_ref": "POLICY",
        "policy_hash": policy_digest,
        "policy_files": sorted(policy_files, key=lambda x: x["policy_ref"]),
        "packs": sorted({r["pack_id"] for r in manifest_runs}),
        "host": {
            "system": platform.system(),
            "release": platform.release(),
            "machine": platform.machine(),
        },
        "product_release": release_identity,
        "runs": manifest_runs,
    }

    (bundle_dir / "MANIFEST.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    write_verify_scripts(bundle_dir)
    generate_bundle_readme(bundle_dir, manifest)
    rc, out = run_cmd([str(bundle_dir / "verify" / "verify.sh")], cwd=bundle_dir)
    if rc != 0:
        print(out)
        raise RuntimeError("bundle verification failed")

    launch_dir = waves_root / launch_id
    if launch_dir.exists():
        shutil.rmtree(launch_dir)
    shutil.copytree(bundle_dir, launch_dir)
    (waves_root / "LATEST").write_text(f"{launch_id}\n", encoding="utf-8")

    lock = {
        "generated": True,
        "launch_id": launch_id,
        "wave_id": wave_id,
        "generated_at": now.isoformat(),
        "config": cfg,
        "runtime_output_root": str(runtime_root),
        "bundle_dir": str(bundle_dir),
        "launch_bundle_dir": str(launch_dir),
        "product_release": release_identity,
    }
    lock_path.write_text(json.dumps(lock, indent=2) + "\n", encoding="utf-8")

    print("PASS: wave run complete")
    print(f"launch_id={launch_id}")
    print(f"wave_id={wave_id}")
    print(f"runtime_output_root={runtime_root}")
    print(f"bundle_dir={bundle_dir}")
    print(f"launch_bundle_dir={launch_dir}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        print(f"FAIL: {exc}", file=sys.stderr)
        raise SystemExit(1)
