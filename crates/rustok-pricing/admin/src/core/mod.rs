mod presentation;
mod requests;
mod routing;

pub(crate) use presentation::{
    format_adjustment_preview, format_effective_context, format_effective_price,
    format_price_list_option_label, format_price_scope, format_product_meta,
    format_variant_identity, format_variant_prices, localized_product_status, pricing_health_badge,
    pricing_health_label, pricing_translation_for_locale, status_badge, summarize_pricing,
};
pub(crate) use requests::{
    build_discount_draft, build_price_draft, build_price_list_rule_draft,
    build_price_list_scope_draft, build_product_admin_href, build_resolution_context,
    clear_price_list_rule_draft, empty_price_draft, normalized_currency_code,
    normalized_price_list_id, normalized_quantity, normalized_region_id, price_draft_from_price,
    text_or_none,
};
pub(crate) use routing::{
    apply_selected_channel_option, format_channel_option_label, format_channel_scope_text,
    normalize_channel_value, selected_channel_key, GLOBAL_CHANNEL_KEY, LEGACY_CHANNEL_KEY,
};
