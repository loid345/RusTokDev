#!/usr/bin/env python3
"""Verify generated mobile manifest is up to date."""

from __future__ import annotations

import argparse
import json
import pathlib
import sys

if __package__:
    from .generate_mobile_manifest import render, render_snapshot_json, scan_modules
else:
    current_dir = pathlib.Path(__file__).resolve().parent
    if str(current_dir) not in sys.path:
        sys.path.insert(0, str(current_dir))
    from generate_mobile_manifest import render, render_snapshot_json, scan_modules


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--repo-root",
        default=".",
        help="Path to repository root containing crates/*/rustok-module.toml",
    )
    parser.add_argument(
        "--manifest",
        default=(
            "rustok_mobile/apps/rustok_admin_mobile/lib/registry/mobile_manifest.g.dart"
        ),
        help="Path to generated Dart manifest",
    )
    parser.add_argument(
        "--snapshot",
        default=(
            "rustok_mobile/tooling/snapshots/mobile_manifest.snapshot.json"
        ),
        help="Path to generated registry snapshot JSON",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    repo_root = pathlib.Path(args.repo_root).resolve()
    manifest_path = pathlib.Path(args.manifest).resolve()
    snapshot_path = pathlib.Path(args.snapshot).resolve()

    modules = scan_modules(repo_root)
    expected = render(modules)
    expected_snapshot = render_snapshot_json(modules)
    if not manifest_path.exists():
        print(f"Manifest file is missing: {manifest_path}")
        return 1

    current = manifest_path.read_text(encoding="utf-8")
    if current != expected:
        print("ERROR: mobile manifest is stale.")
        print("Run:")
        print(
            "  python3 rustok_mobile/tooling/scripts/generate_mobile_manifest.py "
            f"--repo-root {repo_root}"
        )
        return 1

    if not snapshot_path.exists():
        print(f"Snapshot file is missing: {snapshot_path}")
        return 1

    snapshot_current = snapshot_path.read_text(encoding="utf-8")
    if snapshot_current != expected_snapshot:
        print("ERROR: mobile manifest snapshot is stale.")
        print("Run:")
        print(
            "  python3 rustok_mobile/tooling/scripts/generate_mobile_manifest.py "
            f"--repo-root {repo_root}"
        )
        return 1

    try:
        parsed = json.loads(snapshot_current)
    except json.JSONDecodeError as exc:
        print(f"ERROR: snapshot is not valid JSON: {exc}")
        return 1

    required_keys = {"module_slug", "surface_kind", "route_segment", "permissions", "locale_namespace", "child_pages"}
    for index, item in enumerate(parsed):
        if not isinstance(item, dict):
            print(f"ERROR: snapshot entry #{index} is not an object")
            return 1
        missing = required_keys.difference(item.keys())
        if missing:
            print(f"ERROR: snapshot entry #{index} missing keys: {sorted(missing)}")
            return 1

    print(f"OK: mobile manifest and snapshot are up to date ({manifest_path})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
