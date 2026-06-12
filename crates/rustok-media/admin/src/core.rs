use crate::model::{MediaTranslationPayload, MediaUsageSnapshot, UpsertTranslationPayload};

/// Trims user-entered optional metadata and keeps the transport payload free of
/// empty strings. This helper is framework-agnostic so future FFA adapters can
/// reuse the same form-to-command policy without depending on framework-specific signals.
pub fn non_empty_option(value: &str) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

/// Builds the asset dimensions label used by UI adapters. Missing partial
/// dimensions intentionally fall back to the host-localized `not_available`
/// label instead of exposing inconsistent `width × ?` strings.
pub fn media_dimensions_label(
    width: Option<i32>,
    height: Option<i32>,
    not_available: &str,
) -> String {
    width
        .zip(height)
        .map(|(width, height)| format!("{width}×{height}"))
        .unwrap_or_else(|| not_available.to_string())
}

/// Applies the admin pagination label template to a concrete page number.
pub fn page_count_label(template: &str, page: i32) -> String {
    template.replace("{count}", &page.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaTranslationFormState {
    pub title: String,
    pub alt_text: String,
    pub caption: String,
}

impl MediaTranslationFormState {
    pub fn empty() -> Self {
        Self {
            title: String::new(),
            alt_text: String::new(),
            caption: String::new(),
        }
    }

    pub fn from_translation(translation: &MediaTranslationPayload) -> Self {
        Self {
            title: translation.title.clone().unwrap_or_default(),
            alt_text: translation.alt_text.clone().unwrap_or_default(),
            caption: translation.caption.clone().unwrap_or_default(),
        }
    }

    pub fn to_upsert_payload(&self, locale: String) -> UpsertTranslationPayload {
        UpsertTranslationPayload {
            locale,
            title: non_empty_option(&self.title),
            alt_text: non_empty_option(&self.alt_text),
            caption: non_empty_option(&self.caption),
        }
    }
}

pub fn selected_translation_form_state(
    translations: &[MediaTranslationPayload],
    selected_locale: &str,
) -> MediaTranslationFormState {
    translations
        .iter()
        .find(|item| item.locale == selected_locale)
        .map(MediaTranslationFormState::from_translation)
        .unwrap_or_else(MediaTranslationFormState::empty)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaUsageLabels {
    pub files: String,
    pub total_bytes: String,
    pub tenant: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaUsageStatCard {
    pub label: String,
    pub value: String,
}

pub fn media_usage_stat_cards(
    snapshot: MediaUsageSnapshot,
    labels: MediaUsageLabels,
) -> [MediaUsageStatCard; 3] {
    [
        MediaUsageStatCard {
            label: labels.files,
            value: snapshot.file_count.to_string(),
        },
        MediaUsageStatCard {
            label: labels.total_bytes,
            value: snapshot.total_bytes.to_string(),
        },
        MediaUsageStatCard {
            label: labels.tenant,
            value: snapshot.tenant_id,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn translation(locale: &str, title: Option<&str>) -> MediaTranslationPayload {
        MediaTranslationPayload {
            id: format!("translation-{locale}"),
            media_id: "media-1".to_string(),
            locale: locale.to_string(),
            title: title.map(str::to_string),
            alt_text: Some(format!("alt-{locale}")),
            caption: None,
        }
    }

    #[test]
    fn non_empty_option_trims_and_drops_empty_values() {
        assert_eq!(
            non_empty_option("  Alt text  "),
            Some("Alt text".to_string())
        );
        assert_eq!(non_empty_option("   "), None);
    }

    #[test]
    fn media_dimensions_label_requires_both_dimensions() {
        assert_eq!(
            media_dimensions_label(Some(640), Some(480), "n/a"),
            "640×480"
        );
        assert_eq!(media_dimensions_label(Some(640), None, "n/a"), "n/a");
        assert_eq!(media_dimensions_label(None, Some(480), "n/a"), "n/a");
    }

    #[test]
    fn page_count_label_replaces_count_placeholder() {
        assert_eq!(page_count_label("Page {count}", 3), "Page 3");
    }

    #[test]
    fn selected_translation_form_state_uses_matching_locale_or_empty_state() {
        let translations = vec![translation("en", Some("English")), translation("ru", None)];

        assert_eq!(
            selected_translation_form_state(&translations, "en"),
            MediaTranslationFormState {
                title: "English".to_string(),
                alt_text: "alt-en".to_string(),
                caption: String::new(),
            }
        );
        assert_eq!(
            selected_translation_form_state(&translations, "de"),
            MediaTranslationFormState::empty()
        );
    }

    #[test]
    fn translation_form_state_builds_trimmed_upsert_payload() {
        let state = MediaTranslationFormState {
            title: "  Title  ".to_string(),
            alt_text: " ".to_string(),
            caption: "Caption".to_string(),
        };

        assert_eq!(
            state.to_upsert_payload("en".to_string()),
            UpsertTranslationPayload {
                locale: "en".to_string(),
                title: Some("Title".to_string()),
                alt_text: None,
                caption: Some("Caption".to_string()),
            }
        );
    }

    #[test]
    fn media_usage_stat_cards_preserve_label_order() {
        let cards = media_usage_stat_cards(
            MediaUsageSnapshot {
                tenant_id: "tenant-a".to_string(),
                file_count: 2,
                total_bytes: 2048,
            },
            MediaUsageLabels {
                files: "Files".to_string(),
                total_bytes: "Total".to_string(),
                tenant: "Tenant".to_string(),
            },
        );

        assert_eq!(cards[0].value, "2");
        assert_eq!(cards[1].value, "2048");
        assert_eq!(cards[2].value, "tenant-a");
    }
}
