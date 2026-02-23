import argparse
import base64
import json
import re
import subprocess
import sys
from typing import Any

from yai_tools.issue.body import generate_issue_body
from yai_tools.issue.templates import (
    canonical_milestone_title,
    canonical_mp_closure_title,
    canonical_phase_issue_title,
    default_mp_id,
    default_rb_id,
    mp_closure_labels,
    phase_issue_labels,
    phase_label,
    pr_phase_labels,
    render_milestone_body,
    render_mp_closure_body,
    render_phase_issue_body,
    track_label,
)
from yai_tools.pr.body import generate_pr_body
from yai_tools.pr.check import check_pr_body
from yai_tools.verify.agent_pack import run_agent_pack
from yai_tools.verify.architecture_alignment import run_architecture_alignment
from yai_tools.verify.doctor import run_doctor
from yai_tools.verify.frontmatter_schema import run_schema_check
from yai_tools.verify.trace_graph import run_graph
from yai_tools.workflow.branch import make_branch_name, maybe_checkout

_DEFAULT_LABEL_COLOR = "d4a72c"
_EXACT_LABEL_COLORS: dict[str, str] = {
    "bug": "d73a4a",
    "enhancement": "a2eeef",
    "docs": "0075ca",
    "governance": "5319e7",
    "runbook": "1d76db",
    "mp-closure": "8250df",
}
_PREFIX_LABEL_COLORS: tuple[tuple[str, str], ...] = (
    ("phase:", "1d76db"),
    ("track:", "0e8a16"),
    ("class:", "fbca04"),
    ("work-type:", "c5def5"),
    ("worktype:", "c5def5"),
    ("type:", "bfd4f2"),
    ("area:", "0052cc"),
)


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


def _run(args: list[str], input_text: str | None = None) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        args,
        input=input_text,
        text=True,
        capture_output=True,
        check=False,
    )


def _gh_json(args: list[str], input_text: str | None = None) -> Any:
    run = _run(["gh", *args], input_text=input_text)
    if run.returncode != 0:
        raise RuntimeError(run.stderr.strip() or run.stdout.strip() or "gh command failed")
    out = run.stdout.strip()
    if not out:
        return None
    return json.loads(out)


def _gh(args: list[str], input_text: str | None = None) -> str:
    run = _run(["gh", *args], input_text=input_text)
    if run.returncode != 0:
        raise RuntimeError(run.stderr.strip() or run.stdout.strip() or "gh command failed")
    return run.stdout.strip()


def _fetch_repo_issues(repo: str) -> list[dict[str, Any]]:
    run = _run(
        [
            "gh",
            "api",
            "--paginate",
            f"repos/{repo}/issues?state=all&per_page=100",
            "--jq",
            ".[] | @base64",
        ]
    )
    if run.returncode != 0:
        raise RuntimeError(run.stderr.strip() or run.stdout.strip() or "failed to fetch issues")

    out: list[dict[str, Any]] = []
    for line in run.stdout.splitlines():
        line = line.strip()
        if not line:
            continue
        raw = base64.b64decode(line.encode("utf-8")).decode("utf-8")
        out.append(json.loads(raw))
    return out


def _fetch_milestones(repo: str) -> list[dict[str, Any]]:
    return _gh_json(["api", f"repos/{repo}/milestones?state=all&per_page=100"])


def _fetch_labels(repo: str) -> list[dict[str, Any]]:
    run = _run(
        [
            "gh",
            "api",
            "--paginate",
            f"repos/{repo}/labels?per_page=100",
            "--jq",
            ".[] | @base64",
        ]
    )
    if run.returncode != 0:
        raise RuntimeError(run.stderr.strip() or run.stdout.strip() or "failed to fetch labels")

    out: list[dict[str, Any]] = []
    for line in run.stdout.splitlines():
        line = line.strip()
        if not line:
            continue
        raw = base64.b64decode(line.encode("utf-8")).decode("utf-8")
        out.append(json.loads(raw))
    return out


