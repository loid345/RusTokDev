mod api;
mod model;

use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_auth::hooks::{use_tenant, use_token};

use crate::model::{CreatePageDraft, PageListItem};

#[component]
pub fn PagesAdmin() -> impl IntoView {
    let token = use_token();
    let tenant = use_tenant();

    let (refresh_nonce, set_refresh_nonce) = signal(0_u64);
    let (title, set_title) = signal(String::new());
    let (slug, set_slug) = signal(String::new());
    let (body, set_body) = signal(String::new());
    let (locale, set_locale) = signal("en".to_string());
    let (publish_now, set_publish_now) = signal(false);
    let (busy_page_id, set_busy_page_id) = signal(Option::<String>::None);
    let (submit_error, set_submit_error) = signal(Option::<String>::None);

    let pages_resource = Resource::new(
        move || (token.get(), tenant.get(), refresh_nonce.get()),
        move |(token_value, tenant_value, _)| async move {
            api::fetch_pages(token_value, tenant_value).await
        },
    );

    let create_page = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_submit_error.set(None);

        let draft = CreatePageDraft {
            locale: locale.get_untracked(),
            title: title.get_untracked().trim().to_string(),
            slug: slug.get_untracked().trim().to_string(),
            body: body.get_untracked().trim().to_string(),
            template: Some("default".to_string()),
            publish: publish_now.get_untracked(),
        };

        if draft.title.is_empty() || draft.slug.is_empty() || draft.body.is_empty() {
            set_submit_error.set(Some(
                "Title, slug and body are required to create a working page.".to_string(),
            ));
            return;
        }

        let token_value = token.get_untracked();
        let tenant_value = tenant.get_untracked();
        set_busy_page_id.set(Some("create".to_string()));

        spawn_local(async move {
            match api::create_page(token_value, tenant_value, draft).await {
                Ok(_) => {
                    set_title.set(String::new());
                    set_slug.set(String::new());
                    set_body.set(String::new());
                    set_publish_now.set(false);
                    set_refresh_nonce.update(|value| *value += 1);
                }
                Err(err) => {
                    set_submit_error.set(Some(format!("Failed to create page: {err}")));
                }
            }
            set_busy_page_id.set(None);
        });
    };

    let publish_page = Callback::new(move |(page_id, publish): (String, bool)| {
        let token_value = token.get_untracked();
        let tenant_value = tenant.get_untracked();
        set_submit_error.set(None);
        set_busy_page_id.set(Some(page_id.clone()));

        spawn_local(async move {
            let result = if publish {
                api::publish_page(token_value, tenant_value, page_id).await
            } else {
                api::unpublish_page(token_value, tenant_value, page_id).await
            };

            if let Err(err) = result {
                set_submit_error.set(Some(format!("Failed to update page status: {err}")));
            } else {
                set_refresh_nonce.update(|value| *value += 1);
            }

            set_busy_page_id.set(None);
        });
    });

    let delete_page = Callback::new(move |page_id: String| {
        let token_value = token.get_untracked();
        let tenant_value = tenant.get_untracked();
        set_submit_error.set(None);
        set_busy_page_id.set(Some(page_id.clone()));

        spawn_local(async move {
            match api::delete_page(token_value, tenant_value, page_id).await {
                Ok(true) => set_refresh_nonce.update(|value| *value += 1),
                Ok(false) => {
                    set_submit_error.set(Some("Delete page returned false.".to_string()));
                }
                Err(err) => {
                    set_submit_error.set(Some(format!("Failed to delete page: {err}")));
                }
            }
            set_busy_page_id.set(None);
        });
    });

    view! {
        <div class="space-y-6">
            <header class="flex flex-col gap-4 rounded-2xl border border-border bg-card p-6 shadow-sm lg:flex-row lg:items-start lg:justify-between">
                <div class="space-y-2">
                    <span class="inline-flex items-center rounded-full border border-border px-3 py-1 text-xs font-medium text-muted-foreground">
                        "pages"
                    </span>
                    <h1 class="text-2xl font-semibold text-card-foreground">"Pages Builder"</h1>
                    <p class="max-w-2xl text-sm text-muted-foreground">
                        "Canonical module-owned admin slice: list, create, publish and delete pages through the pages module GraphQL contract."
                    </p>
                </div>
            </header>

            <section class="grid gap-6 xl:grid-cols-[minmax(0,1fr)_22rem]">
                <div class="rounded-2xl border border-border bg-card p-6 shadow-sm">
                    <div class="mb-4 flex items-start justify-between gap-4">
                        <div>
                            <h2 class="text-lg font-semibold text-card-foreground">"Pages"</h2>
                            <p class="text-sm text-muted-foreground">
                                "This list is loaded from the module package itself, not from apps/admin."
                            </p>
                        </div>
                    </div>

                    <Suspense
                        fallback=move || view! {
                            <div class="space-y-2">
                                {(0..4).map(|_| view! {
                                    <div class="h-14 animate-pulse rounded-xl bg-muted"></div>
                                }).collect_view()}
                            </div>
                        }
                    >
                        {move || {
                            pages_resource.get().map(|result| {
                                match result {
                                    Ok(page_list) => view! {
                                        <PagesTable
                                            items=page_list.items
                                            total=page_list.total
                                            busy_page_id=busy_page_id.get()
                                            on_toggle_publish=publish_page
                                            on_delete=delete_page
                                        />
                                    }.into_any(),
                                    Err(err) => view! {
                                        <div class="rounded-xl border border-destructive/30 bg-destructive/10 px-4 py-3 text-sm text-destructive">
                                            {format!("Failed to load pages: {err}")}
                                        </div>
                                    }.into_any(),
                                }
                            })
                        }}
                    </Suspense>
                </div>

                <section class="rounded-2xl border border-border bg-card p-6 shadow-sm">
                    <div class="space-y-1">
                        <h2 class="text-lg font-semibold text-card-foreground">"Create page"</h2>
                        <p class="text-sm text-muted-foreground">
                            "A minimal working create flow for the module-owned package."
                        </p>
                    </div>

                    <form class="mt-5 space-y-4" on:submit=create_page>
                        <label class="block space-y-2">
                            <span class="text-sm font-medium text-card-foreground">"Title"</span>
                            <input
                                type="text"
                                class="w-full rounded-lg border border-input bg-background px-3 py-2 text-sm"
                                prop:value=title
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    if slug.get_untracked().trim().is_empty() {
                                        set_slug.set(slugify(value.as_str()));
                                    }
                                    set_title.set(value);
                                }
                            />
                        </label>

                        <label class="block space-y-2">
                            <span class="text-sm font-medium text-card-foreground">"Slug"</span>
                            <input
                                type="text"
                                class="w-full rounded-lg border border-input bg-background px-3 py-2 text-sm"
                                prop:value=slug
                                on:input=move |ev| set_slug.set(event_target_value(&ev))
                            />
                        </label>

                        <label class="block space-y-2">
                            <span class="text-sm font-medium text-card-foreground">"Locale"</span>
                            <input
                                type="text"
                                class="w-full rounded-lg border border-input bg-background px-3 py-2 text-sm"
                                prop:value=locale
                                on:input=move |ev| set_locale.set(event_target_value(&ev))
                            />
                        </label>

                        <label class="block space-y-2">
                            <span class="text-sm font-medium text-card-foreground">"Body"</span>
                            <textarea
                                class="min-h-40 w-full rounded-lg border border-input bg-background px-3 py-2 text-sm"
                                prop:value=body
                                on:input=move |ev| set_body.set(event_target_value(&ev))
                            />
                        </label>

                        <label class="flex items-center gap-2 text-sm text-card-foreground">
                            <input
                                type="checkbox"
                                prop:checked=publish_now
                                on:change=move |ev| set_publish_now.set(event_target_checked(&ev))
                            />
                            "Publish immediately"
                        </label>

                        <Show when=move || submit_error.get().is_some()>
                            <div class="rounded-xl border border-destructive/30 bg-destructive/10 px-4 py-3 text-sm text-destructive">
                                {move || submit_error.get().unwrap_or_default()}
                            </div>
                        </Show>

                        <button
                            type="submit"
                            class="inline-flex w-full items-center justify-center rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground transition hover:bg-primary/90 disabled:opacity-50"
                            disabled=move || busy_page_id.get().as_deref() == Some("create")
                        >
                            {move || {
                                if busy_page_id.get().as_deref() == Some("create") {
                                    "Creating..."
                                } else {
                                    "Create page"
                                }
                            }}
                        </button>
                    </form>
                </section>
            </section>
        </div>
    }
}

