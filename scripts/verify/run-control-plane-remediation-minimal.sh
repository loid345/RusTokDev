#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

run_step() {
  local title="$1"
  shift
  echo "\n==> ${title}"
  echo "\$ $*"
  "$@"
}

run_step "format check" cargo fmt --all -- --check
run_step "migration tests" cargo test -p migration
run_step "module lifecycle tests" cargo test -p rustok-server module_lifecycle
run_step "platform composition tests" cargo test -p rustok-server platform_composition
run_step "manifest validation" cargo xtask validate-manifest
run_step "module contract validation" cargo xtask module validate
run_step "dependabot directory contract" python3 scripts/ci/check-dependabot-directories.py

echo "\nControl-plane remediation minimal verification: PASS"
