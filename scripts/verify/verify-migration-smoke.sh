#!/usr/bin/env bash
set -euo pipefail

# Applies the server migrator from zero against a fresh PostgreSQL database.
# Database create/drop is handled by the ignored Rust integration test, so this
# script does not require psql on the caller machine.
#
# Configuration:
#   RUSTOK_MIGRATION_SMOKE_ADMIN_URL  Admin database URL used for CREATE/DROP DATABASE.
#                                     Defaults to postgres://postgres:postgres@localhost:5432/postgres.
#   RUSTOK_MIGRATION_SMOKE_DB_NAME    Optional temporary database name.
#   RUSTOK_MIGRATION_SMOKE_KEEP_DB    Set to 1 to skip cleanup after the run.

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
export RUSTOK_MIGRATION_SMOKE_ADMIN_URL=${RUSTOK_MIGRATION_SMOKE_ADMIN_URL:-postgres://postgres:postgres@localhost:5432/postgres}
export RUSTOK_MIGRATION_SMOKE_DB_NAME=${RUSTOK_MIGRATION_SMOKE_DB_NAME:-rustok_migration_smoke_$(date +%Y%m%d%H%M%S)_$$}
export RUSTOK_MIGRATION_SMOKE_KEEP_DB=${RUSTOK_MIGRATION_SMOKE_KEEP_DB:-0}

if ! [[ "$RUSTOK_MIGRATION_SMOKE_DB_NAME" =~ ^[A-Za-z_][A-Za-z0-9_]*$ ]]; then
  echo "Invalid RUSTOK_MIGRATION_SMOKE_DB_NAME '$RUSTOK_MIGRATION_SMOKE_DB_NAME'. Use letters, digits, and underscores; first char must be a letter or underscore." >&2
  exit 2
fi

if ! [[ "$RUSTOK_MIGRATION_SMOKE_ADMIN_URL" =~ ^postgres(ql)?:// ]]; then
  echo "RUSTOK_MIGRATION_SMOKE_ADMIN_URL must use postgres:// or postgresql://" >&2
  exit 2
fi

echo "Running PostgreSQL migration smoke against temporary database '$RUSTOK_MIGRATION_SMOKE_DB_NAME'"
(
  cd "$ROOT_DIR"
  cargo test -p migration --test postgres_zero_migration_smoke \
    postgres_zero_migration_smoke_applies_from_empty_database -- --ignored --nocapture
)

echo "Migration smoke completed successfully for '$RUSTOK_MIGRATION_SMOKE_DB_NAME'"
