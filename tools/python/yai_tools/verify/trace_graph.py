from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from yai_tools.verify.generated_sync import check_json_synced, write_json
from yai_tools.verify.traceability import ADR_DIR, MP_DIR, REPO_ROOT, RUNBOOK_DIR, parse_frontmatter

PROPOSAL_DIR = REPO_ROOT / "docs" / "20-governance" / "design" / "proposals"
GENERATED_GRAPH = REPO_ROOT / "docs" / "_generated" / "traceability.graph.v1.json"
GENERATED_LOCK = REPO_ROOT / "docs" / "_generated" / "traceability.lock.v1.json"


def _node_type(p: Path) -> str:
    if p.is_relative_to(PROPOSAL_DIR):
        return "proposal"
    if p.is_relative_to(ADR_DIR):
        return "adr"
    if p.is_relative_to(RUNBOOK_DIR):
        return "runbook"
    if p.is_relative_to(MP_DIR):
        return "milestone_pack"
    if p.is_relative_to(REPO_ROOT / "docs" / "50-qualification" / "test-plans"):
        return "test_plan"
    if p.is_relative_to(REPO_ROOT / "docs" / "60-validation" / "proof"):
        return "proof_pack"
    return "doc"


def _list(v: Any) -> list[str]:
    if isinstance(v, list):
        return [str(x) for x in v]
    if isinstance(v, str):
        return [v]
    return []


def _refs_from_frontmatter(p: Path) -> list[str]:
    txt = p.read_text(encoding="utf-8")
    fm = parse_frontmatter(txt)
    out: list[str] = []
    t = _node_type(p)
    if t == "proposal":
        out += _list(fm.get("adr"))
        out += _list(fm.get("runbooks"))
        out += _list(fm.get("milestone_packs"))
    elif t == "adr":
        if isinstance(fm.get("runbook"), str):
            out.append(str(fm.get("runbook")))
    elif t == "runbook":
        out += _list(fm.get("adr_refs"))
    elif t == "milestone_pack":
        if isinstance(fm.get("runbook"), str):
            out.append(str(fm.get("runbook")))
        out += _list(fm.get("adrs"))
        out += _list(fm.get("spec_anchors"))
    return sorted(set([r for r in out if r.endswith(".md")]))


def build_graph() -> dict[str, Any]:
    docs = sorted(
        list(PROPOSAL_DIR.rglob("*.md"))
        + list(ADR_DIR.rglob("*.md"))
        + list(RUNBOOK_DIR.rglob("*.md"))
        + list(MP_DIR.rglob("*.md"))
    )

    nodes: list[dict[str, Any]] = []
    edges: list[dict[str, Any]] = []
    violations: list[str] = []

    node_set: set[str] = set()
    for p in docs:
        rp = p.relative_to(REPO_ROOT).as_posix()
        node_set.add(rp)
        nodes.append({"id": rp, "type": _node_type(p), "path": rp})

    for p in docs:
        src = p.relative_to(REPO_ROOT).as_posix()
        src_type = _node_type(p)
        refs = _refs_from_frontmatter(p)
        for ref in refs:
            dstp = REPO_ROOT / ref
            if not dstp.exists():
                violations.append(f"broken link: {src} -> {ref}")
                continue
            dst_type = _node_type(dstp)
            relation = f"{src_type}_to_{dst_type}"
            edges.append({"from": src, "to": ref, "relation": relation})
            if ref not in node_set and ref.startswith("docs/"):
                nodes.append({"id": ref, "type": dst_type, "path": ref})
                node_set.add(ref)

    incident: set[str] = set()
    for e in edges:
        incident.add(e["from"])
        incident.add(e["to"])

    orphans = sorted(n["id"] for n in nodes if n["type"] in {"proposal", "adr", "runbook", "milestone_pack"} and n["id"] not in incident)

    graph = {
        "version": 1,
        "nodes": sorted(nodes, key=lambda x: x["id"]),
        "edges": sorted(edges, key=lambda x: (x["from"], x["to"], x["relation"])),
        "orphans": orphans,
        "violations": sorted(set(violations)),
    }

    return graph


def run_graph(write: bool) -> int:
    graph = build_graph()
    lock = {
        "version": 1,
        "node_count": len(graph["nodes"]),
        "edge_count": len(graph["edges"]),
        "violation_count": len(graph["violations"]),
        "orphan_count": len(graph["orphans"]),
    }

    if graph["violations"]:
        print("[docs-graph] FAIL:")
        for v in graph["violations"]:
            print(f"- {v}")
        return 1

    if write:
        write_json(GENERATED_GRAPH, graph)
        write_json(GENERATED_LOCK, lock)
        print("[docs-graph] OK: generated graph and lock updated")
        return 0

    ok_graph, msg_graph = check_json_synced(GENERATED_GRAPH, graph)
    ok_lock, msg_lock = check_json_synced(GENERATED_LOCK, lock)
    if not ok_graph or not ok_lock:
        print("[docs-graph] FAIL:")
        if not ok_graph:
            print(f"- {msg_graph}")
        if not ok_lock:
            print(f"- {msg_lock}")
        return 1

    print("[docs-graph] OK")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(prog="yai-docs-graph")
    mode = ap.add_mutually_exclusive_group(required=True)
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check", action="store_true")
    args = ap.parse_args()
    return run_graph(write=args.write)


if __name__ == "__main__":
    raise SystemExit(main())
