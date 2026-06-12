use leptos::prelude::*;
use rustok_api::UiRouteContext;

use crate::i18n::t;
use crate::model::{StorefrontCheckoutCart, StorefrontCheckoutDeliveryGroup};

pub(super) type ShippingSelectCallback = Callback<(
    StorefrontCheckoutCart,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
)>;

#[component]
pub(super) fn DeliveryGroupsCard(
    cart: StorefrontCheckoutCart,
    groups: Vec<StorefrontCheckoutDeliveryGroup>,
    busy: ReadSignal<bool>,
    pending_label: String,
    on_select_shipping_option: ShippingSelectCallback,
) -> impl IntoView {
    let locale = use_context::<UiRouteContext>().unwrap_or_default().locale;
    let empty_value = t(locale.as_deref(), "commerce.context.empty", "not resolved");

    view! {
        <article class="rounded-2xl border border-border bg-card p-5">
            <div class="space-y-2">
                <div class="text-xs font-medium uppercase tracking-[0.18em] text-muted-foreground">
                    {t(locale.as_deref(), "commerce.delivery.badge", "delivery orchestration")}
                </div>
                <h4 class="text-base font-semibold text-card-foreground">
                    {t(locale.as_deref(), "commerce.delivery.title", "Delivery groups and shipping selections")}
                </h4>
                <p class="text-sm leading-7 text-muted-foreground">
                    {t(locale.as_deref(), "commerce.delivery.subtitle", "Shipping option selection remains in `commerce` because it coordinates cart grouping, shipping availability, and checkout readiness across split module boundaries.")}
                </p>
            </div>
            <div class="mt-4 space-y-3">
                {groups
                    .into_iter()
                    .map(|group| {
                        let current_cart = cart.clone();
                        let selected_shipping_option_id = group.selected_shipping_option_id.clone();
                        let available_options = group.available_shipping_options.clone();
                        let shipping_profile_slug = group.shipping_profile_slug.clone();
                        let seller_id = group.seller_id.clone();
                        let seller_scope = group.seller_scope.clone();
                        let shipping_profile_slug_for_clear = shipping_profile_slug.clone();
                        let seller_id_for_clear = seller_id.clone();
                        let seller_scope_for_clear = seller_scope.clone();
                        let seller_identity = seller_id.clone().or_else(|| seller_scope.clone());
                        view! {
                            <div class="rounded-2xl border border-border bg-background p-4">
                                <div class="flex flex-wrap items-start justify-between gap-3">
                                    <div class="space-y-1">
                                        <div class="text-xs font-medium uppercase tracking-[0.18em] text-muted-foreground">{shipping_profile_slug.clone()}</div>
                                        {seller_identity.as_ref().map(|seller_identity| view! {
                                            <div class="text-sm text-muted-foreground break-all">{seller_identity.clone()}</div>
                                        })}
                                        <div class="text-sm text-muted-foreground">
                                            {format!(
                                                "{}: {}",
                                                t(locale.as_deref(), "commerce.delivery.lineItems", "Line items"),
                                                group.line_item_count
                                            )}
                                        </div>
                                        <div class="text-sm text-muted-foreground break-all">
                                            {format!(
                                                "{}: {}",
                                                t(locale.as_deref(), "commerce.delivery.selected", "Selected option"),
                                                selected_shipping_option_id.clone().unwrap_or_else(|| empty_value.clone())
                                            )}
                                        </div>
                                    </div>
                                    {if selected_shipping_option_id.is_some() {
                                        let clear_pending_label = pending_label.clone();
                                        let clear_locale = locale.clone();
                                        view! {
                                            <button
                                                type="button"
                                                class="inline-flex items-center justify-center rounded-full border border-border px-3 py-1.5 text-xs font-medium text-card-foreground transition hover:bg-muted disabled:cursor-not-allowed disabled:opacity-60"
                                                disabled=move || busy.get()
                                                on:click={
                                                    let current_cart = current_cart.clone();
                                                    let shipping_profile_slug = shipping_profile_slug_for_clear.clone();
                                                    move |_| on_select_shipping_option.run((
                                                        current_cart.clone(),
                                                        shipping_profile_slug.clone(),
                                                        seller_id_for_clear.clone(),
                                                        seller_scope_for_clear.clone(),
                                                        None,
                                                    ))
                                                }
                                            >
                                                {move || {
                                                    if busy.get() {
                                                        clear_pending_label.clone()
                                                    } else {
                                                        t(clear_locale.as_deref(), "commerce.delivery.clear", "Clear selection")
                                                    }
                                                }}
                                            </button>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="text-xs font-medium uppercase tracking-[0.18em] text-muted-foreground">
                                                {t(locale.as_deref(), "commerce.delivery.required", "selection required")}
                                            </div>
                                        }.into_any()
                                    }}
                                </div>
                                <div class="mt-4 grid gap-3">
                                    {if available_options.is_empty() {
                                        view! {
                                            <div class="rounded-2xl border border-dashed border-border px-4 py-3 text-sm text-muted-foreground">
                                                {t(locale.as_deref(), "commerce.delivery.empty", "No shipping options are currently available for this delivery group in the active storefront context.")}
                                            </div>
                                        }.into_any()
                                    } else {
                                        {available_options
                                                .into_iter()
                                                .map(|option| {
                                                    let option_locale = locale.clone();
                                                    let option_pending_label = pending_label.clone();
                                                    let current_cart = cart.clone();
                                                    let shipping_profile_slug = group.shipping_profile_slug.clone();
                                                    let seller_id = group.seller_id.clone();
                                                    let seller_scope = group.seller_scope.clone();
                                                    let option_id = option.id.clone();
                                                    let option_name = option.name.clone();
                                                    let option_label = format!(
                                                        "{} {} · {}",
                                                        option.currency_code,
                                                        option.amount,
                                                        option.provider_id
                                                    );
                                                    let is_selected = group
                                                        .selected_shipping_option_id
                                                        .as_ref()
                                                        .map(|value| value == &option_id)
                                                        .unwrap_or(false);
                                                    view! {
                                                        <div class="rounded-2xl border border-border px-4 py-3">
                                                            <div class="flex flex-wrap items-center justify-between gap-3">
                                                                <div class="space-y-1">
                                                                    <div class="text-sm font-medium text-card-foreground">{option_name}</div>
                                                                    <div class="text-xs text-muted-foreground">{option_label}</div>
                                                                </div>
                                                                <div class="flex items-center gap-2">
                                                                    <div class="text-xs font-medium uppercase tracking-[0.18em] text-muted-foreground">
                                                                        {if is_selected {
                                                                            t(option_locale.as_deref(), "commerce.delivery.current", "selected")
                                                                        } else if option.active {
                                                                            t(option_locale.as_deref(), "commerce.delivery.available", "available")
                                                                        } else {
                                                                            t(option_locale.as_deref(), "commerce.delivery.inactive", "inactive")
                                                                        }}
                                                                    </div>
                                                                    <button
                                                                        type="button"
                                                                        class="inline-flex items-center justify-center rounded-full border border-border px-3 py-1.5 text-xs font-medium text-card-foreground transition hover:bg-muted disabled:cursor-not-allowed disabled:opacity-60"
                                                                        disabled=move || busy.get() || is_selected || !option.active
                                                                        on:click={
                                                                            let current_cart = current_cart.clone();
                                                                            let shipping_profile_slug = shipping_profile_slug.clone();
                                                                            let option_id = option_id.clone();
                                                                            move |_| on_select_shipping_option.run((
                                                                                current_cart.clone(),
                                                                                shipping_profile_slug.clone(),
                                                                                seller_id.clone(),
                                                                                seller_scope.clone(),
                                                                                Some(option_id.clone()),
                                                                            ))
                                                                        }
                                                                    >
                                                                        {move || {
                                                                            if busy.get() {
                                                                                option_pending_label.clone()
                                                                            } else if is_selected {
                                                                                t(option_locale.as_deref(), "commerce.delivery.current", "selected")
                                                                            } else {
                                                                                t(option_locale.as_deref(), "commerce.delivery.choose", "Choose")
                                                                            }
                                                                        }}
                                                                    </button>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    }
                                                })
                                                .collect_view().into_any()}
                                    }}
                                </div>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </article>
    }
}
