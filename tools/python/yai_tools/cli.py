import argparse
import sys

from yai_tools.pr.body import generate_pr_body
from yai_tools.workflow.branch import make_branch_name, maybe_checkout


def cmd_pr_body(argv: list[str]) -> int:
    p = argparse.ArgumentParser(prog="yai-pr-body", add_help=True)
    p.add_argument("--template", required=True, help="default|docs-governance|type-a-milestone|type-b-twin-pr")
    p.add_argument("--issue", required=True, help="#123 or 123 or N/A")
    p.add_argument("--reason", default="", help="Required when issue is N/A")
    p.add_argument("--out", default="", help="Output file. If omitted: stdout.")
    args = p.parse_args(argv)

    md = generate_pr_body(template=args.template, issue=args.issue, reason=args.reason)

    if args.out:
        with open(args.out, "w", encoding="utf-8") as f:
            f.write(md)
        return 0

    sys.stdout.write(md)
    return 0


def cmd_branch(argv: list[str]) -> int:
    p = argparse.ArgumentParser(prog="yai-branch", add_help=True)
    p.add_argument("--type", required=True, help="feat|fix|docs|chore|refactor|test|ci")
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
        print("Usage: python -m yai_tools.cli <pr-body|branch> ...", file=sys.stderr)
        return 2

    sub = sys.argv[1]
    rest = sys.argv[2:]

    if sub == "pr-body":
        return cmd_pr_body(rest)
    if sub == "branch":
        return cmd_branch(rest)

    print(f"Unknown subcommand: {sub}", file=sys.stderr)
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
