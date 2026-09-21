use chrono::Utc;
use sea_orm::sea_query::{Expr, ExprTrait, Func, OnConflict};
use sea_orm::{
    ActiveValue::Set, DatabaseConnection, DatabaseTransaction, EntityTrait,
};
use tracing::instrument;
use uuid::Uuid;

use rustok_core::{Action, Resource, SecurityContext};

use crate::dto::ForumUserStatsResponse;
use crate::entities::forum_user_stat;
use crate::error::ForumResult;
use crate::services::rbac::enforce_scope;

pub struct UserStatsService {
    db: DatabaseConnection,
}

impl UserStatsService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    #[instrument(skip(self, security))]
    pub async fn get(
        &self,
        tenant_id: Uuid,
        security: SecurityContext,
        user_id: Uuid,
    ) -> ForumResult<ForumUserStatsResponse> {
        enforce_scope(&security, Resource::ForumTopics, Action::Read)?;
        let row = forum_user_stat::Entity::find_by_id((tenant_id, user_id))
            .one(&self.db)
            .await?;

        Ok(match row {
            Some(row) => ForumUserStatsResponse {
                user_id: row.user_id,
                topic_count: row.topic_count,
                reply_count: row.reply_count,
                solution_count: row.solution_count,
                updated_at: row.updated_at.to_rfc3339(),
            },
            None => ForumUserStatsResponse {
                user_id,
                topic_count: 0,
                reply_count: 0,
                solution_count: 0,
                updated_at: Utc::now().to_rfc3339(),
            },
        })
    }

    pub(crate) async fn adjust_topic_count_in_tx(
        txn: &DatabaseTransaction,
        tenant_id: Uuid,
        user_id: Option<Uuid>,
        delta: i32,
    ) -> ForumResult<()> {
        Self::adjust_counts_in_tx(txn, tenant_id, user_id, delta, 0, 0).await
    }

    pub(crate) async fn adjust_reply_count_in_tx(
        txn: &DatabaseTransaction,
        tenant_id: Uuid,
        user_id: Option<Uuid>,
        delta: i32,
    ) -> ForumResult<()> {
        Self::adjust_counts_in_tx(txn, tenant_id, user_id, 0, delta, 0).await
    }

    pub(crate) async fn adjust_solution_count_in_tx(
        txn: &DatabaseTransaction,
        tenant_id: Uuid,
        user_id: Option<Uuid>,
        delta: i32,
    ) -> ForumResult<()> {
        Self::adjust_counts_in_tx(txn, tenant_id, user_id, 0, 0, delta).await
    }

    async fn adjust_counts_in_tx(
        txn: &DatabaseTransaction,
        tenant_id: Uuid,
        user_id: Option<Uuid>,
        topic_delta: i32,
        reply_delta: i32,
        solution_delta: i32,
    ) -> ForumResult<()> {
        let Some(user_id) = user_id else {
            return Ok(());
        };

        let now = Utc::now();
        let topic_value = Expr::col(forum_user_stat::Column::TopicCount);
        let reply_value = Expr::col(forum_user_stat::Column::ReplyCount);
        let solution_value = Expr::col(forum_user_stat::Column::SolutionCount);

        forum_user_stat::Entity::insert(forum_user_stat::ActiveModel {
            tenant_id: Set(tenant_id),
            user_id: Set(user_id),
            topic_count: Set(topic_delta.max(0)),
            reply_count: Set(reply_delta.max(0)),
            solution_count: Set(solution_delta.max(0)),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
        })
        .on_conflict(
            OnConflict::columns([
                forum_user_stat::Column::TenantId,
                forum_user_stat::Column::UserId,
            ])
            .values([
                (
                    forum_user_stat::Column::TopicCount,
                    Func::greatest([
                        topic_value.add(topic_delta).into(),
                        Expr::val(0).into(),
                    ])
                    .into(),
                ),
                (
                    forum_user_stat::Column::ReplyCount,
                    Func::greatest([
                        reply_value.add(reply_delta).into(),
                        Expr::val(0).into(),
                    ])
                    .into(),
                ),
                (
                    forum_user_stat::Column::SolutionCount,
                    Func::greatest([
                        solution_value.add(solution_delta).into(),
                        Expr::val(0).into(),
                    ])
                    .into(),
                ),
                (
                    forum_user_stat::Column::UpdatedAt,
                    Expr::val(now).into(),
                ),
            ])
            .to_owned(),
        )
        .exec(txn)
        .await?;

        Ok(())
    }

    pub(crate) async fn decrement_topic_thread_in_tx(
        txn: &DatabaseTransaction,
        tenant_id: Uuid,
        topic_author_id: Option<Uuid>,
        reply_author_ids: &[Option<Uuid>],
        solution_author_id: Option<Uuid>,
    ) -> ForumResult<()> {
        Self::adjust_topic_count_in_tx(txn, tenant_id, topic_author_id, -1).await?;
        for reply_author_id in reply_author_ids {
            Self::adjust_reply_count_in_tx(txn, tenant_id, *reply_author_id, -1).await?;
        }
        Self::adjust_solution_count_in_tx(txn, tenant_id, solution_author_id, -1).await?;
        Ok(())
    }
}