def _label_color(label: str) -> str:
    normalized = label.strip().lower()
    if normalized in _EXACT_LABEL_COLORS:
        return _EXACT_LABEL_COLORS[normalized]
    for prefix, color in _PREFIX_LABEL_COLORS:
        if normalized.startswith(prefix):
            return color
    return _DEFAULT_LABEL_COLOR


def _ensure_label(repo: str, label: str, apply: bool) -> None:
    if not apply:
        return
    _gh(["label", "create", label, "-R", repo, "--force", "--color", _label_color(label)])


def _ensure_labels(repo: str, labels: list[str], apply: bool) -> None:
    for label in labels:
        _ensure_label(repo, label, apply)


def _seed_labels_from_labeler(repo_root: str) -> set[str]:
    path = f"{repo_root}/.github/labeler.yml"
    out: set[str] = set()
    try:
        with open(path, "r", encoding="utf-8") as fh:
            for line in fh:
                match = re.match(r'^\s*"([^"]+)":\s*$', line)
                if match:
                    out.add(match.group(1))
    except FileNotFoundError:
        return out
    return out


def _find_milestone_by_title(milestones: list[dict[str, Any]], title: str) -> dict[str, Any] | None:
    for ms in milestones:
        if (ms.get("title") or "") == title:
            return ms
    return None


def _ensure_phase_milestone(
    repo: str,
    track: str,
    phase: str,
    rb_anchor: str,
    mp_id: str,
    apply: bool,
    report: list[str],
) -> tuple[str, int | None]:
    canonical_title = canonical_milestone_title(track, phase)
    legacy_title = f"{default_rb_id(track)}-{phase}"
    body = render_milestone_body(track=track, phase=phase, rb_anchor=rb_anchor, mp_id=mp_id)

    milestones = _fetch_milestones(repo)
    canonical = _find_milestone_by_title(milestones, canonical_title)
    if canonical:
        return canonical_title, int(canonical["number"])

    legacy = _find_milestone_by_title(milestones, legacy_title)
    if legacy:
        number = int(legacy["number"])
        report.append(f"milestone rename: {legacy_title} -> {canonical_title}")
        if apply:
            _gh_json(
                ["api", "-X", "PATCH", f"repos/{repo}/milestones/{number}", "--input", "-"],
                input_text=json.dumps({"title": canonical_title, "description": body}),
            )
        return canonical_title, number

    report.append(f"milestone create: {canonical_title}")
    if not apply:
        return canonical_title, None

    created = _gh_json(
        ["api", "-X", "POST", f"repos/{repo}/milestones", "--input", "-"],
        input_text=json.dumps({"title": canonical_title, "description": body}),
    )
    return canonical_title, int(created["number"])


def _create_issue(
    repo: str,
    title: str,
    body: str,
    labels: list[str],
    milestone_number: int | None,
    apply: bool,
) -> int:
    payload: dict[str, Any] = {
        "title": title,
        "body": body,
        "labels": labels,
    }
    if milestone_number is not None:
        payload["milestone"] = milestone_number

    if not apply:
        print("DRY-RUN create issue")
        print(json.dumps(payload, indent=2))
        return 0

    _ensure_labels(repo, labels, apply=True)
    created = _gh_json(
        ["api", "-X", "POST", f"repos/{repo}/issues", "--input", "-"],
        input_text=json.dumps(payload),
    )
    print(created.get("html_url", ""))
    return int(created["number"])


def _issue_labels(issue: dict[str, Any]) -> set[str]:
    out: set[str] = set()
    for label in issue.get("labels", []) or []:
        if isinstance(label, dict):
            name = label.get("name")
            if name:
                out.add(name)
    return out


def _legacy_closure_title(rb_id: str, phase: str) -> str:
    return f"{rb_id}-{phase}"


def _mp_id_is_phase_specific(mp_id: str, phase: str) -> bool:
    return mp_id.upper().endswith(f"-{phase}".upper())


