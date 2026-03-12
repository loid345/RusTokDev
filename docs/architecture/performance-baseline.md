# Performance Baseline

This document defines the repeatable evidence workflow for `2.8`.

## Goal

Before query rewrites, new indexes, read models, or partitioning, we capture:

- top SQL statements from `pg_stat_statements` on PostgreSQL;
- `EXPLAIN` plans for known hot paths in `apps/server`;
- a tenant-scoped snapshot that can be compared over time.

The current baseline task targets these hot paths first:

- `root.users.count`
- `root.users.page`
- `root.dashboard_stats.users_snapshot`
- `root.dashboard_stats.posts_snapshot`
- `root.dashboard_stats.orders_snapshot`
- `root.recent_activity.recent_users`

## Collection Task

Task implementation:

- [`db_baseline.rs`](/C:/проекты/RusTok/apps/server/src/tasks/db_baseline.rs)

Preflight:

- ensure PostgreSQL is reachable on `localhost:5432`;
- ensure the target database (`rustok_dev` in development) is up;
- if `cargo loco` is unavailable locally, use the built server binary task runner from `apps/server`.

Run with default active tenant:

```powershell
cargo loco task --name db_baseline
```

Run for a specific tenant and save to file:

```powershell
cargo loco task --name db_baseline --args "tenant_id=<uuid> top_n=15 output=tmp/db-baseline.json"
```

Binary fallback from `apps/server`:

```powershell
..\..\target\debug\rustok-server.exe task db_baseline output:../../tmp/db-baseline.json
```

## Output

The task emits JSON with:

- `generated_at`
- `backend`
- `tenant_id`
- `top_n`
- `pg_stat_statements`
- `explain_plans`

On PostgreSQL, `pg_stat_statements.available=true` means the extension is enabled and readable.

On SQLite, the task still captures `EXPLAIN QUERY PLAN` for the hot paths, but `pg_stat_statements` is reported as unavailable by design.

## How To Read It

Focus on three questions:

1. Which statements dominate total execution time?
2. Do the hot-path plans use the indexes we expect?
3. Is the expensive work coming from `COUNT`, sorting, JSON extraction, or broad scans?

Use the result to decide one of four actions for each path:

- leave as is
- rewrite query
- add or adjust index
- move to cache or read model

## Workflow

Recommended sequence:

1. Warm the target path with representative traffic.
2. Run `db_baseline` for the tenant you care about.
3. Save the JSON artifact for the current date.
4. Make the query/index change.
5. Re-run the same task and compare plans plus top statements.

## Notes

- PostgreSQL evidence is only useful if `pg_stat_statements` is enabled on the server.
- The task is intentionally read-only.
- The task does not decide optimizations for you; it creates the evidence bundle needed for architecture decisions.
