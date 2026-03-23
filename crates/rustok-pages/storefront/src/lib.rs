mod api;
mod model;

use leptos::prelude::*;

use crate::model::{PageDetail, PageListItem, StorefrontPagesData};

#[component]
pub fn PagesView() -> impl IntoView {
    let pages_resource = Resource::new_blocking(
        || (),
        |_| async move { api::fetch_storefront_pages().await },
    );

    view! {
        <section class="rounded-3xl border border-border bg-card p-8 shadow-sm">
            <div class="max-w-3xl space-y-3">
                <span class="inline-flex items-center rounded-full border border-border px-3 py-1 text-xs font-medium text-muted-foreground">
                    "pages"
                </span>
                <h2 class="text-3xl font-semibold text-card-foreground">
                    "Page-driven storefront experiences"
                </h2>
                <p class="text-sm text-muted-foreground">
                    "This module package now renders real page data through the pages module contract instead of a static placeholder."
                </p>
            </div>

            <div class="mt-8">
                <Suspense fallback=|| view! {
                    <div class="space-y-4">
                        <div class="h-32 animate-pulse rounded-2xl bg-muted"></div>
                        <div class="grid gap-3 md:grid-cols-2">
                            <div class="h-24 animate-pulse rounded-2xl bg-muted"></div>
                            <div class="h-24 animate-pulse rounded-2xl bg-muted"></div>
                        </div>
                    </div>
                }>
                    {move || {
                        let pages_resource = pages_resource.clone();
                        Suspend::new(async move {
                            match pages_resource.await {
                                Ok(data) => view! { <PagesShowcase data /> }.into_any(),
                                Err(err) => view! {
                                    <div class="rounded-2xl border border-destructive/30 bg-destructive/10 px-4 py-3 text-sm text-destructive">
                                        {format!("Failed to load pages storefront data: {err}")}
                                    </div>
                                }.into_any(),
                            }
                        })
                    }}
                </Suspense>
            </div>
        </section>
    }
}

#[component]
fn PagesShowcase(data: StorefrontPagesData) -> impl IntoView {
    view! {
        <div class="space-y-6">
            <HomePageCard home=data.home />
            <PublishedPagesList items=data.pages.items total=data.pages.total />
        </div>
    }
}

#[component]
fn HomePageCard(home: Option<PageDetail>) -> impl IntoView {
    let Some(home) = home else {
        return view! {
            <article class="rounded-2xl border border-dashed border-border p-6">
                <h3 class="text-lg font-semibold text-card-foreground">"Home page is not published yet"</h3>
                <p class="mt-2 text-sm text-muted-foreground">
                    "Create and publish a page with slug `home` in the pages admin package to populate this storefront surface."
                </p>
            </article>
        }
        .into_any();
    };

    let title = home
        .translation
        .as_ref()
        .and_then(|translation| translation.title.clone())
        .unwrap_or_else(|| "Home".to_string());
    let slug = home
        .translation
        .as_ref()
        .and_then(|translation| translation.slug.clone())
        .unwrap_or_else(|| "home".to_string());
    let effective_locale = home
        .effective_locale
        .clone()
        .unwrap_or_else(|| "default".to_string());
    let body = home
        .body
        .as_ref()
        .map(|body| summarize_content(body.content.as_str(), body.format.as_str()))
        .unwrap_or_else(|| "No body content yet.".to_string());

    view! {
        <article class="rounded-2xl border border-border bg-background p-6">
            <div class="flex flex-wrap items-center gap-2 text-xs font-medium uppercase tracking-[0.22em] text-muted-foreground">
                <span>{format!("slug: {slug}")}</span>
                <span>"·"</span>
                <span>{format!("locale: {effective_locale}")}</span>
            </div>
            <h3 class="mt-3 text-2xl font-semibold text-foreground">{title}</h3>
            <p class="mt-3 whitespace-pre-line text-sm leading-7 text-muted-foreground">{body}</p>
        </article>
    }
    .into_any()
}

#[component]
fn PublishedPagesList(items: Vec<PageListItem>, total: u64) -> impl IntoView {
    if items.is_empty() {
        return view! {
            <article class="rounded-2xl border border-dashed border-border p-6">
                <p class="text-sm text-muted-foreground">
                    "No published pages are available for storefront rendering yet."
                </p>
            </article>
        }
        .into_any();
    }

    view! {
        <div class="space-y-3">
            <div class="flex items-center justify-between gap-3">
                <h3 class="text-lg font-semibold text-card-foreground">"Published pages"</h3>
                <span class="text-sm text-muted-foreground">{format!("{total} total")}</span>
            </div>
            <div class="grid gap-3 md:grid-cols-2">
                {items
                    .into_iter()
                    .map(|page| {
                        view! {
                            <article class="rounded-2xl border border-border bg-background p-5">
                                <div class="text-xs font-medium uppercase tracking-[0.2em] text-muted-foreground">
                                    {page.status}
                                </div>
                                <h4 class="mt-2 text-base font-semibold text-foreground">
                                    {page.title.unwrap_or_else(|| "Untitled page".to_string())}
                                </h4>
                                <p class="mt-2 text-sm text-muted-foreground">
                                    {format!(
                                        "/{}",
                                        page.slug.unwrap_or_else(|| "missing-slug".to_string())
                                    )}
                                </p>
                                <p class="mt-3 text-xs text-muted-foreground">
                                    {format!("template: {}", page.template)}
                                </p>
                            </article>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
    .into_any()
}

fn summarize_content(content: &str, format: &str) -> String {
    if format.eq_ignore_ascii_case("markdown") {
        return content.trim().to_string();
    }

    format!(
        "Stored in `{format}` format. Raw body length: {} characters.",
        content.chars().count()
    )
}
