#!/usr/bin/env python3
import json
import sys
from pathlib import Path


def read_json(path: Path, default=None):
    if default is None:
        default = {}
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except Exception:
        return default


def read_jsonl(path: Path):
    rows = []
    try:
        for line in path.read_text(encoding="utf-8").splitlines():
            line = line.strip()
            if line:
                rows.append(json.loads(line))
    except Exception:
        pass
    return rows


def write_json(path: Path, payload):
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def fmt(v):
    if isinstance(v, bool):
        return "true" if v else "false"
    return str(v)


def main():
    if len(sys.argv) < 3:
        raise SystemExit("usage: render_human_evidence.py <evidence_dir> <trial_id>")

    evidence_dir = Path(sys.argv[1])
    trial_id = sys.argv[2]

    decisions = read_jsonl(evidence_dir / "decision_records.jsonl")
    timeline = read_jsonl(evidence_dir / "timeline.jsonl")
    metrics = read_json(evidence_dir / "containment_metrics.json")
    baseline = read_json(evidence_dir / "baseline.json")

    write_json(evidence_dir / "decision_records.pretty.json", decisions)
    write_json(evidence_dir / "timeline.pretty.json", timeline)

    primary = decisions[0] if decisions else {}
    decision = primary.get("decision", {})
    enforcement = primary.get("enforcement", {})
    principal = primary.get("principal", {})

    kpis = [
        ("decision_outcome", metrics.get("decision_outcome", decision.get("outcome", "n/a"))),
        ("reason_code", metrics.get("reason_code", decision.get("reason_code", "n/a"))),
        ("connect_established", metrics.get("connect_established", "n/a")),
        ("bytes_exfiltrated", metrics.get("bytes_exfiltrated", "n/a")),
        ("local_target_reachable", metrics.get("local_target_reachable", "n/a")),
        ("time_to_contain_ms", metrics.get("time_to_contain_ms", "n/a")),
        ("forbidden_effect_success_rate", metrics.get("forbidden_effect_success_rate", "n/a")),
        ("forbidden_effect_block_rate", metrics.get("forbidden_effect_block_rate", "n/a")),
        ("containment_success_rate", metrics.get("containment_success_rate", "n/a")),
    ]

    kpi_lines = [
        "# KPI Summary",
        "",
        "| KPI | Value |",
        "|---|---:|",
    ]
    for k, v in kpis:
        kpi_lines.append(f"| {k} | {fmt(v)} |")
    kpi_lines.append("")
    (evidence_dir / "KPI_SUMMARY.md").write_text("\n".join(kpi_lines), encoding="utf-8")

    summary_lines = [
        "# Evidence Executive Summary",
        "",
        "## Identity",
        f"- Trial: `{trial_id}`",
        f"- Run: `{evidence_dir.name}`",
        f"- Domain pack: `{primary.get('domain_pack_id', 'D1-digital/egress-v1')}`",
        f"- Baseline ID: `{baseline.get('baseline_id', 'n/a')}`",
        f"- Baseline Hash: `{baseline.get('baseline_hash', 'n/a')}`",
        f"- Contract Ref: `{baseline.get('contract_ref', 'n/a')}`",
        "",
        "## Decision",
        f"- Outcome: `{decision.get('outcome', 'n/a')}`",
        f"- Reason code: `{decision.get('reason_code', 'n/a')}`",
        f"- Enforcement result: `{enforcement.get('result', 'n/a')}`",
        "",
        "## Traceability",
        f"- ws_id: `{primary.get('ws_id', 'n/a')}`",
        f"- trace_id: `{primary.get('trace_id', 'n/a')}`",
        f"- principal.id: `{principal.get('id', 'n/a')}`",
        "",
        "## KPI Highlights",
        f"- local_target_reachable: `{fmt(metrics.get('local_target_reachable', 'n/a'))}`",
        f"- connect_established: `{fmt(metrics.get('connect_established', 'n/a'))}`",
        f"- bytes_exfiltrated: `{fmt(metrics.get('bytes_exfiltrated', 'n/a'))}`",
        f"- time_to_contain_ms: `{fmt(metrics.get('time_to_contain_ms', 'n/a'))}`",
        f"- forbidden_effect_block_rate: `{fmt(metrics.get('forbidden_effect_block_rate', 'n/a'))}`",
        f"- containment_success_rate: `{fmt(metrics.get('containment_success_rate', 'n/a'))}`",
        "",
        "## Artifact Set",
        "- `baseline.json`",
        "- `decision_records.jsonl`",
        "- `decision_records.pretty.json`",
        "- `timeline.jsonl`",
        "- `timeline.pretty.json`",
        "- `containment_metrics.json`",
        "- `KPI_SUMMARY.md`",
        "- `system_state.txt`",
    ]
    (evidence_dir / "EVIDENCE_SUMMARY.md").write_text("\n".join(summary_lines) + "\n", encoding="utf-8")

    idx = evidence_dir / "EVIDENCE_INDEX.md"
    if idx.exists():
        txt = idx.read_text(encoding="utf-8")
        marker = "## Human-readable Views"
        if marker not in txt:
            txt = txt.rstrip() + "\n\n" + "\n".join([
                marker,
                "- `EVIDENCE_SUMMARY.md`",
                "- `KPI_SUMMARY.md`",
                "- `decision_records.pretty.json`",
                "- `timeline.pretty.json`",
            ]) + "\n"
            idx.write_text(txt, encoding="utf-8")


if __name__ == "__main__":
    main()