#[component]
fn PagesTable(
    items: Vec<PageListItem>,
    total: u64,
    busy_page_id: Option<String>,
    on_toggle_publish: Callback<(String, bool)>,
    on_delete: Callback<String>,
) -> impl IntoView {
    if items.is_empty() {
        return view! {
            <div class="rounded-xl border border-dashed border-border p-12 text-center">
                <p class="text-sm text-muted-foreground">
                    "No pages yet. Create the first one from the module package form."
                </p>
            </div>
        }
        .into_any();
    }

    view! {
        <div class="space-y-4">
            <div class="text-sm text-muted-foreground">{format!("{total} page(s)")}</div>
            <div class="overflow-hidden rounded-xl border border-border">
                <table class="w-full text-sm">
                    <thead class="border-b border-border bg-muted/50">
                        <tr>
                            <th class="px-4 py-3 text-left text-xs font-semibold uppercase tracking-wider text-muted-foreground">"Title"</th>
                            <th class="px-4 py-3 text-left text-xs font-semibold uppercase tracking-wider text-muted-foreground">"Slug"</th>
                            <th class="px-4 py-3 text-left text-xs font-semibold uppercase tracking-wider text-muted-foreground">"Status"</th>
                            <th class="px-4 py-3 text-left text-xs font-semibold uppercase tracking-wider text-muted-foreground">"Updated"</th>
                            <th class="px-4 py-3"></th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-border">
                        {items
                            .into_iter()
                            .map(|page| {
                                let page_id = page.id.clone();
                                let is_busy = busy_page_id.as_deref() == Some(page_id.as_str());
                                let is_published = page.status.eq_ignore_ascii_case("published");
                                view! {
                                    <tr class="transition-colors hover:bg-muted/30">
                                        <td class="px-4 py-3">
                                            <div class="font-medium text-foreground">
                                                {page.title.unwrap_or_else(|| "Untitled page".to_string())}
                                            </div>
                                            <div class="text-xs text-muted-foreground">{page.template}</div>
                                        </td>
                                        <td class="px-4 py-3 text-muted-foreground">
                                            {page.slug.unwrap_or_else(|| "-".to_string())}
                                        </td>
                                        <td class="px-4 py-3">
                                            <StatusBadge status=page.status.clone() />
                                        </td>
                                        <td class="px-4 py-3 text-xs text-muted-foreground">{page.updated_at}</td>
                                        <td class="px-4 py-3">
                                            <div class="flex justify-end gap-2">
                                                <button
                                                    class="rounded-lg border border-border px-3 py-1 text-xs font-medium text-foreground transition hover:bg-accent hover:text-accent-foreground disabled:opacity-50"
                                                    disabled=is_busy
                                                    on:click={
                                                        let page_id = page.id.clone();
                                                        move |_| on_toggle_publish.run((page_id.clone(), !is_published))
                                                    }
                                                >
                                                    {if is_busy {
                                                        "...".to_string()
                                                    } else if is_published {
                                                        "Unpublish".to_string()
                                                    } else {
                                                        "Publish".to_string()
                                                    }}
                                                </button>
                                                <button
                                                    class="rounded-lg border border-destructive/30 px-3 py-1 text-xs font-medium text-destructive transition hover:bg-destructive/10 disabled:opacity-50"
                                                    disabled=is_busy
                                                    on:click={
                                                        let page_id = page.id.clone();
                                                        move |_| on_delete.run(page_id.clone())
                                                    }
                                                >
                                                    "Delete"
                                                </button>
                                            </div>
                                        </td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </div>
        </div>
    }
    .into_any()
}

#[component]
fn StatusBadge(status: String) -> impl IntoView {
    let normalized = status.to_lowercase();
    let class_name = match normalized.as_str() {
        "published" => {
            "bg-emerald-50 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-400"
        }
        "archived" => "bg-muted text-muted-foreground",
        _ => "bg-primary/10 text-primary",
    };

    view! {
        <span class=format!("inline-flex rounded-full px-2.5 py-0.5 text-xs font-semibold {class_name}")>
            {status}
        </span>
    }
}

fn slugify(value: &str) -> String {
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
