use crate::i18n::t;
use crate::model::{StorefrontCheckoutCart, StorefrontCommerceData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommerceStorefrontShellViewModel {
    pub badge: String,
    pub title: String,
    pub subtitle: String,
    pub load_error: String,
    pub action_error: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommerceStorefrontContextViewModel {
    pub effective_locale: String,
    pub tenant: String,
    pub tenant_default_locale: String,
    pub channel: String,
    pub channel_resolution_source: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommerceCheckoutCartSummaryViewModel {
    pub id: String,
    pub status: String,
    pub subtotal: String,
    pub adjustment_total: String,
    pub shipping_total: String,
    pub total: String,
    pub line_item_count: String,
    pub adjustment_count: String,
    pub delivery_group_count: String,
    pub selected_shipping_option: String,
}

pub fn build_storefront_shell_view_model(locale: Option<&str>) -> CommerceStorefrontShellViewModel {
    CommerceStorefrontShellViewModel {
        badge: t(locale, "commerce.badge", "commerce"),
        title: t(locale, "commerce.title", "Commerce orchestration hub"),
        subtitle: t(
            locale,
            "commerce.subtitle",
            "Catalog, pricing, regions, and cart line-item handling now live in module-owned storefront packages. Commerce remains the aggregate storefront handoff for checkout context and cross-domain flow.",
        ),
        load_error: t(
            locale,
            "commerce.error.load",
            "Failed to load commerce storefront data",
        ),
        action_error: t(
            locale,
            "commerce.error.action",
            "Failed to update aggregate checkout state",
        ),
    }
}

pub fn build_storefront_context_view_model(
    data: StorefrontCommerceData,
    locale: Option<&str>,
) -> CommerceStorefrontContextViewModel {
    let empty_value = t(locale, "commerce.context.empty", "not resolved");

    CommerceStorefrontContextViewModel {
        effective_locale: data.effective_locale,
        tenant: data
            .tenant_slug
            .unwrap_or_else(|| t(locale, "commerce.context.tenantMissing", "host tenant")),
        tenant_default_locale: data.tenant_default_locale,
        channel: data.channel_slug.unwrap_or_else(|| empty_value.clone()),
        channel_resolution_source: data
            .channel_resolution_source
            .unwrap_or_else(|| empty_value.clone()),
    }
}

pub fn build_checkout_cart_summary_view_model(
    cart: &StorefrontCheckoutCart,
    locale: Option<&str>,
) -> CommerceCheckoutCartSummaryViewModel {
    CommerceCheckoutCartSummaryViewModel {
        id: cart.id.clone(),
        status: cart.status.clone(),
        subtotal: format_money(cart.currency_code.as_str(), cart.subtotal_amount.as_str()),
        adjustment_total: format_money(cart.currency_code.as_str(), cart.adjustment_total.as_str()),
        shipping_total: format_money(cart.currency_code.as_str(), cart.shipping_total.as_str()),
        total: format_money(cart.currency_code.as_str(), cart.total_amount.as_str()),
        line_item_count: cart.line_item_count.to_string(),
        adjustment_count: cart.adjustment_count.to_string(),
        delivery_group_count: cart.delivery_group_count.to_string(),
        selected_shipping_option: cart
            .selected_shipping_option_id
            .clone()
            .unwrap_or_else(|| t(locale, "commerce.context.empty", "not resolved")),
    }
}

pub fn error_with_context(context: &str, error: &str) -> String {
    format!("{context}: {error}")
}

fn format_money(currency_code: &str, amount: &str) -> String {
    format!("{currency_code} {amount}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn storefront_data(
        tenant_slug: Option<&str>,
        channel_slug: Option<&str>,
        channel_resolution_source: Option<&str>,
    ) -> StorefrontCommerceData {
        StorefrontCommerceData {
            effective_locale: "ru".to_string(),
            tenant_slug: tenant_slug.map(str::to_string),
            tenant_default_locale: "en".to_string(),
            channel_slug: channel_slug.map(str::to_string),
            channel_resolution_source: channel_resolution_source.map(str::to_string),
            selected_cart_id: None,
            checkout: None,
        }
    }

    #[test]
    fn context_view_model_preserves_resolved_context() {
        let view_model = build_storefront_context_view_model(
            storefront_data(Some("main"), Some("web"), Some("domain")),
            Some("en"),
        );

        assert_eq!(view_model.effective_locale, "ru");
        assert_eq!(view_model.tenant, "main");
        assert_eq!(view_model.tenant_default_locale, "en");
        assert_eq!(view_model.channel, "web");
        assert_eq!(view_model.channel_resolution_source, "domain");
    }

    #[test]
    fn context_view_model_applies_missing_context_fallbacks() {
        let view_model =
            build_storefront_context_view_model(storefront_data(None, None, None), Some("en"));

        assert_eq!(view_model.tenant, "host tenant");
        assert_eq!(view_model.channel, "not resolved");
        assert_eq!(view_model.channel_resolution_source, "not resolved");
    }

    fn checkout_cart(selected_shipping_option_id: Option<&str>) -> StorefrontCheckoutCart {
        StorefrontCheckoutCart {
            id: "cart-1".to_string(),
            status: "open".to_string(),
            currency_code: "USD".to_string(),
            subtotal_amount: "10.00".to_string(),
            adjustment_total: "1.00".to_string(),
            shipping_total: "2.00".to_string(),
            total_amount: "13.00".to_string(),
            channel_slug: None,
            email: None,
            customer_id: None,
            region_id: None,
            country_code: None,
            locale_code: None,
            selected_shipping_option_id: selected_shipping_option_id.map(str::to_string),
            line_item_count: 2,
            adjustment_count: 1,
            delivery_group_count: 3,
            adjustments: Vec::new(),
            delivery_groups: Vec::new(),
        }
    }

    #[test]
    fn checkout_cart_summary_formats_counts_money_and_selected_shipping() {
        let view_model =
            build_checkout_cart_summary_view_model(&checkout_cart(Some("ship-1")), Some("en"));

        assert_eq!(view_model.id, "cart-1");
        assert_eq!(view_model.status, "open");
        assert_eq!(view_model.subtotal, "USD 10.00");
        assert_eq!(view_model.adjustment_total, "USD 1.00");
        assert_eq!(view_model.shipping_total, "USD 2.00");
        assert_eq!(view_model.total, "USD 13.00");
        assert_eq!(view_model.line_item_count, "2");
        assert_eq!(view_model.adjustment_count, "1");
        assert_eq!(view_model.delivery_group_count, "3");
        assert_eq!(view_model.selected_shipping_option, "ship-1");
    }

    #[test]
    fn checkout_cart_summary_applies_missing_shipping_fallback() {
        let view_model = build_checkout_cart_summary_view_model(&checkout_cart(None), Some("en"));

        assert_eq!(view_model.selected_shipping_option, "not resolved");
    }
}
