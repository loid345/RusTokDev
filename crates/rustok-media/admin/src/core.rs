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

#[cfg(test)]
mod tests {
    use super::*;

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
}
