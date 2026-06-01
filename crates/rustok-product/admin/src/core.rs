use crate::i18n::t;
use crate::model::{ProductDetail, ProductPricingDetail, ProductTranslation, ShippingProfile};

fn locale_tags_match(left: &str, right: &str) -> bool {
    left.trim()
        .replace('_', "-")
        .eq_ignore_ascii_case(&right.trim().replace('_', "-"))
}

pub(crate) fn translation_for_locale(
    translations: &[ProductTranslation],
    requested_locale: Option<&str>,
) -> Option<ProductTranslation> {
    requested_locale.and_then(|requested_locale| {
        translations
            .iter()
            .find(|translation| locale_tags_match(&translation.locale, requested_locale))
            .cloned()
    })
}

pub(crate) fn primary_catalog_currency(product: Option<&ProductDetail>) -> Option<String> {
    product.and_then(|item| {
        item.variants
            .first()
            .and_then(|variant| variant.prices.first())
            .map(|price| price.currency_code.clone())
    })
}

pub(crate) fn format_catalog_snapshot_price(
    locale: Option<&str>,
    product: Option<&ProductDetail>,
) -> String {
    product
        .and_then(|item| item.variants.first())
        .and_then(|variant| variant.prices.first())
        .map(|price| {
            format_scoped_price(
                locale,
                &price.currency_code,
                &price.amount,
                price.compare_at_amount.as_deref(),
                None,
            )
        })
        .unwrap_or_else(|| t(locale, "product.summary.noPricing", "no pricing"))
}

pub(crate) fn format_pricing_preview(
    locale: Option<&str>,
    pricing: Option<&ProductPricingDetail>,
) -> String {
    let Some(pricing) = pricing else {
        return t(
            locale,
            "product.summary.pricingUnavailable",
            "Pricing module preview is unavailable.",
        );
    };

    let Some(variant) = pricing.variants.first() else {
        return t(locale, "product.summary.noPricing", "no pricing");
    };

    if let Some(price) = variant.effective_price.as_ref() {
        let mut label = format_scoped_price(
            locale,
            &price.currency_code,
            &price.amount,
            price.compare_at_amount.as_deref(),
            price.discount_percent.as_deref(),
        );
        if let Some(scope) = format_pricing_scope(
            locale,
            price.price_list_id.as_deref(),
            price.channel_slug.as_deref(),
            price.channel_id.as_deref(),
        ) {
            label.push_str(format!(" | {scope}").as_str());
        }
        return label;
    }

    variant
        .prices
        .first()
        .map(|price| {
            format_scoped_price(
                locale,
                &price.currency_code,
                &price.amount,
                price.compare_at_amount.as_deref(),
                price.discount_percent.as_deref(),
            )
        })
        .unwrap_or_else(|| t(locale, "product.summary.noPricing", "no pricing"))
}

fn format_scoped_price(
    locale: Option<&str>,
    currency_code: &str,
    amount: &str,
    compare_at_amount: Option<&str>,
    discount_percent: Option<&str>,
) -> String {
    let mut label = if let Some(compare_at_amount) = compare_at_amount {
        format!(
            "{} {} ({})",
            currency_code,
            amount,
            t(locale, "product.summary.compareAt", "compare-at {value}")
                .replace("{value}", compare_at_amount),
        )
    } else {
        format!("{currency_code} {amount}")
    };

    if let Some(discount_percent) = discount_percent.filter(|value| !value.trim().is_empty()) {
        label.push_str(format!(" (-{discount_percent}%)").as_str());
    }

    label
}

fn format_pricing_scope(
    locale: Option<&str>,
    price_list_id: Option<&str>,
    channel_slug: Option<&str>,
    channel_id: Option<&str>,
) -> Option<String> {
    let price_list_id = price_list_id.filter(|value| !value.trim().is_empty());
    let channel_slug = channel_slug.filter(|value| !value.trim().is_empty());
    let channel_id = channel_id.filter(|value| !value.trim().is_empty());

    if price_list_id.is_none() && channel_slug.is_none() && channel_id.is_none() {
        return None;
    }

    let mut parts = Vec::new();
    if let Some(price_list_id) = price_list_id {
        parts.push(t(locale, "product.summary.priceList", "price list") + " " + price_list_id);
    }
    match (channel_slug, channel_id) {
        (Some(channel_slug), Some(channel_id)) => parts.push(
            t(locale, "product.summary.channel", "channel")
                + " "
                + channel_slug
                + " ("
                + channel_id
                + ")",
        ),
        (Some(channel_slug), None) => {
            parts.push(t(locale, "product.summary.channel", "channel") + " " + channel_slug)
        }
        (None, Some(channel_id)) => {
            parts.push(t(locale, "product.summary.channel", "channel") + " " + channel_id)
        }
        (None, None) => {}
    }

    Some(parts.join(" | "))
}

