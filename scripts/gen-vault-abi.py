#!/usr/bin/env python3
import argparse
import json
import os
import sys
from datetime import datetime, timezone

TYPE_SIZES = {
    "u8": 1,
    "u16": 2,
    "u32": 4,
    "u64": 8,
}


def die(msg: str) -> None:
    print(f"ERROR: {msg}", file=sys.stderr)
    sys.exit(1)


def load_spec(path: str) -> dict:
    try:
        with open(path, "r", encoding="utf-8") as f:
            return json.load(f)
    except FileNotFoundError:
        die(f"spec not found: {path}")
    except json.JSONDecodeError as e:
        die(f"invalid json: {e}")


def validate_header(header: dict) -> None:
    fields = header.get("fields", [])
    if not fields:
        die("header.fields is empty")

    seen_offsets = set()
    last_offset = -1
    for field in fields:
        name = field.get("name")
        ftype = field.get("type")
        offset = field.get("offset")

        if name is None or ftype is None or offset is None:
            die("each field must include name, type, offset")
        if ftype not in TYPE_SIZES:
            die(f"unknown type: {ftype}")
        if not isinstance(offset, int):
            die(f"offset for {name} must be int")
        if offset in seen_offsets:
            die(f"duplicate offset {offset} for field {name}")
        if offset <= last_offset:
            die(f"offsets out of order at field {name}: {offset} <= {last_offset}")

        seen_offsets.add(offset)
        last_offset = offset

    last_field = fields[-1]
    last_end = last_field["offset"] + TYPE_SIZES[last_field["type"]]
    header_size = header.get("size")
    if not isinstance(header_size, int):
        die("header.size must be int")
    if header_size < last_end:
        die(f"header.size {header_size} < last field end {last_end}")


def ensure_dir(path: str) -> None:
    os.makedirs(path, exist_ok=True)


def write_header(spec: dict, out_path: str) -> None:
    header = spec["header"]
    fields = header["fields"]
    timestamp = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    lines = []
    lines.append("/* AUTO-GENERATED: law/specs/vault/vault_abi.json */")
    lines.append(f"/* Generated: {timestamp} */")
    lines.append("#ifndef YAI_VAULT_ABI_H")
    lines.append("#define YAI_VAULT_ABI_H")
    lines.append("")
    lines.append(f"#define YAI_VAULT_ABI_VERSION {spec['version']}")
    lines.append(f"#define YAI_VAULT_LAYOUT_BYTES {spec['layout_bytes']}")
    lines.append(f"#define YAI_VAULT_HEADER_SIZE {header['size']}")
    lines.append("")

    for field in fields:
        name = field["name"].upper()
        lines.append(f"#define YAI_VAULT_OFF_{name} {field['offset']}")

    lines.append("")
    lines.append("#endif")

    ensure_dir(os.path.dirname(out_path))
    with open(out_path, "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")


def write_tla(spec: dict, out_path: str) -> None:
    header = spec["header"]
    fields = header["fields"]
    timestamp = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    lines = []
    lines.append("\\* AUTO-GENERATED: law/specs/vault/vault_abi.json")
    lines.append(f"\\* Generated: {timestamp}")
    lines.append("---- MODULE LAW_IDS ----")
    lines.append(f"VaultAbiVersion == {spec['version']}")
    lines.append(f"VaultLayoutBytes == {spec['layout_bytes']}")
    lines.append(f"VaultHeaderSize == {header['size']}")
    for field in fields:
        name = field["name"]
        # CamelCase-ish: Off + FieldName with underscores -> words
        const = "Off" + "".join([w.capitalize() for w in name.split("_")])
        lines.append(f"{const} == {field['offset']}")
    lines.append("====")

    ensure_dir(os.path.dirname(out_path))
    with open(out_path, "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate vault ABI header + TLA constants")
    parser.add_argument(
        "--spec",
        default=os.path.join("law", "specs", "vault", "vault_abi.json"),
        help="path to vault_abi.json",
    )
    parser.add_argument(
        "--out-dir",
        default=None,
        help="optional output root directory (mirrors law/specs and law/formal)",
    )
    args = parser.parse_args()

    spec = load_spec(args.spec)
    if "header" not in spec:
        die("spec missing header")
    validate_header(spec["header"])

    if args.out_dir:
        base = args.out_dir
        header_path = os.path.join(base, "law", "specs", "vault", "yai_vault_abi.h")
        tla_path = os.path.join(base, "law", "formal", "law_ids.tla")
    else:
        header_path = os.path.join("law", "specs", "vault", "yai_vault_abi.h")
        tla_path = os.path.join("law", "formal", "law_ids.tla")

    write_header(spec, header_path)
    write_tla(spec, tla_path)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
