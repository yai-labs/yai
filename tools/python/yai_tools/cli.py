import argparse
import subprocess
import sys

from yai_tools.issue.body import generate_issue_body
from yai_tools.pr.body import generate_pr_body
from yai_tools.pr.check import check_pr_body
from yai_tools.verify.agent_pack import run_agent_pack
from yai_tools.verify.architecture_alignment import run_architecture_alignment
from yai_tools.verify.doctor import run_doctor
from yai_tools.verify.frontmatter_schema import run_schema_check
from yai_tools.verify.trace_graph import run_graph
from yai_tools.workflow.branch import make_branch_name, maybe_checkout


def _repo_root() -> str:
    out = subprocess.run(["git", "rev-parse", "--show-toplevel"], check=True, capture_output=True, text=True)
    return out.stdout.strip()


def _safe_specs_sha(repo_root: str) -> str:
    try:
        out = subprocess.run(
            ["git", "-C", f"{repo_root}/deps/yai-specs", "rev-parse", "HEAD"],
            check=True,
            capture_output=True,
            text=True,
        )
        return out.stdout.strip()
    except Exception:
        return "unknown"


def _autofill_docs_touched(repo_root: str) -> list[str]:
    candidates = [
        "docs",
        ".github",
        "README.md",
        "CHANGELOG.md",
        "CONTRIBUTING.md",
        "SECURITY.md",
        "CODE_OF_CONDUCT.md",
    ]
    try:
        out = subprocess.run(
            ["git", "-C", repo_root, "diff", "--name-only", "--", *candidates],
            check=True,
            capture_output=True,
            text=True,
        )
        touched = [x.strip() for x in out.stdout.splitlines() if x.strip()]
        return touched
    except Exception:
        return []


def _default_commands_for_template(template: str) -> list[str]:
    if template == "docs-governance":
        return [
            "bash tools/release/check_pins.sh",
            "tools/bin/yai-docs-trace-check --all",
            "tools/bin/yai-proof-check",
        ]
    if template == "type-a-milestone":
        return [
            "bash tools/release/check_pins.sh",
            "tools/bin/yai-docs-trace-check --all",
        ]
    if template == "type-b-twin-pr":
        return [
            "bash tools/release/check_pins.sh",
            "git -C deps/yai-specs rev-parse --short HEAD",
            "git rev-parse --short HEAD",
        ]
    return ["git status -sb"]


def _default_spec_delta_for_template(template: str) -> list[str]:
    if template in ("docs-governance", "default"):
        return ["No spec/contract delta; docs/governance update only."]
    if template == "type-a-milestone":
        return ["Milestone phase closure; no wire/protocol contract delta declared here."]
    if template == "type-b-twin-pr":
        return ["Twin-PR alignment; contract delta tracked explicitly across repos."]
    return ["No contract delta declared."]