def _matches_phase(issue: dict[str, Any], track: str, phase: str, rb_id: str, mp_id: str, milestone_title: str) -> bool:
    title = (issue.get("title") or "")
    body = (issue.get("body") or "")
    title_l = title.lower()
    body_l = body.lower()
    labels = _issue_labels(issue)

    ms = issue.get("milestone") or {}
    ms_title = (ms.get("title") or "")
    if ms_title == milestone_title:
        return True

    if ms_title == _legacy_closure_title(rb_id, phase):
        return True

    if phase_label(phase) in labels or track_label(track) in labels:
        return True

    if phase in title_l or phase in body_l:
        needles = [track.lower(), rb_id.lower(), mp_id.lower(), _legacy_closure_title(rb_id, phase).lower()]
        if any(n in title_l or n in body_l for n in needles):
            return True

    if _legacy_closure_title(rb_id, phase).lower() in title_l:
        return True

    return False


def _role_for_issue(issue: dict[str, Any], rb_id: str, mp_id: str, phase: str) -> str:
    title = (issue.get("title") or "").strip()
    title_l = title.lower()

    if issue.get("pull_request"):
        return "pr"

    if title_l.startswith("mp-closure:"):
        return "mp-closure"

    if title_l.startswith("runbook:"):
        return "phase-issue"

    if title_l == _legacy_closure_title(rb_id, phase).lower():
        return "mp-closure"

    if rb_id.lower() in title_l and phase in title_l:
        return "phase-issue"

    if mp_id.lower() in title_l and phase in title_l:
        return "mp-closure"

    return "other"


def _canonicalize_phase_issue_title(title: str, rb_id: str, phase: str) -> str | None:
    txt = title.strip()
    low = txt.lower()
    if low.startswith("runbook:"):
        return None
    if txt.startswith(rb_id):
        return f"runbook: {txt}"
    if rb_id in txt and phase in txt:
        return f"runbook: {txt}"
    return None


def _report_line(
    number: int,
    old_title: str,
    new_title: str | None,
    add_labels: list[str],
    remove_labels: list[str],
    milestone_changed: bool,
) -> str:
    ren = f"{old_title} -> {new_title}" if new_title else old_title
    return (
        f"#{number}: {ren}; "
        f"labels +{add_labels or []} -{remove_labels or []}; "
        f"milestone_updated={str(milestone_changed).lower()}"
    )


def cmd_pr_body(argv: list[str]) -> int:
    p = argparse.ArgumentParser(prog="yai-pr-body", add_help=True)
    p.add_argument("--template", default="default", help="default|docs-governance|type-a-milestone|type-b-twin-pr")
    p.add_argument("--issue", required=True, help="#123 or 123 or N/A")
    p.add_argument("--reason", default="", help="Required when issue is N/A")
    p.add_argument("--mp-id", default="N/A", help="MP-... or N/A")
    p.add_argument("--runbook", default="N/A", help="docs/20-governance/runbooks/<name>.md#<anchor> or N/A")
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
                    if code == 0:
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
    p.add_argument("--runbook", default="N/A", help="docs/20-governance/runbooks/<name>.md")
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


def cmd_milestone_body(argv: list[str]) -> int:
    p = argparse.ArgumentParser(prog="yai-dev milestone body", add_help=True)
    p.add_argument("--track", required=True)
    p.add_argument("--phase", required=True)
    p.add_argument("--rb-anchor", required=True)
    p.add_argument("--mp-id", required=True, help="Phase MP-ID, e.g. MP-CONTRACT-BASELINE-LOCK-0.1.0")
    p.add_argument("--objective", default="")
    p.add_argument("--out", default="")
    args = p.parse_args(argv)

    body = render_milestone_body(
        track=args.track,
        phase=args.phase,
        rb_anchor=args.rb_anchor,
        mp_id=args.mp_id,
        objective=args.objective,
    )
    if args.out:
        with open(args.out, "w", encoding="utf-8") as f:
            f.write(body)
        return 0
    sys.stdout.write(body)
    return 0


