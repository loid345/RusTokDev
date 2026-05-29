use rustok_api::TenantContext;
use rustok_content::resolve_by_locale_with_fallback;
use rustok_core::normalize_locale_tag;
use rustok_media::entities::{media, media_translation};
use rustok_tenant::entities::tenant_module;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use url::Url;
use uuid::Uuid;

use crate::SeoResult;

use super::SeoService;
use super::TargetState;

impl SeoService {
    pub(super) async fn enrich_target_state_with_media_hooks(
        &self,
        tenant: &TenantContext,
        state: &mut TargetState,
    ) -> SeoResult<()> {
        if !self.media_module_enabled(tenant.id).await? {
            return Ok(());
        }

        for image in &mut state.open_graph.images {
            let media_id = image
                .media_id
                .or_else(|| parse_media_id_from_url(image.url.as_str()));
            let Some(media_id) = media_id else {
                continue;
            };

            image.media_id = Some(media_id);

            if let Err(error) = self
                .enrich_image_from_media(
                    tenant.id,
                    tenant.default_locale.as_str(),
                    state.effective_locale.as_str(),
                    image,
                    media_id,
                )
                .await
            {
                tracing::warn!(
                    tenant_id = %tenant.id,
                    media_id = %media_id,
                    error = %error,
                    "SEO media image enrichment failed"
                );
            }
        }

        if let Some(first) = state.open_graph.images.first() {
            state
                .template_fields
                .insert("image_url".to_string(), first.url.clone());
            if let Some(alt) = first.alt.as_deref().map(str::trim).filter(|value| !value.is_empty())
            {
                state
                    .template_fields
                    .insert("image_alt".to_string(), alt.to_string());
            }
            if let Some(width) = first.width {
                state
                    .template_fields
                    .insert("image_width".to_string(), width.to_string());
            }
            if let Some(height) = first.height {
                state
                    .template_fields
                    .insert("image_height".to_string(), height.to_string());
            }
            if let Some(mime_type) = first
                .mime_type
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
            {
                state
                    .template_fields
                    .insert("image_mime_type".to_string(), mime_type.to_string());
            }
        }

        Ok(())
    }

    async fn media_module_enabled(&self, tenant_id: Uuid) -> SeoResult<bool> {
        tenant_module::Entity::is_enabled(&self.db, tenant_id, "media")
            .await
            .map_err(Into::into)
    }

    async fn enrich_image_from_media(
        &self,
        tenant_id: Uuid,
        default_locale: &str,
        requested_locale: &str,
        image: &mut crate::dto::SeoImageAsset,
        media_id: Uuid,
    ) -> Result<(), sea_orm::DbErr> {
        let Some(model) = media::Entity::find()
            .filter(media::Column::TenantId.eq(tenant_id))
            .filter(media::Column::Id.eq(media_id))
            .one(&self.db)
            .await?
        else {
            return Ok(());
        };

        if image.width.is_none() {
            image.width = model.width;
        }
        if image.height.is_none() {
            image.height = model.height;
        }
        if image
            .mime_type
            .as_deref()
            .map(str::trim)
            .is_none_or(|value| value.is_empty())
        {
            image.mime_type = Some(model.mime_type);
        }

        if image.alt.as_deref().map(str::trim).is_some_and(|value| !value.is_empty()) {
            return Ok(());
        }

        let translations = media_translation::Entity::find()
            .filter(media_translation::Column::MediaId.eq(media_id))
            .all(&self.db)
            .await?;

        let normalized_requested = normalize_locale_tag(requested_locale)
            .unwrap_or_else(|| requested_locale.to_string());
        let normalized_default = normalize_locale_tag(default_locale)
            .unwrap_or_else(|| default_locale.to_string());
        let resolved = resolve_by_locale_with_fallback(
            translations.as_slice(),
            normalized_requested.as_str(),
            Some(normalized_default.as_str()),
            |item| item.locale.as_str(),
        );

        let resolved_alt = resolved
            .item
            .and_then(|item| item.alt_text.as_deref())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string)
            .or_else(|| {
                translations
                    .iter()
                    .find_map(|item| item.alt_text.as_deref())
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(str::to_string)
            });

        if let Some(alt) = resolved_alt {
            image.alt = Some(alt);
        }

        Ok(())
    }
}

fn parse_media_id_from_url(value: &str) -> Option<Uuid> {
    let parsed = Url::parse(value)
        .or_else(|_| Url::parse(format!("https://rustok.local{value}").as_str()))
        .ok()?;

    if let Some(query_media_id) = parsed
        .query_pairs()
        .find(|(key, _)| key.eq_ignore_ascii_case("media_id") || key.eq_ignore_ascii_case("id"))
        .and_then(|(_, value)| Uuid::parse_str(value.as_ref()).ok())
    {
        return Some(query_media_id);
    }

    let segments = parsed
        .path_segments()
        .map(|items| items.filter(|segment| !segment.is_empty()).collect::<Vec<_>>())
        .unwrap_or_default();
    for pair in segments.windows(2) {
        if pair[0].eq_ignore_ascii_case("media") {
            if let Ok(id) = Uuid::parse_str(pair[1]) {
                return Some(id);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::parse_media_id_from_url;
    use uuid::Uuid;

    #[test]
    fn parse_media_id_from_url_supports_relative_absolute_and_query() {
        let media_id = Uuid::new_v4();

        assert_eq!(
            parse_media_id_from_url(format!("/api/media/{media_id}").as_str()),
            Some(media_id)
        );
        assert_eq!(
            parse_media_id_from_url(
                format!("https://demo.test/api/media/{media_id}?download=1").as_str()
            ),
            Some(media_id)
        );
        assert_eq!(
            parse_media_id_from_url(
                format!("https://demo.test/assets/image.png?media_id={media_id}").as_str()
            ),
            Some(media_id)
        );
        assert_eq!(
            parse_media_id_from_url("https://demo.test/assets/image.png"),
            None
        );
    }
}
