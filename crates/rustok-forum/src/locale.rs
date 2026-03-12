use rustok_content::dto::{BodyResponse, NodeTranslationResponse};
use rustok_content::{available_locales_from, resolve_by_locale_with_fallback};

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
    resolve_translation_with_fallback(translations, requested, None)
}

pub fn resolve_translation_with_fallback<'a>(
    translations: &'a [NodeTranslationResponse],
    requested: &str,
    fallback_locale: Option<&str>,
) -> ResolvedTranslation<'a> {
    let resolved =
        resolve_by_locale_with_fallback(translations, requested, fallback_locale, |translation| {
            &translation.locale
        });
    ResolvedTranslation {
        translation: resolved.item,
        effective_locale: resolved.effective_locale,
    }
}

pub fn resolve_body<'a>(bodies: &'a [BodyResponse], requested: &str) -> ResolvedBody<'a> {
    resolve_body_with_fallback(bodies, requested, None)
}

pub fn resolve_body_with_fallback<'a>(
    bodies: &'a [BodyResponse],
    requested: &str,
    fallback_locale: Option<&str>,
) -> ResolvedBody<'a> {
    let resolved =
        resolve_by_locale_with_fallback(bodies, requested, fallback_locale, |body| &body.locale);
    ResolvedBody {
        body: resolved.item,
        effective_locale: resolved.effective_locale,
    }
}

pub fn available_locales(translations: &[NodeTranslationResponse]) -> Vec<String> {
    available_locales_from(translations, |translation| &translation.locale)
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
