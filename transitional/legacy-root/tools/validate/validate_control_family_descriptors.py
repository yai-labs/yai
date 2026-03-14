#!/usr/bin/env python3
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CF = ROOT / "governance" / "families"
IDX = CF / "index"

families_index = json.loads((IDX / "families.index.json").read_text(encoding="utf-8"))
descriptor_index = json.loads((IDX / "families.descriptors.index.json").read_text(encoding="utf-8"))
family_matrix = json.loads((IDX / "family.matrix.v1.json").read_text(encoding="utf-8"))
hierarchy = json.loads((IDX / "family-hierarchy.json").read_text(encoding="utf-8"))

if descriptor_index.get("kind") != "families.descriptors.index":
    raise SystemExit("families.descriptors.index invalid kind")
if family_matrix.get("kind") != "family.matrix.v1":
    raise SystemExit("family.matrix invalid kind")

hier_specs = {e.get("canonical_name"): set(e.get("specializations", [])) for e in hierarchy.get("entries", [])}

entries = descriptor_index.get("entries", [])
if not entries:
    raise SystemExit("families.descriptors.index has no entries")

seen = set()
for e in entries:
    fam = e.get("canonical_name")
    fid = e.get("family_id")
    dref = e.get("descriptor_ref")
    mref = e.get("materialized_manifest_ref")
    if not fam or not fid or not dref or not mref:
        raise SystemExit(f"descriptor index entry missing fields: {e}")
    if fam in seen:
        raise SystemExit(f"duplicate descriptor index family: {fam}")
    seen.add(fam)

    descriptor_path = ROOT / "governance" / dref
    if not descriptor_path.exists():
        raise SystemExit(f"missing descriptor file: {dref}")
    dobj = json.loads(descriptor_path.read_text(encoding="utf-8"))

    if dobj.get("kind") != "control_family_descriptor.v1":
        raise SystemExit(f"invalid descriptor kind for {fam}")
    if dobj.get("canonical_name") != fam:
        raise SystemExit(f"descriptor canonical mismatch for {fam}")
    if dobj.get("family_id") != fid:
        raise SystemExit(f"descriptor family_id mismatch for {fam}")

    spec_a = set(dobj.get("hierarchy", {}).get("specializations", []))
    spec_b = hier_specs.get(fam, set())
    if spec_b and spec_a != spec_b:
        raise SystemExit(f"specialization hierarchy mismatch for {fam}")

    materialized_ref = dobj.get("materialized_manifest_ref")
    if materialized_ref != mref:
        raise SystemExit(f"materialized manifest mismatch for {fam}")
    if not (ROOT / "governance" / materialized_ref).exists():
        raise SystemExit(f"missing materialized manifest for {fam}: {materialized_ref}")

# families.index must now carry descriptor refs as primary metadata anchor.
for e in families_index.get("families", []):
    fam = e.get("canonical_name")
    if not e.get("descriptor_ref"):
        raise SystemExit(f"families.index missing descriptor_ref for {fam}")
    if not e.get("materialized_manifest_ref"):
        raise SystemExit(f"families.index missing materialized_manifest_ref for {fam}")

matrix_map = {e.get("canonical_name"): e for e in family_matrix.get("entries", [])}
for fam in seen:
    if fam not in matrix_map:
        raise SystemExit(f"family.matrix missing {fam}")
    me = matrix_map[fam]
    if not me.get("descriptor_ref"):
        raise SystemExit(f"family.matrix missing descriptor_ref for {fam}")

print("control_family_descriptors: ok")
