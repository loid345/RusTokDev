use std::collections::HashSet;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};
use thiserror::Error;

use rustok_core::{ModuleContext, ModuleRegistry};

use crate::models::_entities::tenant_modules;
use crate::models::_entities::tenant_modules::Entity as TenantModulesEntity;

pub struct ModuleLifecycleService;

#[derive(Debug, Error)]
pub enum ToggleModuleError {
    #[error("Unknown module")]
    UnknownModule,
    /// Core modules are part of the platform kernel and can never be disabled.
    /// See `ModuleKind::Core` and `DECISIONS/2026-02-19-module-kind-core-vs-optional.md`.
    #[error("Module '{0}' is a core platform module and cannot be disabled")]
    CoreModuleCannotBeDisabled(String),
    #[error("Missing module dependencies: {0}")]
    MissingDependencies(String),
    #[error("Module is required by: {0}")]
    HasDependents(String),
    #[error("Database error: {0}")]
    Database(#[from] DbErr),
    #[error("Module hook failed: {0}")]
    HookFailed(String),
}

#[derive(Debug, Error)]
pub enum UpdateModuleSettingsError {
    #[error("Unknown module")]
    UnknownModule,
    #[error("Module '{0}' is not enabled for this tenant")]
    ModuleNotEnabled(String),
    #[error("Module settings must be a JSON object")]
    InvalidSettings,
    #[error("Database error: {0}")]
    Database(#[from] DbErr),
}

impl ModuleLifecycleService {
    pub async fn toggle_module(
        db: &DatabaseConnection,
        registry: &ModuleRegistry,
        tenant_id: uuid::Uuid,
        module_slug: &str,
        enabled: bool,
    ) -> Result<tenant_modules::Model, ToggleModuleError> {
        let Some(module_impl) = registry.get(module_slug) else {
            return Err(ToggleModuleError::UnknownModule);
        };

        if !enabled && registry.is_core(module_slug) {
            return Err(ToggleModuleError::CoreModuleCannotBeDisabled(
                module_slug.to_string(),
            ));
        }

        let enabled_modules = TenantModulesEntity::find_enabled(db, tenant_id).await?;
        let enabled_set: HashSet<String> = enabled_modules.into_iter().collect();

        if enabled {
            let missing: Vec<String> = module_impl
                .dependencies()
                .iter()
                .filter(|dependency| !enabled_set.contains(**dependency))
                .map(|dependency| (*dependency).to_string())
                .collect();

            if !missing.is_empty() {
                return Err(ToggleModuleError::MissingDependencies(missing.join(", ")));
            }
        } else {
            let dependents: Vec<String> = registry
                .list()
                .into_iter()
                .filter(|module| enabled_set.contains(module.slug()))
                .filter(|module| module.dependencies().contains(&module_slug))
                .map(|module| module.slug().to_string())
                .collect();

            if !dependents.is_empty() {
                return Err(ToggleModuleError::HasDependents(dependents.join(", ")));
            }
        }

        let (module, previous_enabled, changed) =
            Self::persist_module_state(db, tenant_id, module_slug, enabled).await?;

        if !changed {
            return Ok(module);
        }

        let module_ctx = ModuleContext {
            db,
            tenant_id,
            config: &module.settings,
        };

        let hook_result = if enabled {
            module_impl.on_enable(module_ctx).await
        } else {
            module_impl.on_disable(module_ctx).await
        };

        if let Err(err) = hook_result {
            tracing::error!(
                "Module hook failed for {} (enabled={}): {}. Reverting to {}",
                module_slug,
                enabled,
                err,
                previous_enabled
            );

            let _ =
                Self::persist_module_state(db, tenant_id, module_slug, previous_enabled).await?;
            return Err(ToggleModuleError::HookFailed(err.to_string()));
        }

        Ok(module)
    }

    pub async fn update_module_settings(
        db: &DatabaseConnection,
        registry: &ModuleRegistry,
        tenant_id: uuid::Uuid,
        module_slug: &str,
        settings: serde_json::Value,
    ) -> Result<tenant_modules::Model, UpdateModuleSettingsError> {
        let Some(_module_impl) = registry.get(module_slug) else {
            return Err(UpdateModuleSettingsError::UnknownModule);
        };

        if !settings.is_object() {
            return Err(UpdateModuleSettingsError::InvalidSettings);
        }

        let existing = TenantModulesEntity::find()
            .filter(tenant_modules::Column::TenantId.eq(tenant_id))
            .filter(tenant_modules::Column::ModuleSlug.eq(module_slug))
            .one(db)
            .await?;

        let is_core = registry.is_core(module_slug);

        match existing {
            Some(model) => {
                if !is_core && !model.enabled {
                    return Err(UpdateModuleSettingsError::ModuleNotEnabled(
                        module_slug.to_string(),
                    ));
                }

                let was_enabled = model.enabled;
                let mut active: tenant_modules::ActiveModel = model.into();
                active.enabled = Set(is_core || was_enabled);
                active.settings = Set(settings);
                Ok(active.update(db).await?)
            }
            None if is_core => {
                let module = tenant_modules::ActiveModel {
                    id: Set(rustok_core::generate_id()),
                    tenant_id: Set(tenant_id),
                    module_slug: Set(module_slug.to_string()),
                    enabled: Set(true),
                    settings: Set(settings),
                    created_at: sea_orm::ActiveValue::NotSet,
                    updated_at: sea_orm::ActiveValue::NotSet,
                }
                .insert(db)
                .await?;

                Ok(module)
            }
            None => Err(UpdateModuleSettingsError::ModuleNotEnabled(
                module_slug.to_string(),
            )),
        }
    }

