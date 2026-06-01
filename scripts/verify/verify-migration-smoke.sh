#!/usr/bin/env bash
set -euo pipefail

# Applies the server migrator from zero against a fresh PostgreSQL database.
#
# Configuration:
#   RUSTOK_MIGRATION_SMOKE_ADMIN_URL  Admin database URL used for CREATE/DROP DATABASE.
#                                     Defaults to postgres://postgres:postgres@localhost:5432/postgres.
#   RUSTOK_MIGRATION_SMOKE_DB_NAME    Optional temporary database name.
#   RUSTOK_MIGRATION_SMOKE_KEEP_DB    Set to 1 to skip cleanup after the run.

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
ADMIN_URL=${RUSTOK_MIGRATION_SMOKE_ADMIN_URL:-postgres://postgres:postgres@localhost:5432/postgres}
DB_NAME=${RUSTOK_MIGRATION_SMOKE_DB_NAME:-rustok_migration_smoke_$(date +%Y%m%d%H%M%S)_$$}
KEEP_DB=${RUSTOK_MIGRATION_SMOKE_KEEP_DB:-0}

if ! [[ "$DB_NAME" =~ ^[A-Za-z_][A-Za-z0-9_]*$ ]]; then
  echo "Invalid RUSTOK_MIGRATION_SMOKE_DB_NAME '$DB_NAME'. Use letters, digits, and underscores; first char must be a letter or underscore." >&2
  exit 2
fi

if ! command -v psql >/dev/null 2>&1; then
  echo "psql is required for migration smoke database create/drop." >&2
  exit 2
fi

TARGET_URL=$(python - "$ADMIN_URL" "$DB_NAME" <<'PY'
import sys
from urllib.parse import urlsplit, urlunsplit

admin_url = sys.argv[1]
db_name = sys.argv[2]
parts = urlsplit(admin_url)
if parts.scheme not in {"postgres", "postgresql"}:
    raise SystemExit("RUSTOK_MIGRATION_SMOKE_ADMIN_URL must use postgres:// or postgresql://")
print(urlunsplit((parts.scheme, parts.netloc, f"/{db_name}", parts.query, parts.fragment)))
PY
)

cleanup() {
  if [[ "$KEEP_DB" == "1" ]]; then
    echo "Keeping smoke database '$DB_NAME' at $TARGET_URL"
    return
  fi
  psql "$ADMIN_URL" -v ON_ERROR_STOP=1 -c "DROP DATABASE IF EXISTS \"$DB_NAME\" WITH (FORCE);" >/dev/null
}
trap cleanup EXIT

echo "Creating smoke database '$DB_NAME'"
psql "$ADMIN_URL" -v ON_ERROR_STOP=1 -c "CREATE DATABASE \"$DB_NAME\";" >/dev/null

echo "Applying server migrator from zero"
(
  cd "$ROOT_DIR"
  RUSTOK_MIGRATION_SMOKE_DATABASE_URL="$TARGET_URL" \
    cargo test -p migration --test postgres_zero_migration_smoke \
      postgres_zero_migration_smoke_applies_from_empty_database -- --ignored --nocapture
)

echo "Migration smoke completed successfully for '$DB_NAME'"
