#!/usr/bin/env bash
set -euo pipefail

SCRIPT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/auth_release_gate.sh"

fail() {
  echo "[FAIL] $1" >&2
  exit 1
}

pass() {
  echo "[PASS] $1"
}

make_mock_cargo() {
  local dir="$1"
  cat > "$dir/mock-cargo" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail

if [[ "$1" != "test" ]]; then
  echo "unexpected command" >&2
  exit 2
fi

suite="${4:-}"
if [[ "$suite" == "auth_lifecycle" ]]; then
  if [[ "${MOCK_FAIL_AUTH_LIFECYCLE:-0}" == "1" ]]; then
    echo "simulated auth_lifecycle failure" >&2
    exit 1
  fi
  echo "auth_lifecycle ok"
  exit 0
fi

if [[ "$suite" == "auth" ]]; then
  if [[ "${MOCK_FAIL_AUTH:-0}" == "1" ]]; then
    echo "simulated auth failure" >&2
    exit 1
  fi
  echo "auth ok"
  exit 0
fi

echo "unexpected suite: $suite" >&2
exit 3
MOCK
  chmod +x "$dir/mock-cargo"
}

test_default_run_marks_pending_external_gates() {
  local tmp
  tmp="$(mktemp -d)"
  make_mock_cargo "$tmp"

  RUSTOK_CARGO_BIN="$tmp/mock-cargo" "$SCRIPT" --artifacts-dir "$tmp/artifacts" >"$tmp/out.log"

  local report
  report="$(rg -o 'Done\. Report: .*' "$tmp/out.log" | sed 's/Done\. Report: //')"
  [[ -n "$report" && -f "$report" ]] || fail "report file missing"
  rg -q '| Integration .* | Done |' "$report" || fail "integration gate should be done"
  rg -q '| REST/GraphQL parity \(staging\) | Pending |' "$report" || fail "parity gate should be pending"
  rg -q '| Security review sign-off | Pending |' "$report" || fail "security gate should be pending"
  pass "default run executes local tests and leaves external gates pending"
}

test_require_all_gates_fails_when_external_evidence_missing() {
  local tmp
  tmp="$(mktemp -d)"
  make_mock_cargo "$tmp"

  set +e
  RUSTOK_CARGO_BIN="$tmp/mock-cargo" "$SCRIPT" --require-all-gates --artifacts-dir "$tmp/artifacts" >"$tmp/out.log" 2>&1
  local code=$?
  set -e

  [[ $code -eq 1 ]] || fail "expected --require-all-gates to fail"
  rg -q 'Gate check failed' "$tmp/out.log" || fail "missing gate failure message"
  pass "require-all-gates fails without parity/security evidence"
}

test_require_all_gates_passes_with_evidence_files() {
  local tmp
  tmp="$(mktemp -d)"
  make_mock_cargo "$tmp"

  mkdir -p "$tmp/evidence"
  echo "parity ok" > "$tmp/evidence/parity.md"
  echo "security ok" > "$tmp/evidence/security.md"

  RUSTOK_CARGO_BIN="$tmp/mock-cargo" "$SCRIPT" \
    --require-all-gates \
    --parity-report "$tmp/evidence/parity.md" \
    --security-signoff "$tmp/evidence/security.md" \
    --artifacts-dir "$tmp/artifacts" >"$tmp/out.log"

  local report
  report="$(rg -o 'Done\. Report: .*' "$tmp/out.log" | sed 's/Done\. Report: //')"
  [[ -n "$report" && -f "$report" ]] || fail "report file missing"
  rg -q '| REST/GraphQL parity \(staging\) | Done |' "$report" || fail "parity gate should be done"
  rg -q '| Security review sign-off | Done |' "$report" || fail "security gate should be done"
  pass "require-all-gates passes with parity and security evidence"
}

test_skip_local_tests_marks_integration_pending() {
  local tmp
  tmp="$(mktemp -d)"
  make_mock_cargo "$tmp"

  RUSTOK_CARGO_BIN="$tmp/mock-cargo" "$SCRIPT" --skip-local-tests --artifacts-dir "$tmp/artifacts" >"$tmp/out.log"

  local report
  report="$(rg -o 'Done\. Report: .*' "$tmp/out.log" | sed 's/Done\. Report: //')"
  [[ -n "$report" && -f "$report" ]] || fail "report file missing"
  rg -q '| Integration .* | Pending | Skipped by flag --skip-local-tests |' "$report" || fail "integration gate should be pending when skipped"
  pass "skip-local-tests leaves integration gate pending"
}

test_default_run_marks_pending_external_gates
test_require_all_gates_fails_when_external_evidence_missing
test_require_all_gates_passes_with_evidence_files
test_skip_local_tests_marks_integration_pending

echo "auth_release_gate tests passed"
