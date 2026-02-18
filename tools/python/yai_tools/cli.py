import argparse
import sys

from yai_tools.issue.body import generate_issue_body
from yai_tools.pr.body import generate_pr_body
from yai_tools.pr.check import check_pr_body
from yai_tools.workflow.branch import make_branch_name, maybe_checkout


def cmd_pr_body(argv: list[str]) -> int:
    p = argparse.ArgumentParser(prog="yai-pr-body", add_help=True)
    p.add_argument("--template", default="default", help="default|docs-governance|type-a-milestone|type-b-twin-pr")
    p.add_argument("--issue", required=True, help="#123 or 123 or N/A")
    p.add_argument("--reason", default="", help="Required when issue is N/A")
    p.add_argument("--mp-id", default="N/A", help="MP-... or N/A")
    p.add_argument("--runbook", default="N/A", help="docs/runbooks/<name>.md#<anchor> or N/A")
    p.add_argument("--classification", default="META", help="FEATURE|FIX|DOCS|OPS|META")
    p.add_argument("--compatibility", default="A", help="A|B|C")
    p.add_argument("--out", default="", help="Output file. If omitted: stdout.")
    args = p.parse_args(argv)

    md = generate_pr_body(
        template=args.template,
        issue=args.issue,
        reason=args.reason,
        mp_id=args.mp_id,
        runbook=args.runbook,
        classification=args.classification,
        compatibility=args.compatibility,
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


def main() -> int:
    if len(sys.argv) < 2:
        print("Usage: python -m yai_tools.cli <pr-body|pr-check|branch|issue-body> ...", file=sys.stderr)
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

    print(f"Unknown subcommand: {sub}", file=sys.stderr)
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
