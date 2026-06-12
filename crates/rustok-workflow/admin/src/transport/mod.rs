mod graphql_adapter;
mod native_server_adapter;

use std::fmt::{Display, Formatter};

use crate::core::{WorkflowAdminTransportContext, WorkflowTemplateCreateCommand};
use crate::model::{WorkflowSummary, WorkflowTemplateDto};

#[derive(Debug, Clone)]
pub enum TransportError {
    NativeAndGraphql { native: String, graphql: String },
}

impl Display for TransportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NativeAndGraphql { native, graphql } => write!(
                f,
                "native path failed: {native}; graphql path failed: {graphql}"
            ),
        }
    }
}

impl std::error::Error for TransportError {}

fn combine_native_and_graphql_error(
    native: leptos::prelude::ServerFnError,
    graphql: graphql_adapter::TransportError,
) -> TransportError {
    TransportError::NativeAndGraphql {
        native: native.to_string(),
        graphql: graphql.to_string(),
    }
}

pub async fn fetch_workflows(
    context: WorkflowAdminTransportContext,
) -> Result<Vec<WorkflowSummary>, TransportError> {
    match native_server_adapter::fetch_workflows_native().await {
        Ok(response) => Ok(response),
        Err(native_error) => graphql_adapter::fetch_workflows(context.token, context.tenant_slug)
            .await
            .map_err(|graphql_error| combine_native_and_graphql_error(native_error, graphql_error)),
    }
}

pub async fn fetch_templates(
    context: WorkflowAdminTransportContext,
) -> Result<Vec<WorkflowTemplateDto>, TransportError> {
    match native_server_adapter::fetch_templates_native().await {
        Ok(response) => Ok(response),
        Err(native_error) => graphql_adapter::fetch_templates(context.token, context.tenant_slug)
            .await
            .map_err(|graphql_error| combine_native_and_graphql_error(native_error, graphql_error)),
    }
}

pub async fn create_from_template(
    context: WorkflowAdminTransportContext,
    command: WorkflowTemplateCreateCommand,
) -> Result<String, TransportError> {
    match native_server_adapter::create_from_template_native(
        command.template_id.clone(),
        command.workflow_name.clone(),
    )
    .await
    {
        Ok(response) => Ok(response),
        Err(native_error) => graphql_adapter::create_from_template(
            context.token,
            context.tenant_slug,
            command.template_id,
            command.workflow_name,
        )
        .await
        .map_err(|graphql_error| combine_native_and_graphql_error(native_error, graphql_error)),
    }
}
