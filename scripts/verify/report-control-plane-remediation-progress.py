#!/usr/bin/env python3
from __future__ import annotations

import os
import re
from pathlib import Path

PLAN_PATH = Path(os.environ.get("RUSTOK_REMEDIATION_PLAN_PATH", "docs/research/control-plane-module-lifecycle-remediation-plan.md"))


def main() -> int:
    if not PLAN_PATH.exists():
        print(f"ERROR: remediation plan not found: {PLAN_PATH}")
        return 1

    text = PLAN_PATH.read_text(encoding="utf-8")
    lines = text.splitlines()
    pending: list[tuple[int, str]] = []
    in_progress: list[tuple[int, str]] = []

    for idx, line in enumerate(lines, start=1):
        if re.search(r"- \[ \]", line):
            pending.append((idx, line.strip()))
        elif re.search(r"- \[~\]", line):
            in_progress.append((idx, line.strip()))

    completed = len(re.findall(r"- \[x\]", text))

    print("Control-plane remediation plan progress")
    print(f"source: {PLAN_PATH}")
    print(f"completed: {completed}")
    print(f"in_progress: {len(in_progress)}")
    print(f"pending: {len(pending)}")

    if in_progress:
        print("\nTop in-progress items:")
        for line_no, item in in_progress[:10]:
            print(f"  L{line_no}: {item}")

    if pending:
        print("\nTop pending items:")
        for line_no, item in pending[:10]:
            print(f"  L{line_no}: {item}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
