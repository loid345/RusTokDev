use leptos::prelude::*;

use crate::core::{
    build_payment_collection_card_view_model, PaymentCollectionCardData,
    PaymentCollectionCardLabels,
};

#[component]
pub fn PaymentCollectionCard(
    payment_collection: Option<PaymentCollectionCardData>,
    labels: PaymentCollectionCardLabels,
) -> impl IntoView {
    let view_model = build_payment_collection_card_view_model(payment_collection, &labels);

    view! {
        <article class="rounded-2xl border border-dashed border-border p-5">
            <div class="text-xs font-medium uppercase tracking-[0.18em] text-muted-foreground">
                {labels.badge}
            </div>
            <p class="mt-2 text-sm text-muted-foreground">
                {labels.module_ownership}
            </p>
            <div class="mt-4 text-xs font-medium uppercase tracking-[0.18em] text-muted-foreground">
                {format!("{} · {}", view_model.collection_id, view_model.status)}
            </div>
        </article>
    }
}
