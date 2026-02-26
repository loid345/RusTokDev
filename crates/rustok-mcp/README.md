# rustok-mcp

`rustok-mcp` is the MCP adapter crate for RusToK. It uses the official Rust SDK (`rmcp`) and
exposes RusToK tools/resources by wiring them to platform services.

## Goals

- Keep MCP support as a thin adapter layer.
- Reuse the `rmcp` SDK for protocol, schema, and transport handling.
- Expose domain operations via typed tools with generated JSON Schemas.
- Return tool payloads in a standard response envelope (`McpToolResponse`).

## Tooling overview

### Module tools
- `list_modules`: list all registered modules.
- `query_modules`: list modules with pagination and filters.
- `module_exists` / `module_details`: module lookups by slug.
- `content_module` / `blog_module` / `forum_module` / `pages_module`: domain module metadata.
- `mcp_health`: readiness snapshot for MCP server.

### Alloy scripting tools (available when `AlloyMcpState` is configured)
- `alloy_list_scripts`: list scripts with optional status filter.
- `alloy_get_script`: get a script by name or UUID.
- `alloy_create_script`: create a new Rhai script.
- `alloy_update_script`: update an existing script (code, description, status).
- `alloy_delete_script`: delete a script by UUID.
- `alloy_validate_script`: validate Rhai syntax without executing.
- `alloy_run_script`: execute a script manually with params and optional entity context.
- `alloy_list_entity_types`: list known entity types.
- `alloy_script_helpers`: list available helper functions with signatures.

## Quick start

```rust
use rustok_core::registry::ModuleRegistry;
use rustok_mcp::McpServerConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let registry = ModuleRegistry::new();
    let config = McpServerConfig::new(registry);
    rustok_mcp::serve_stdio(config).await
}
```

To enable a tool allow-list:

```rust
use std::collections::HashSet;

use rustok_core::registry::ModuleRegistry;
use rustok_mcp::McpServerConfig;

let registry = ModuleRegistry::new();
let enabled = HashSet::from([
    "list_modules".to_string(),
    "mcp_health".to_string(),
]);
let config = McpServerConfig::with_enabled_tools(registry, enabled);
```

To enable Alloy scripting tools, construct the server with `with_alloy`:

```rust
use std::sync::Arc;
use rustok_core::registry::ModuleRegistry;
use rustok_mcp::{RusToKMcpServer, AlloyMcpState};
use alloy_scripting::{InMemoryStorage, ScriptOrchestrator, create_default_engine};

let registry = ModuleRegistry::new();
let engine = Arc::new(create_default_engine());
let storage = Arc::new(InMemoryStorage::new());
let orchestrator = Arc::new(ScriptOrchestrator::new(engine.clone(), storage.clone()));
let alloy = AlloyMcpState::new(storage, engine, orchestrator);
let server = RusToKMcpServer::with_alloy(registry, alloy);
```

For more details see `docs/implementation-plan.md`.


## Взаимодействие
- встроенный binary target `rustok-mcp-server`
- crates/rustok-core (registry/services)
- доменные модули через service layer

## Документация
- Локальная документация: `./docs/`
- План реализации MCP-модуля: `./docs/implementation-plan.md`
- Общая документация платформы: `/docs`

## Паспорт компонента
- **Роль в системе:** Адаптер MCP-инструментов поверх Rust SDK (`rmcp`) для RusToK сервисов.
- **Основные данные/ответственность:** бизнес-логика и API данного компонента; структура кода и документации в корне компонента.
- **Взаимодействует с:**
  - встроенный binary target `rustok-mcp-server`
  - crates/rustok-core (registry/services)
  - доменные модули через service layer
- **Точки входа:**
  - `crates/rustok-mcp/src/lib.rs`
- **Локальная документация:** `./docs/`
- **Глобальная документация платформы:** `/docs/`