    async fn persist_module_state(
        db: &DatabaseConnection,
        tenant_id: uuid::Uuid,
        module_slug: &str,
        enabled: bool,
    ) -> Result<(tenant_modules::Model, bool, bool), DbErr> {
        let module_slug = module_slug.to_string();

        db.transaction::<_, (tenant_modules::Model, bool, bool), DbErr>(move |txn| {
            let module_slug = module_slug.clone();
            Box::pin(async move {
                let existing = TenantModulesEntity::find()
                    .filter(tenant_modules::Column::TenantId.eq(tenant_id))
                    .filter(tenant_modules::Column::ModuleSlug.eq(&module_slug))
                    .one(txn)
                    .await?;

                match existing {
                    Some(model) => {
                        if model.enabled == enabled {
                            return Ok((model.clone(), model.enabled, false));
                        }

                        let previous_enabled = model.enabled;
                        let mut active: tenant_modules::ActiveModel = model.into();
                        active.enabled = Set(enabled);
                        let updated = active.update(txn).await?;
                        Ok((updated, previous_enabled, true))
                    }
                    None => {
                        let module = tenant_modules::ActiveModel {
                            id: Set(rustok_core::generate_id()),
                            tenant_id: Set(tenant_id),
                            module_slug: Set(module_slug),
                            enabled: Set(enabled),
                            settings: Set(serde_json::json!({})),
                            created_at: sea_orm::ActiveValue::NotSet,
                            updated_at: sea_orm::ActiveValue::NotSet,
                        }
                        .insert(txn)
                        .await?;

                        Ok((module, !enabled, true))
                    }
                }
            })
        })
        .await
        .map_err(|err| match err {
            sea_orm::TransactionError::Connection(db_err) => db_err,
            sea_orm::TransactionError::Transaction(db_err) => db_err,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{ModuleLifecycleService, UpdateModuleSettingsError};
    use crate::models::_entities::tenant_modules;
    use crate::models::tenants;
    use crate::modules::build_registry;
    use migration::Migrator;
    use rustok_core::ModuleRegistry;
    use rustok_index::IndexModule;
    use rustok_rbac::RbacModule;
    use rustok_tenant::TenantModule;
    use rustok_test_utils::db::setup_test_db_with_migrations;
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
    use serial_test::serial;

    fn build_test_registry() -> ModuleRegistry {
        ModuleRegistry::new()
            .register(IndexModule)
            .register(TenantModule)
            .register(RbacModule)
    }

    #[test]
    fn disable_core_module_is_rejected() {
        let registry = build_test_registry();
        assert!(registry.is_core("tenant"));
        assert!(registry.is_core("rbac"));
        assert!(registry.is_core("index"));
    }

    #[test]
    fn disable_optional_module_is_allowed() {
        let registry = build_test_registry();
        assert!(!registry.is_core("content"));
        assert!(!registry.is_core("commerce"));
        assert!(!registry.is_core("blog"));
    }

    #[tokio::test]
    #[serial]
    async fn update_module_settings_rejects_disabled_optional_module() {
        let db = setup_test_db_with_migrations::<Migrator>().await;
        let registry = build_registry();
        let tenant =
            tenants::ActiveModel::new("Module settings tenant", "module-settings-disabled")
                .insert(&db)
                .await
                .expect("insert tenant");

        let result = ModuleLifecycleService::update_module_settings(
            &db,
            &registry,
            tenant.id,
            "content",
            serde_json::json!({ "postsPerPage": 20 }),
        )
        .await;

        assert!(matches!(
            result,
            Err(UpdateModuleSettingsError::ModuleNotEnabled(slug)) if slug == "content"
        ));
    }

    #[tokio::test]
    #[serial]
    async fn update_module_settings_persists_enabled_optional_module() {
        let db = setup_test_db_with_migrations::<Migrator>().await;
        let registry = build_registry();
        let tenant = tenants::ActiveModel::new("Module settings tenant", "module-settings-enabled")
            .insert(&db)
            .await
            .expect("insert tenant");

        ModuleLifecycleService::toggle_module(&db, &registry, tenant.id, "content", true)
            .await
            .expect("enable content module");

        let updated = ModuleLifecycleService::update_module_settings(
            &db,
            &registry,
            tenant.id,
            "content",
            serde_json::json!({ "postsPerPage": 20 }),
        )
        .await
        .expect("update module settings");

        assert!(updated.enabled);
        assert_eq!(updated.settings["postsPerPage"], serde_json::json!(20));
    }

    #[tokio::test]
    #[serial]
    async fn update_module_settings_upserts_core_module_row() {
        let db = setup_test_db_with_migrations::<Migrator>().await;
        let registry = build_registry();
        let tenant = tenants::ActiveModel::new("Module settings tenant", "module-settings-core")
            .insert(&db)
            .await
            .expect("insert tenant");

        let updated = ModuleLifecycleService::update_module_settings(
            &db,
            &registry,
            tenant.id,
            "tenant",
            serde_json::json!({ "workspaceName": "Acme" }),
        )
        .await
        .expect("update core module settings");

        assert!(updated.enabled);
        assert_eq!(updated.module_slug, "tenant");

        let stored = tenant_modules::Entity::find()
            .filter(tenant_modules::Column::TenantId.eq(tenant.id))
            .filter(tenant_modules::Column::ModuleSlug.eq("tenant"))
            .one(&db)
            .await
            .expect("load stored core settings")
            .expect("tenant_modules row");
        assert_eq!(stored.settings["workspaceName"], serde_json::json!("Acme"));
        assert!(stored.enabled);
    }
}