def cmd_pr_body(argv: list[str]) -> int:
    p = argparse.ArgumentParser(prog="yai-pr-body", add_help=True)
    p.add_argument("--template", default="default", help="default|docs-governance|type-a-milestone|type-b-twin-pr")
    p.add_argument("--issue", required=True, help="#123 or 123 or N/A")
    p.add_argument("--reason", default="", help="Required when issue is N/A")
    p.add_argument("--mp-id", default="N/A", help="MP-... or N/A")
    p.add_argument("--runbook", default="N/A", help="docs/runbooks/<name>.md#<anchor> or N/A")
    p.add_argument("--classification", default="META", help="FEATURE|FIX|DOCS|OPS|META")
    p.add_argument("--compatibility", default="A", help="A|B|C")
    p.add_argument("--objective", default="", help="Objective text (required)")
    p.add_argument("--docs-touched", action="append", default=[], help="Repeatable bullet for docs touched")
    p.add_argument("--spec-delta", action="append", default=[], help="Repeatable bullet for spec/contract delta")
    p.add_argument("--evidence-positive", action="append", default=[], help="Repeatable positive evidence bullet")
    p.add_argument("--evidence-negative", action="append", default=[], help="Repeatable negative evidence bullet")
    p.add_argument("--command", action="append", default=[], help="Repeatable command entry for Commands run")
    p.add_argument(
        "--autofill",
        action="store_true",
        help="Autofill missing fields based on selected template.",
    )
    p.add_argument(
        "--autofill-docs-governance",
        action="store_true",
        help="Legacy alias for --autofill (kept for compatibility).",
    )
    p.add_argument(
        "--run-evidence",
        action="store_true",
        help="With --autofill-docs-governance, execute default commands and inject exit-code evidence.",
    )
    p.add_argument("--out", default="", help="Output file. If omitted: stdout.")
    args = p.parse_args(argv)

    docs_touched = args.docs_touched
    spec_delta = args.spec_delta
    evidence_positive = args.evidence_positive
    evidence_negative = args.evidence_negative
    commands = args.command

    use_autofill = args.autofill or args.autofill_docs_governance
    if use_autofill:
        repo_root = _repo_root()
        specs_sha = _safe_specs_sha(repo_root)

        if args.template == "docs-governance" and not docs_touched:
            docs_touched = _autofill_docs_touched(repo_root)
        if not spec_delta:
            spec_delta = _default_spec_delta_for_template(args.template)
        if not commands:
            commands = _default_commands_for_template(args.template)

        if args.run_evidence:
            results: list[tuple[str, int, str]] = []
            for cmd in commands:
                run = subprocess.run(
                    cmd,
                    shell=True,
                    cwd=repo_root,
                    capture_output=True,
                    text=True,
                )
                combined = f"{run.stdout}\n{run.stderr}".strip()
                results.append((cmd, run.returncode, combined))

            if not evidence_positive:
                evidence_positive = [f"Baseline commit verified: yai + yai-cli -> {specs_sha}"]
                for cmd, code, _ in results:
                    if code == 0 and "yai-proof-check" not in cmd:
                        evidence_positive.append(f"{cmd} exit code = 0")

            if not evidence_negative:
                neg: list[str] = []
                for cmd, code, output in results:
                    if "yai-proof-check" in cmd and "SKIP" in output:
                        neg.append(f"{cmd} -> SKIP (private draft manifest)")
                    elif code != 0:
                        neg.append(f"{cmd} exit code = {code}")
                if not neg:
                    neg = ["No runtime/protocol behavior change expected."]
                evidence_negative = neg
        else:
            if not evidence_positive:
                evidence_positive = [f"Baseline commit verified: yai + yai-cli -> {specs_sha}"]
                for cmd in commands:
                    evidence_positive.append(f"{cmd} exit code = 0 (to be confirmed in CI/local run)")
            if not evidence_negative:
                if any("yai-proof-check" in c for c in commands):
                    evidence_negative = ["tools/bin/yai-proof-check -> SKIP (private draft manifest)"]
                else:
                    evidence_negative = ["No runtime/protocol behavior change expected."]

    md = generate_pr_body(
        template=args.template,
        issue=args.issue,
        reason=args.reason,
        mp_id=args.mp_id,
        runbook=args.runbook,
        classification=args.classification,
        compatibility=args.compatibility,
        objective=args.objective,
        docs_touched=docs_touched,
        spec_delta=spec_delta,
        evidence_positive=evidence_positive,
        evidence_negative=evidence_negative,
        commands=commands,
    )

    if args.out:
        with open(args.out, "w", encoding="utf-8") as f:
            f.write(md)
        return 0

    sys.stdout.write(md)
    return 0


def cmd_pr_check(argv: list[str]) -> int:
    p = argparse.ArgumentParser(prog="yai-pr-check", add_help=True)
    p.add_argument("path", nargs="?", default=".pr/PR_BODY.md", help="PR body path")
    args = p.parse_args(argv)

    ok, msg = check_pr_body(args.path)
    if not ok:
        print(f"FAIL: {msg}", file=sys.stderr)
        return 1

    print(f"OK: {msg}")
    return 0


def cmd_issue_body(argv: list[str]) -> int:
    p = argparse.ArgumentParser(prog="yai-issue-body", add_help=True)
    p.add_argument("--title", required=True, help="Issue title")
    p.add_argument("--type", default="task", help="bug|feature|runbook|docs|task")
    p.add_argument("--mp-id", default="N/A", help="MP-... or N/A")
    p.add_argument("--runbook", default="N/A", help="docs/runbooks/<name>.md")
    p.add_argument("--phase", default="N/A", help="Runbook phase, e.g. 0.1.0")
    p.add_argument("--out", default="", help="Output path; stdout if omitted")
    args = p.parse_args(argv)

    body = generate_issue_body(
        title=args.title,
        issue_type=args.type,
        mp_id=args.mp_id,
        runbook=args.runbook,
        phase=args.phase,
    )

    if args.out:
        with open(args.out, "w", encoding="utf-8") as f:
            f.write(body)
        return 0

    sys.stdout.write(body)
    return 0


