#!/usr/bin/env python3
"""Validate that every Dependabot update directory exists in the repository."""
from __future__ import annotations

import argparse
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
CONFIG = ROOT / ".github" / "dependabot.yml"
DIRECTORY_RE = re.compile(r'^\s*directory:\s*["\']?([^"\'\s#]+)')


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Validate that every Dependabot update directory exists."
    )
    parser.add_argument(
        "--root",
        type=pathlib.Path,
        default=ROOT,
        help="Repository root directory (default: auto-detected from script location).",
    )
    parser.add_argument(
        "--config",
        type=pathlib.Path,
        default=CONFIG,
        help="Path to dependabot.yml (default: <root>/.github/dependabot.yml).",
    )
    return parser.parse_args()


def normalize_directory(directory: str) -> str:
    trimmed = directory.strip()
    if not trimmed:
        return trimmed
    if not trimmed.startswith("/"):
        trimmed = f"/{trimmed}"
    if trimmed != "/":
        trimmed = trimmed.rstrip("/")
    return trimmed


def main() -> int:
    args = parse_args()
    root = args.root.resolve()
    config = args.config if args.config.is_absolute() else (root / args.config)
    if not config.is_file():
        print(f"Dependabot config file not found: {config}", file=sys.stderr)
        return 1

    missing: list[str] = []
    seen: set[str] = set()
    duplicates: set[str] = set()
    for line in config.read_text(encoding="utf-8").splitlines():
        match = DIRECTORY_RE.match(line)
        if not match:
            continue
        directory = normalize_directory(match.group(1))
        if directory in seen:
            duplicates.add(directory)
        else:
            seen.add(directory)
        path = root / directory.lstrip("/")
        if not path.is_dir():
            missing.append(directory)

    if duplicates:
        print("Dependabot directories contain duplicates:", file=sys.stderr)
        for directory in sorted(duplicates):
            print(f"  - {directory}", file=sys.stderr)
        return 1

    if missing:
        print("Dependabot directories do not exist:", file=sys.stderr)
        for directory in sorted(set(missing)):
            print(f"  - {directory}", file=sys.stderr)
        return 1

    print("All Dependabot update directories exist.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
