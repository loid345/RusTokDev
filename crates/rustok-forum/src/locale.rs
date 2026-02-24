use rustok_content::dto::{BodyResponse, NodeTranslationResponse};

pub struct ResolvedTranslation<'a> {
    pub translation: Option<&'a NodeTranslationResponse>,
    pub effective_locale: String,
}

pub struct ResolvedBody<'a> {
    pub body: Option<&'a BodyResponse>,
    pub effective_locale: String,
}

pub fn resolve_translation<'a>(
    translations: &'a [NodeTranslationResponse],
    requested: &str,
) -> ResolvedTranslation<'a> {
    if let Some(t) = translations.iter().find(|t| t.locale == requested) {
        return ResolvedTranslation {
            translation: Some(t),
            effective_locale: requested.to_string(),
        };
    }
    if let Some(t) = translations.iter().find(|t| t.locale == "en") {
        return ResolvedTranslation {
            translation: Some(t),
            effective_locale: "en".to_string(),
        };
    }
    if let Some(t) = translations.first() {
        return ResolvedTranslation {
            translation: Some(t),
            effective_locale: t.locale.clone(),
        };
    }
    ResolvedTranslation {
        translation: None,
        effective_locale: requested.to_string(),
    }
}

pub fn resolve_body<'a>(bodies: &'a [BodyResponse], requested: &str) -> ResolvedBody<'a> {
    if let Some(b) = bodies.iter().find(|b| b.locale == requested) {
        return ResolvedBody {
            body: Some(b),
            effective_locale: requested.to_string(),
        };
    }
    if let Some(b) = bodies.iter().find(|b| b.locale == "en") {
        return ResolvedBody {
            body: Some(b),
            effective_locale: "en".to_string(),
        };
    }
    if let Some(b) = bodies.first() {
        return ResolvedBody {
            body: Some(b),
            effective_locale: b.locale.clone(),
        };
    }
    ResolvedBody {
        body: None,
        effective_locale: requested.to_string(),
    }
}

pub fn available_locales(translations: &[NodeTranslationResponse]) -> Vec<String> {
    translations.iter().map(|t| t.locale.clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustok_content::dto::NodeTranslationResponse;

    fn tr(locale: &str) -> NodeTranslationResponse {
        NodeTranslationResponse {
            locale: locale.to_string(),
            title: Some(format!("Title {locale}")),
            slug: None,
            excerpt: None,
        }
    }

    #[test]
    fn resolve_exact_locale() {
        let translations = vec![tr("en"), tr("ru")];
        let result = resolve_translation(&translations, "ru");
        assert_eq!(result.effective_locale, "ru");
        assert_eq!(result.translation.unwrap().locale, "ru");
    }

    #[test]
    fn resolve_falls_back_to_en() {
        let translations = vec![tr("en"), tr("de")];
        let result = resolve_translation(&translations, "ru");
        assert_eq!(result.effective_locale, "en");
    }

    #[test]
    fn resolve_falls_back_to_first() {
        let translations = vec![tr("de"), tr("fr")];
        let result = resolve_translation(&translations, "ru");
        assert_eq!(result.effective_locale, "de");
    }

    #[test]
    fn resolve_empty_returns_requested() {
        let translations: Vec<NodeTranslationResponse> = vec![];
        let result = resolve_translation(&translations, "en");
        assert!(result.translation.is_none());
        assert_eq!(result.effective_locale, "en");
    }

    #[test]
    fn available_locales_returns_all() {
        let translations = vec![tr("en"), tr("ru"), tr("de")];
        let locales = available_locales(&translations);
        assert_eq!(locales, vec!["en", "ru", "de"]);
    }
}
