//! Cleanup Task
//!
//! Removes old sessions and temporary data.
//! Run with: `cargo loco task --name cleanup --args "sessions"`

use async_trait::async_trait;
use chrono::Utc;
use loco_rs::{
    app::AppContext,
    task::{Task, TaskInfo, Vars},
    Result,
};
use rustok_core::UserRole;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbBackend, EntityTrait, QueryFilter, Statement, TryGetable,
};

use crate::models::sessions;

/// Cleanup task for maintenance operations
pub struct CleanupTask;

#[async_trait]
impl Task for CleanupTask {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "cleanup".to_string(),
            detail: "Remove old sessions and temporary data".to_string(),
        }
    }

    async fn run(&self, ctx: &AppContext, vars: &Vars) -> Result<()> {
        let target = vars.cli.get("target").map_or("", String::as_str);

        match target {
            "sessions" => {
                tracing::info!("Cleaning up expired sessions...");
                let now = Utc::now();
                let result = sessions::Entity::delete_many()
                    .filter(sessions::Column::ExpiresAt.lt(now))
                    .exec(&ctx.db)
                    .await?;

                tracing::info!(deleted = result.rows_affected, "Session cleanup complete");
            }
            "cache" => {
                tracing::info!("Clearing temporary cache entries...");
                // Cache cleanup would go here
                tracing::info!("Cache cleanup complete");
            }
            "rbac-report" => {
                let stats = load_rbac_consistency_stats(ctx).await?;
                tracing::info!(
                    users_without_roles_total = stats.users_without_roles_total,
                    orphan_user_roles_total = stats.orphan_user_roles_total,
                    orphan_role_permissions_total = stats.orphan_role_permissions_total,
                    "RBAC consistency report"
                );
            }
            "rbac-backfill" => {
                let before = load_rbac_consistency_stats(ctx).await?;
                tracing::info!(
                    users_without_roles_total = before.users_without_roles_total,
                    orphan_user_roles_total = before.orphan_user_roles_total,
                    orphan_role_permissions_total = before.orphan_role_permissions_total,
                    "RBAC consistency before backfill"
                );

                let users_without_tenant_roles = load_users_without_tenant_roles(ctx).await?;
                for user in &users_without_tenant_roles {
                    crate::services::auth::AuthService::assign_role_permissions(
                        &ctx.db,
                        &user.id,
                        &user.tenant_id,
                        user.role.clone(),
                    )
                    .await?;
                }

                let after = load_rbac_consistency_stats(ctx).await?;
                tracing::info!(
                    fixed_users = users_without_tenant_roles.len(),
                    users_without_roles_total = after.users_without_roles_total,
                    orphan_user_roles_total = after.orphan_user_roles_total,
                    orphan_role_permissions_total = after.orphan_role_permissions_total,
                    "RBAC backfill complete"
                );
            }
            "" => {
                tracing::info!("Running full cleanup...");
                let now = Utc::now();
                let result = sessions::Entity::delete_many()
                    .filter(sessions::Column::ExpiresAt.lt(now))
                    .exec(&ctx.db)
                    .await?;

                tracing::info!(deleted = result.rows_affected, "Full cleanup complete");
            }
            _ => {
                tracing::warn!("Unknown cleanup target: {}", target);
                tracing::info!(
                    "Available targets: sessions, cache, rbac-report, rbac-backfill, or empty for full"
                );
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct RbacConsistencyStats {
    users_without_roles_total: i64,
    orphan_user_roles_total: i64,
    orphan_role_permissions_total: i64,
}

#[derive(Debug, Clone)]
struct UserBackfillCandidate {
    id: uuid::Uuid,
    tenant_id: uuid::Uuid,
    role: UserRole,
}

async fn load_rbac_consistency_stats(ctx: &AppContext) -> Result<RbacConsistencyStats> {
    let row = ctx
        .db
        .query_one(Statement::from_string(
            DbBackend::Postgres,
            "SELECT
                 (SELECT COUNT(*)
                  FROM users u
                  LEFT JOIN user_roles ur ON ur.user_id = u.id
                  WHERE ur.id IS NULL) AS users_without_roles_total,
                 (SELECT COUNT(*)
                  FROM user_roles ur
                  LEFT JOIN roles r ON r.id = ur.role_id
                  WHERE r.id IS NULL) AS orphan_user_roles_total,
                 (SELECT COUNT(*)
                  FROM role_permissions rp
                  LEFT JOIN permissions p ON p.id = rp.permission_id
                  WHERE p.id IS NULL) AS orphan_role_permissions_total"
                .to_string(),
        ))
        .await?
        .ok_or_else(|| loco_rs::Error::string("RBAC consistency stats query returned no rows"))?;

    Ok(RbacConsistencyStats {
        users_without_roles_total: read_metric(&row, "users_without_roles_total")?,
        orphan_user_roles_total: read_metric(&row, "orphan_user_roles_total")?,
        orphan_role_permissions_total: read_metric(&row, "orphan_role_permissions_total")?,
    })
}

async fn load_users_without_tenant_roles(ctx: &AppContext) -> Result<Vec<UserBackfillCandidate>> {
    let rows = ctx
        .db
        .query_all(Statement::from_string(
            DbBackend::Postgres,
            "SELECT u.id, u.tenant_id, u.role
             FROM users u
             WHERE NOT EXISTS (
                 SELECT 1
                 FROM user_roles ur
                 JOIN roles r ON r.id = ur.role_id
                 WHERE ur.user_id = u.id AND r.tenant_id = u.tenant_id
             )"
            .to_string(),
        ))
        .await?;

    let mut result = Vec::with_capacity(rows.len());
    for row in rows {
        let role_raw = row.try_get::<String>("", "role")?;
        let role = role_raw.parse::<UserRole>().map_err(|error| {
            loco_rs::Error::string(format!(
                "invalid user role '{role_raw}' for backfill: {error}"
            ))
        })?;

        result.push(UserBackfillCandidate {
            id: row.try_get("", "id")?,
            tenant_id: row.try_get("", "tenant_id")?,
            role,
        });
    }

    Ok(result)
}

fn read_metric(row: &sea_orm::QueryResult, column: &str) -> Result<i64> {
    row.try_get::<i64>("", column).map_err(Into::into)
}