def cmd_issue_phase(argv: list[str]) -> int:
    p = argparse.ArgumentParser(prog="yai-dev issue phase", add_help=True)
    p.add_argument("--track", required=True)
    p.add_argument("--phase", required=True)
    p.add_argument("--rb-id", required=True)
    p.add_argument("--title", required=True, help="Short phase title, e.g. Pin Baseline Freeze")
    p.add_argument("--rb-anchor", required=True)
    p.add_argument("--mp-id", required=True, help="Phase MP-ID, e.g. MP-CONTRACT-BASELINE-LOCK-0.1.0")
    p.add_argument("--repo", default="yai-labs/yai")
    p.add_argument("--class-label", default="")
    p.add_argument("--dry-run", action="store_true")
    args = p.parse_args(argv)
    if not _mp_id_is_phase_specific(args.mp_id, args.phase):
        print(
            f"warning: mp-id '{args.mp_id}' is not phase-specific; expected suffix '-{args.phase}'",
            file=sys.stderr,
        )

    report: list[str] = []
    milestone_title, milestone_number = _ensure_phase_milestone(
        repo=args.repo,
        track=args.track,
        phase=args.phase,
        rb_anchor=args.rb_anchor,
        mp_id=args.mp_id,
        apply=not args.dry_run,
        report=report,
    )

    title = canonical_phase_issue_title(args.rb_id, args.phase, args.title)
    body = render_phase_issue_body(
        track=args.track,
        phase=args.phase,
        rb_id=args.rb_id,
        rb_anchor=args.rb_anchor,
        mp_id=args.mp_id,
    )
    labels = phase_issue_labels(track=args.track, phase=args.phase, include_class_label=False)
    if args.class_label:
        labels.append(args.class_label)

    number = _create_issue(
        repo=args.repo,
        title=title,
        body=body,
        labels=labels,
        milestone_number=milestone_number,
        apply=not args.dry_run,
    )
    if args.dry_run:
        print(f"DRY-RUN milestone={milestone_title} issue={title} number={number}")
        for row in report:
            print(row)
    return 0


def cmd_issue_mp_closure(argv: list[str]) -> int:
    p = argparse.ArgumentParser(prog="yai-dev issue mp-closure", add_help=True)
    p.add_argument("--track", required=True)
    p.add_argument("--phase", required=True)
    p.add_argument("--mp-id", required=True)
    p.add_argument("--repo", default="yai-labs/yai")
    p.add_argument("--rb-anchor", default="")
    p.add_argument("--dry-run", action="store_true")
    args = p.parse_args(argv)
    if not _mp_id_is_phase_specific(args.mp_id, args.phase):
        print(
            f"warning: mp-id '{args.mp_id}' is not phase-specific; expected suffix '-{args.phase}'",
            file=sys.stderr,
        )

    rb_anchor = args.rb_anchor.strip() or f"docs/20-governance/runbooks/{args.track}.md#{args.phase}"
    report: list[str] = []
    milestone_title, milestone_number = _ensure_phase_milestone(
        repo=args.repo,
        track=args.track,
        phase=args.phase,
        rb_anchor=rb_anchor,
        mp_id=args.mp_id,
        apply=not args.dry_run,
        report=report,
    )

    title = canonical_mp_closure_title(args.mp_id, args.phase)
    body = render_mp_closure_body(
        track=args.track,
        phase=args.phase,
        mp_id=args.mp_id,
        milestone_title=milestone_title,
    )
    labels = mp_closure_labels(track=args.track, phase=args.phase)

    number = _create_issue(
        repo=args.repo,
        title=title,
        body=body,
        labels=labels,
        milestone_number=milestone_number,
        apply=not args.dry_run,
    )
    if args.dry_run:
        print(f"DRY-RUN milestone={milestone_title} issue={title} number={number}")
        for row in report:
            print(row)
    return 0