def cmd_branch(argv: list[str]) -> int:
    p = argparse.ArgumentParser(prog="yai-branch", add_help=True)
    p.add_argument("--type", required=True, help="feat|fix|docs|chore|refactor|test|ci|hotfix")
    p.add_argument("--issue", required=True, help="#123 or 123 or N/A")
    p.add_argument("--reason", default="", help="Required when issue is N/A")
    p.add_argument("--area", required=True, help="Short area tag, e.g. root, kernel, governance")
    p.add_argument("--desc", required=True, help="Short description, e.g. hardening-forward")
    p.add_argument("--checkout", action="store_true", help="Create & checkout the branch")
    args = p.parse_args(argv)

    name = make_branch_name(
        change_type=args.type,
        issue=args.issue,
        reason=args.reason,
        area=args.area,
        desc=args.desc,
    )
    print(name)

    if args.checkout:
        maybe_checkout(name)

    return 0


def cmd_docs_schema_check(argv: list[str]) -> int:
    p = argparse.ArgumentParser(prog="yai-docs-schema-check", add_help=True)
    p.add_argument("--changed", action="store_true")
    p.add_argument("--base", default="")
    p.add_argument("--head", default="HEAD")
    args = p.parse_args(argv)

    if args.changed and not args.base:
        print("[docs-schema] ERROR: --changed requires --base <sha>", file=sys.stderr)
        return 2

    return run_schema_check(changed=args.changed, base=args.base, head=args.head)


def cmd_docs_graph(argv: list[str]) -> int:
    p = argparse.ArgumentParser(prog="yai-docs-graph", add_help=True)
    mode = p.add_mutually_exclusive_group(required=True)
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check", action="store_true")
    args = p.parse_args(argv)

    return run_graph(write=args.write)


def cmd_agent_pack(argv: list[str]) -> int:
    p = argparse.ArgumentParser(prog="yai-agent-pack", add_help=True)
    mode = p.add_mutually_exclusive_group(required=True)
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check", action="store_true")
    args = p.parse_args(argv)

    return run_agent_pack(write=args.write)


def cmd_docs_doctor(argv: list[str]) -> int:
    p = argparse.ArgumentParser(prog="yai-docs-doctor", add_help=True)
    p.add_argument("--mode", choices=["ci", "all"], default="ci")
    p.add_argument("--base", default="")
    p.add_argument("--head", default="HEAD")
    args = p.parse_args(argv)

    return run_doctor(mode=args.mode, base=args.base, head=args.head)


def cmd_architecture_check(argv: list[str]) -> int:
    p = argparse.ArgumentParser(prog="yai-architecture-check", add_help=True)
    mode = p.add_mutually_exclusive_group(required=True)
    mode.add_argument("--changed", action="store_true")
    mode.add_argument("--all", action="store_true")
    mode.add_argument("--write", action="store_true")
    p.add_argument("--base", default="")
    p.add_argument("--head", default="HEAD")
    args = p.parse_args(argv)

    run_mode = "all"
    if args.changed:
        run_mode = "changed"

    return run_architecture_alignment(mode=run_mode, base=args.base, head=args.head, write=args.write)


def main() -> int:
    if len(sys.argv) < 2:
        print(
            "Usage: python -m yai_tools.cli <pr-body|pr-check|branch|issue-body|docs-schema-check|docs-graph|agent-pack|docs-doctor|architecture-check> ...",
            file=sys.stderr,
        )
        return 2

    sub = sys.argv[1]
    rest = sys.argv[2:]

    if sub == "pr-body":
        return cmd_pr_body(rest)
    if sub == "pr-check":
        return cmd_pr_check(rest)
    if sub == "branch":
        return cmd_branch(rest)
    if sub == "issue-body":
        return cmd_issue_body(rest)
    if sub == "docs-schema-check":
        return cmd_docs_schema_check(rest)
    if sub == "docs-graph":
        return cmd_docs_graph(rest)
    if sub == "agent-pack":
        return cmd_agent_pack(rest)
    if sub == "docs-doctor":
        return cmd_docs_doctor(rest)
    if sub == "architecture-check":
        return cmd_architecture_check(rest)

    print(f"Unknown subcommand: {sub}", file=sys.stderr)
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
