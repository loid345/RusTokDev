use rustok_core::UserRole;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyRoleResolution {
    pub role: Option<UserRole>,
    pub cache_hit: bool,
}

#[async_trait::async_trait]
pub trait LegacyRoleStore {
    type Error;

    async fn load_legacy_role(
        &self,
        tenant_id: &uuid::Uuid,
        user_id: &uuid::Uuid,
    ) -> Result<Option<UserRole>, Self::Error>;
}

#[async_trait::async_trait]
pub trait LegacyRoleCache {
    async fn get(&self, tenant_id: &uuid::Uuid, user_id: &uuid::Uuid) -> Option<UserRole>;

    async fn insert(&self, tenant_id: &uuid::Uuid, user_id: &uuid::Uuid, role: UserRole);

    async fn invalidate(&self, tenant_id: &uuid::Uuid, user_id: &uuid::Uuid);
}

pub async fn resolve_legacy_role_with_cache<S, C>(
    store: &S,
    cache: &C,
    tenant_id: &uuid::Uuid,
    user_id: &uuid::Uuid,
) -> Result<LegacyRoleResolution, S::Error>
where
    S: LegacyRoleStore,
    C: LegacyRoleCache,
{
    if let Some(cached_role) = cache.get(tenant_id, user_id).await {
        return Ok(LegacyRoleResolution {
            role: Some(cached_role),
            cache_hit: true,
        });
    }

    let role = store.load_legacy_role(tenant_id, user_id).await?;
    if let Some(role) = role.as_ref() {
        cache.insert(tenant_id, user_id, role.clone()).await;
    }

    Ok(LegacyRoleResolution {
        role,
        cache_hit: false,
    })
}

pub async fn invalidate_cached_legacy_role<C: LegacyRoleCache>(
    cache: &C,
    tenant_id: &uuid::Uuid,
    user_id: &uuid::Uuid,
) {
    cache.invalidate(tenant_id, user_id).await;
}

#[cfg(test)]
mod tests {
    use super::{
        invalidate_cached_legacy_role, resolve_legacy_role_with_cache, LegacyRoleCache,
        LegacyRoleStore,
    };
    use async_trait::async_trait;
    use rustok_core::UserRole;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    type LegacyRoleCacheKey = (uuid::Uuid, uuid::Uuid);
    type LegacyRoleCacheMap = HashMap<LegacyRoleCacheKey, UserRole>;

    #[derive(Default)]
    struct StubCache {
        values: Arc<Mutex<LegacyRoleCacheMap>>,
    }

    struct StubStore {
        role: Option<UserRole>,
        load_calls: Arc<Mutex<u64>>,
    }

    #[async_trait]
    impl LegacyRoleCache for StubCache {
        async fn get(&self, tenant_id: &uuid::Uuid, user_id: &uuid::Uuid) -> Option<UserRole> {
            self.values
                .lock()
                .await
                .get(&(*tenant_id, *user_id))
                .cloned()
        }

        async fn insert(&self, tenant_id: &uuid::Uuid, user_id: &uuid::Uuid, role: UserRole) {
            self.values
                .lock()
                .await
                .insert((*tenant_id, *user_id), role);
        }

        async fn invalidate(&self, tenant_id: &uuid::Uuid, user_id: &uuid::Uuid) {
            self.values.lock().await.remove(&(*tenant_id, *user_id));
        }
    }

    #[async_trait]
    impl LegacyRoleStore for StubStore {
        type Error = String;

        async fn load_legacy_role(
            &self,
            _tenant_id: &uuid::Uuid,
            _user_id: &uuid::Uuid,
        ) -> Result<Option<UserRole>, Self::Error> {
            *self.load_calls.lock().await += 1;
            Ok(self.role.clone())
        }
    }

    #[tokio::test]
    async fn returns_cached_legacy_role_on_second_lookup() {
        let tenant_id = uuid::Uuid::new_v4();
        let user_id = uuid::Uuid::new_v4();
        let load_calls = Arc::new(Mutex::new(0));
        let store = StubStore {
            role: Some(UserRole::Manager),
            load_calls: load_calls.clone(),
        };
        let cache = StubCache::default();

        let first = resolve_legacy_role_with_cache(&store, &cache, &tenant_id, &user_id)
            .await
            .unwrap();
        let second = resolve_legacy_role_with_cache(&store, &cache, &tenant_id, &user_id)
            .await
            .unwrap();

        assert!(!first.cache_hit);
        assert_eq!(first.role, Some(UserRole::Manager));
        assert!(second.cache_hit);
        assert_eq!(second.role, Some(UserRole::Manager));
        assert_eq!(*load_calls.lock().await, 1);
    }

    #[tokio::test]
    async fn missing_legacy_role_is_not_cached() {
        let tenant_id = uuid::Uuid::new_v4();
        let user_id = uuid::Uuid::new_v4();
        let load_calls = Arc::new(Mutex::new(0));
        let store = StubStore {
            role: None,
            load_calls: load_calls.clone(),
        };
        let cache = StubCache::default();

        let first = resolve_legacy_role_with_cache(&store, &cache, &tenant_id, &user_id)
            .await
            .unwrap();
        let second = resolve_legacy_role_with_cache(&store, &cache, &tenant_id, &user_id)
            .await
            .unwrap();

        assert!(!first.cache_hit);
        assert_eq!(first.role, None);
        assert!(!second.cache_hit);
        assert_eq!(second.role, None);
        assert_eq!(*load_calls.lock().await, 2);
    }

    #[tokio::test]
    async fn invalidate_cached_legacy_role_evicts_entry() {
        let tenant_id = uuid::Uuid::new_v4();
        let user_id = uuid::Uuid::new_v4();
        let cache = StubCache::default();

        cache.insert(&tenant_id, &user_id, UserRole::Admin).await;

        invalidate_cached_legacy_role(&cache, &tenant_id, &user_id).await;

        assert!(cache.get(&tenant_id, &user_id).await.is_none());
    }
}