pub(crate) fn build_admin_pricing_href(module_route_base: &str, product: &ProductDetail) -> String {
    let mut params = vec![format!("id={}", product.id)];
    if let Some(currency_code) =
        primary_catalog_currency(Some(product)).filter(|value| !value.trim().is_empty())
    {
        params.push(format!("currency={currency_code}"));
    }
    params.push("quantity=1".to_string());
    format!("{module_route_base}?{}", params.join("&"))
}

pub(crate) fn format_known_shipping_profiles(
    locale: Option<&str>,
    profiles: &[ShippingProfile],
) -> String {
    let slugs = profiles
        .iter()
        .filter(|profile| profile.active)
        .map(|profile| profile.slug.as_str())
        .collect::<Vec<_>>();
    if slugs.is_empty() {
        t(locale, "product.common.noneYet", "none yet")
    } else {
        slugs.join(", ")
    }
}

pub(crate) fn shipping_profile_choice_label(
    locale: Option<&str>,
    profile: &ShippingProfile,
) -> String {
    if profile.active {
        format!("{} ({})", profile.name, profile.slug)
    } else {
        format!(
            "{} ({}, {})",
            profile.name,
            profile.slug,
            t(locale, "product.common.inactive", "inactive")
        )
    }
}

pub(crate) fn localized_product_status(locale: Option<&str>, status: &str) -> String {
    match status {
        "ACTIVE" => t(locale, "product.status.active", "Active"),
        "ARCHIVED" => t(locale, "product.status.archived", "Archived"),
        _ => t(locale, "product.status.draft", "Draft"),
    }
}

pub(crate) fn format_product_meta(
    locale: Option<&str>,
    handle: &str,
    vendor: Option<&str>,
) -> String {
    let handle_label = t(locale, "product.summary.handle", "handle");
    let vendor_label = t(locale, "product.summary.vendor", "vendor");
    match vendor.filter(|value| !value.is_empty()) {
        Some(vendor) => format!("{handle_label}: {handle} | {vendor_label}: {vendor}"),
        None => format!("{handle_label}: {handle}"),
    }
}

pub(crate) fn format_product_shipping_profile(locale: Option<&str>, slug: &str) -> String {
    t(locale, "product.summary.profileChip", "profile {slug}").replace("{slug}", slug)
}

pub(crate) fn text_or_none(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

pub(crate) fn status_badge(status: &str) -> &'static str {
    match status {
        "ACTIVE" => "border-emerald-200 bg-emerald-50 text-emerald-700",
        "ARCHIVED" => "border-slate-200 bg-slate-100 text-slate-700",
        _ => "border-amber-200 bg-amber-50 text-amber-700",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_or_none_trims_empty_admin_filters() {
        assert_eq!(text_or_none("  ".to_string()), None);
        assert_eq!(
            text_or_none(" active ".to_string()),
            Some("active".to_string())
        );
    }

    #[test]
    fn admin_status_labels_and_badges_are_framework_agnostic() {
        assert_eq!(localized_product_status(Some("en"), "ACTIVE"), "Active");
        assert!(status_badge("ARCHIVED").contains("slate"));
        assert!(status_badge("DRAFT").contains("amber"));
    }

    #[test]
    fn product_meta_and_profile_chip_are_stable() {
        assert_eq!(
            format_product_meta(Some("en"), "winter-coat", Some("Acme")),
            "handle: winter-coat | vendor: Acme",
        );
        assert_eq!(
            format_product_shipping_profile(Some("en"), "standard"),
            "profile standard",
        );
    }
}
