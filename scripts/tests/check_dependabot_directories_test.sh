#!/usr/bin/env bash
set -euo pipefail

SCRIPT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/ci/check-dependabot-directories.py"

fail() {
  echo "[FAIL] $1" >&2
  exit 1
}

pass() {
  echo "[PASS] $1"
}

test_passes_for_existing_directories() {
  python3 "$SCRIPT" >/tmp/check_dependabot_ok.log
  rg -q "All Dependabot update directories exist" /tmp/check_dependabot_ok.log \
    || fail "expected success message"
  pass "script passes for current repository dependabot directories"
}

test_fails_for_missing_directory() {
  local tmp
  tmp="$(mktemp -d)"
  mkdir -p "$tmp/.github" "$tmp/apps/admin"
  cat > "$tmp/.github/dependabot.yml" <<'YAML'
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/apps/admin"
    schedule:
      interval: "daily"
  - package-ecosystem: "cargo"
    directory: "/apps/does-not-exist"
    schedule:
      interval: "daily"
YAML

  set +e
  python3 "$SCRIPT" --root "$tmp" --config "$tmp/.github/dependabot.yml" >"$tmp/out.log" 2>&1
  local code=$?
  set -e

  [[ $code -eq 1 ]] || fail "expected exit code 1 for missing directory"
  rg -q "Dependabot directories do not exist" "$tmp/out.log" || fail "expected failure heading"
  rg -q "/apps/does-not-exist" "$tmp/out.log" || fail "expected missing directory in output"
  pass "script fails when dependabot contains missing directory"
}

test_passes_for_existing_directories
test_fails_for_missing_directory

echo "check_dependabot_directories tests passed"
