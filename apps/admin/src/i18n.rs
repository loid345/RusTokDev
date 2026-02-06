use serde_json::{Map, Value};
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Locale {
    En,
    Ru,
}

static EN_MESSAGES: OnceLock<Map<String, Value>> = OnceLock::new();
static RU_MESSAGES: OnceLock<Map<String, Value>> = OnceLock::new();

pub fn translate(locale: Locale, key: &str) -> String {
    let messages = match locale {
        Locale::En => EN_MESSAGES.get_or_init(|| load_messages(include_str!("../locales/en.json"))),
        Locale::Ru => RU_MESSAGES.get_or_init(|| load_messages(include_str!("../locales/ru.json"))),
    };

    messages
        .get(key)
        .and_then(|value| value.as_str())
        .unwrap_or(key)
        .to_string()
}

fn load_messages(source: &str) -> Map<String, Value> {
    serde_json::from_str::<Map<String, Value>>(source).unwrap_or_default()
}