def cmd_fix_phase(argv: list[str]) -> int:
    p = argparse.ArgumentParser(prog="yai-dev fix phase", add_help=True)
    p.add_argument("--track", required=True)
    p.add_argument("--phase", required=True)
    p.add_argument("--repo", default="yai-labs/yai")
    p.add_argument("--rb-id", default="")
    p.add_argument("--mp-id", default="", help="Phase MP-ID; default derives to MP-<TRACK>-<phase>")
    p.add_argument("--rb-anchor", default="")
    p.add_argument("--class-label", default="")
    p.add_argument("--apply", action="store_true")
    args = p.parse_args(argv)

    rb_id = args.rb_id.strip() or default_rb_id(args.track)
    mp_id = args.mp_id.strip() or default_mp_id(args.track, args.phase)
    if not _mp_id_is_phase_specific(mp_id, args.phase):
        print(
            f"warning: mp-id '{mp_id}' is not phase-specific; expected suffix '-{args.phase}'",
            file=sys.stderr,
        )
    rb_anchor = args.rb_anchor.strip() or f"docs/20-governance/runbooks/{args.track}.md#{args.phase}"
    apply = args.apply

    report: list[str] = []
    warnings: list[str] = []
    milestone_title, milestone_number = _ensure_phase_milestone(
        repo=args.repo,
        track=args.track,
        phase=args.phase,
        rb_anchor=rb_anchor,
        mp_id=mp_id,
        apply=apply,
        report=report,
    )

    issues = _fetch_repo_issues(args.repo)
    candidates = [
        it
        for it in issues
        if _matches_phase(it, track=args.track, phase=args.phase, rb_id=rb_id, mp_id=mp_id, milestone_title=milestone_title)
    ]

    if not candidates:
        warnings.append("No candidate issues/PRs found for requested phase.")

    canonical_phase = phase_label(args.phase)
    canonical_track = track_label(args.track)
    canonical_mp_title = canonical_mp_closure_title(mp_id, args.phase)

    for item in candidates:
        number = int(item["number"])
        old_title = item.get("title") or ""
        role = _role_for_issue(item, rb_id=rb_id, mp_id=mp_id, phase=args.phase)
        existing_labels = _issue_labels(item)

        desired_labels: set[str] = set()
        if role == "phase-issue":
            desired_labels.update(phase_issue_labels(args.track, args.phase, include_class_label=False))
            if args.class_label:
                desired_labels.add(args.class_label)
        elif role == "mp-closure":
            desired_labels.update(mp_closure_labels(args.track, args.phase))
        elif role == "pr":
            desired_labels.update(pr_phase_labels(args.track, args.phase))
        else:
            desired_labels.update(pr_phase_labels(args.track, args.phase))

        to_add = sorted(x for x in desired_labels if x not in existing_labels)
        to_remove: list[str] = []
        for label in sorted(existing_labels):
            if label.startswith("phase:") and label != canonical_phase:
                to_remove.append(label)
            if label.startswith("track:") and label != canonical_track:
                to_remove.append(label)

        if role == "phase-issue" and "mp-closure" in existing_labels:
            to_remove.append("mp-closure")
        if role == "mp-closure" and "runbook" in existing_labels:
            to_remove.append("runbook")

        new_title: str | None = None
        legacy = _legacy_closure_title(rb_id, args.phase)
        if role == "mp-closure":
            if old_title.strip() == legacy or not old_title.lower().startswith("mp-closure:"):
                new_title = canonical_mp_title
        elif role == "phase-issue":
            new_title = _canonicalize_phase_issue_title(old_title, rb_id=rb_id, phase=args.phase)

        ms = item.get("milestone") or {}
        ms_title = ms.get("title") or ""
        milestone_changed = ms_title != milestone_title

        merged_labels = set(existing_labels)
        for label in to_remove:
            merged_labels.discard(label)
        merged_labels.update(to_add)

        report.append(
            _report_line(
                number=number,
                old_title=old_title,
                new_title=new_title,
                add_labels=to_add,
                remove_labels=sorted(set(to_remove)),
                milestone_changed=milestone_changed,
            )
        )

        if not apply:
            continue

        _ensure_labels(args.repo, sorted(merged_labels), apply=True)

        payload: dict[str, Any] = {
            "labels": sorted(merged_labels),
        }
        if milestone_number is not None:
            payload["milestone"] = milestone_number
        if new_title:
            payload["title"] = new_title

        _gh_json(
            ["api", "-X", "PATCH", f"repos/{args.repo}/issues/{number}", "--input", "-"],
            input_text=json.dumps(payload),
        )

    print(f"phase: {args.track}@{args.phase}")
    print(f"milestone: {milestone_title}")
    print(f"mode: {'APPLY' if apply else 'DRY-RUN'}")
    print("updates:")
    for row in report:
        print(f"- {row}")

    if warnings:
        print("warnings:")
        for w in warnings:
            print(f"- {w}")

    return 0


