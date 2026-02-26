#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT

FAKE_CURL="$WORK_DIR/fake-curl.sh"
ARTIFACTS_DIR="$WORK_DIR/artifacts"
STATE_FILE="$WORK_DIR/state"

cat > "$FAKE_CURL" <<'FAKE'
#!/usr/bin/env bash
set -euo pipefail

state_file="${MOCK_STATE_FILE:?}"
count=0
if [[ -f "$state_file" ]]; then
  count="$(cat "$state_file")"
fi
count=$((count + 1))
printf '%s' "$count" > "$state_file"

cat <<METRICS
rustok_rbac_decision_mismatch_total 0
rustok_rbac_shadow_compare_failures_total 0
rustok_rbac_permission_checks_denied $((count * 2))
rustok_rbac_permission_checks_allowed $((count * 10))
METRICS
FAKE

chmod +x "$FAKE_CURL"

MOCK_STATE_FILE="$STATE_FILE" RUSTOK_CURL_BIN="$FAKE_CURL" \
  "$ROOT_DIR/scripts/rbac_cutover_baseline.sh" \
  --samples 3 \
  --interval-sec 0 \
  --artifacts-dir "$ARTIFACTS_DIR" >/dev/null

REPORT_FILE="$(find "$ARTIFACTS_DIR" -maxdepth 1 -name 'rbac_cutover_baseline_*.md' | head -n 1)"
[[ -n "$REPORT_FILE" ]] || { echo "missing cutover baseline report" >&2; exit 1; }

rg -Fq "status: pass" "$REPORT_FILE"
rg -Fq "mismatch_total: 0 -> 0 (delta 0)" "$REPORT_FILE"

echo "rbac_cutover_baseline smoke test passed"
