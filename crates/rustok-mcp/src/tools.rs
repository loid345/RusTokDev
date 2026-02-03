use schemars::JsonSchema;
use serde::Serialize;

use rustok_core::registry::ModuleRegistry;

#[derive(Clone)]
pub struct McpState {
    pub registry: ModuleRegistry,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ModuleInfo {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ModuleListResponse {
    pub modules: Vec<ModuleInfo>,
}

#[rmcp::tool]
pub async fn list_modules(state: &'static McpState) -> ModuleListResponse {
    let modules = state
        .registry
        .list()
        .into_iter()
        .map(|module| ModuleInfo {
            slug: module.slug().to_string(),
            name: module.name().to_string(),
            description: module.description().to_string(),
            version: module.version().to_string(),
            dependencies: module
                .dependencies()
                .iter()
                .map(|dep| dep.to_string())
                .collect(),
        })
        .collect();

    ModuleListResponse { modules }
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ModuleLookupResponse {
    pub slug: String,
    pub exists: bool,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ModuleLookupRequest {
    pub slug: String,
}

#[rmcp::tool]
pub async fn module_exists(
    state: &'static McpState,
    input: ModuleLookupRequest,
) -> ModuleLookupResponse {
    let exists = state.registry.contains(&input.slug);

    ModuleLookupResponse {
        slug: input.slug,
        exists,
    }
}
