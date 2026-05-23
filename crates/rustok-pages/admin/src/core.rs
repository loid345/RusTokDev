pub fn slugify(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub fn parse_channel_slugs(value: &str) -> Vec<String> {
    let mut items = value
        .split(',')
        .map(|item| item.trim().to_ascii_lowercase())
        .filter(|item| !item.is_empty())
        .collect::<Vec<_>>();
    items.sort();
    items.dedup();
    items
}

pub fn error_with_context(context: &str, error: &str) -> String {
    format!("{}: {}", context, error)
}

pub fn status_badge_class(status: &str) -> &'static str {
    match status.to_ascii_lowercase().as_str() {
        "published" => {
            "bg-emerald-50 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-400"
        }
        "archived" => "bg-muted text-muted-foreground",
        _ => "bg-primary/10 text-primary",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_normalizes_ascii_words() {
        assert_eq!(slugify("Hello, Rustok Pages!"), "hello-rustok-pages");
    }

    #[test]
    fn parse_channel_slugs_trims_sorts_and_deduplicates() {
        assert_eq!(
            parse_channel_slugs(" web, mobile-app,WEB, , mobile-app "),
            vec!["mobile-app".to_string(), "web".to_string()]
        );
    }

    #[test]
    fn error_with_context_formats_consistently() {
        assert_eq!(
            error_with_context("Failed to save page", "timeout"),
            "Failed to save page: timeout"
        );
        assert_eq!(
            status_badge_class("published"),
            "bg-emerald-50 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-400"
        );
        assert_eq!(status_badge_class("draft"), "bg-primary/10 text-primary");
        assert_eq!(busy_key_with_id("edit", "p_1"), "edit:p_1");
        assert_eq!(busy_key_for_save(Some("p_2")), "save:p_2");
        assert_eq!(busy_key_for_save(None), "create");
    }
}

pub fn busy_key_with_id(action: &str, page_id: &str) -> String {
    format!("{}:{}", action, page_id)
}

pub fn busy_key_for_save(page_id: Option<&str>) -> String {
    page_id
        .map(|id| busy_key_with_id("save", id))
        .unwrap_or_else(|| "create".to_string())
}
