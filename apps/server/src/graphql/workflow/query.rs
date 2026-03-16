use async_graphql::{Context, Object, Result};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::context::AuthContext;
use crate::graphql::errors::GraphQLError;
use crate::services::rbac_service::RbacService;
use rustok_core::Permission;
use rustok_workflow::WorkflowService;

use super::types::*;

#[derive(Default)]
pub struct WorkflowQuery;

#[Object]
impl WorkflowQuery {
    /// List all workflows for a tenant
    async fn workflows(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
    ) -> Result<Vec<GqlWorkflowSummary>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <async_graphql::FieldError as GraphQLError>::unauthenticated())?;

        let has_perm = RbacService::has_any_permission(
            db,
            &tenant_id,
            &auth.user_id,
            &[Permission::WORKFLOWS_LIST, Permission::WORKFLOWS_MANAGE],
        )
        .await
        .map_err(|e| <async_graphql::FieldError as GraphQLError>::internal_error(&e.to_string()))?;

        if !has_perm {
            return Err(<async_graphql::FieldError as GraphQLError>::permission_denied(
                "Permission denied: workflows:list required",
            ));
        }

        let service = WorkflowService::new(db.clone());
        let workflows = service
            .list(tenant_id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(workflows.into_iter().map(Into::into).collect())
    }

    /// Get a single workflow with its steps
    async fn workflow(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        id: Uuid,
    ) -> Result<Option<GqlWorkflow>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <async_graphql::FieldError as GraphQLError>::unauthenticated())?;

        let has_perm = RbacService::has_any_permission(
            db,
            &tenant_id,
            &auth.user_id,
            &[Permission::WORKFLOWS_READ, Permission::WORKFLOWS_MANAGE],
        )
        .await
        .map_err(|e| <async_graphql::FieldError as GraphQLError>::internal_error(&e.to_string()))?;

        if !has_perm {
            return Err(<async_graphql::FieldError as GraphQLError>::permission_denied(
                "Permission denied: workflows:read required",
            ));
        }

        let service = WorkflowService::new(db.clone());
        match service.get(tenant_id, id).await {
            Ok(w) => Ok(Some(w.into())),
            Err(rustok_workflow::WorkflowError::NotFound(_)) => Ok(None),
            Err(e) => Err(async_graphql::Error::new(e.to_string())),
        }
    }

    /// List executions for a workflow
    async fn workflow_executions(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        workflow_id: Uuid,
    ) -> Result<Vec<GqlWorkflowExecution>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <async_graphql::FieldError as GraphQLError>::unauthenticated())?;

        let has_perm = RbacService::has_any_permission(
            db,
            &tenant_id,
            &auth.user_id,
            &[
                Permission::WORKFLOWS_LIST,
                Permission::WORKFLOWS_MANAGE,
            ],
        )
        .await
        .map_err(|e| <async_graphql::FieldError as GraphQLError>::internal_error(&e.to_string()))?;

        if !has_perm {
            return Err(<async_graphql::FieldError as GraphQLError>::permission_denied(
                "Permission denied: workflows:list required",
            ));
        }

        let service = WorkflowService::new(db.clone());
        let executions = service
            .list_executions(tenant_id, workflow_id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(executions.into_iter().map(Into::into).collect())
    }

    /// Get a single execution by id
    async fn workflow_execution(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        execution_id: Uuid,
    ) -> Result<Option<GqlWorkflowExecution>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <async_graphql::FieldError as GraphQLError>::unauthenticated())?;

        let has_perm = RbacService::has_any_permission(
            db,
            &tenant_id,
            &auth.user_id,
            &[Permission::WORKFLOWS_READ, Permission::WORKFLOWS_MANAGE],
        )
        .await
        .map_err(|e| <async_graphql::FieldError as GraphQLError>::internal_error(&e.to_string()))?;

        if !has_perm {
            return Err(<async_graphql::FieldError as GraphQLError>::permission_denied(
                "Permission denied: workflows:read required",
            ));
        }

        let service = WorkflowService::new(db.clone());
        match service.get_execution(tenant_id, execution_id).await {
            Ok(e) => Ok(Some(e.into())),
            Err(rustok_workflow::WorkflowError::ExecutionNotFound(_)) => Ok(None),
            Err(e) => Err(async_graphql::Error::new(e.to_string())),
        }
    }
}
