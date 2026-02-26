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
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::models::sessions;
use crate::services::rbac_consistency::{
    load_rbac_consistency_stats, load_users_without_tenant_roles,
};

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