def cmd_label_sync(argv: list[str]) -> int:
    p = argparse.ArgumentParser(prog="yai-label-sync", add_help=True)
    p.add_argument("--repo", default="yai-labs/yai")
    p.add_argument("--apply", action="store_true")
    args = p.parse_args(argv)

    apply = args.apply
    report: list[str] = []

    repo_root = _repo_root()
    seed_labels = _seed_labels_from_labeler(repo_root)
    seed_labels.update(
        {
            "runbook",
            "governance",
            "mp-closure",
            "class:A",
            "class:B",
            "class:C",
            "bug",
            "enhancement",
            "docs",
            "type:docs",
            "type:ci",
        }
    )

    current = _fetch_labels(args.repo)
    current_by_name = {str(x.get("name", "")).strip(): x for x in current if str(x.get("name", "")).strip()}
    all_labels = sorted(set(current_by_name.keys()) | seed_labels)

    for name in all_labels:
        desired_color = _label_color(name)
        current_color = ""
        if name in current_by_name:
            current_color = str(current_by_name[name].get("color", "")).strip().lower()
        if current_color == desired_color:
            continue

        if current_color:
            report.append(f"{name}: {current_color} -> {desired_color}")
        else:
            report.append(f"{name}: <missing> -> {desired_color}")

        if apply:
            _gh(["label", "create", name, "-R", args.repo, "--force", "--color", desired_color])

    print(f"repo: {args.repo}")
    print(f"mode: {'APPLY' if apply else 'DRY-RUN'}")
    print("updates:")
    if report:
        for row in report:
            print(f"- {row}")
    else:
        print("- none")
    return 0


def cmd_dev_issue(argv: list[str]) -> int:
    if not argv or argv[0] in {"-h", "--help"}:
        print("Usage: yai-dev-issue <phase|mp-closure|legacy-body> ...")
        print("  phase      create canonical runbook phase issue")
        print("  mp-closure create canonical MP closure issue")
        print("  legacy-body forwards to old issue-body flags (--title --type ...)")
        return 0
    if argv and argv[0] == "phase":
        return cmd_issue_phase(argv[1:])
    if argv and argv[0] == "mp-closure":
        return cmd_issue_mp_closure(argv[1:])
    return cmd_issue_body(argv)


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
            "Usage: python -m yai_tools.cli <pr-body|pr-check|branch|issue-body|dev-issue|milestone-body|issue-phase|issue-mp-closure|fix-phase|label-sync|docs-schema-check|docs-graph|agent-pack|docs-doctor|architecture-check> ...",
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
    if sub == "dev-issue":
        return cmd_dev_issue(rest)
    if sub == "milestone-body":
        return cmd_milestone_body(rest)
    if sub == "issue-phase":
        return cmd_issue_phase(rest)
    if sub == "issue-mp-closure":
        return cmd_issue_mp_closure(rest)
    if sub == "fix-phase":
        return cmd_fix_phase(rest)
    if sub == "label-sync":
        return cmd_label_sync(rest)
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
