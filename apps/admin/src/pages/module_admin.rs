use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

use crate::app::modules::page_for_route_segment;
use crate::app::providers::enabled_modules::use_enabled_modules_context;

#[derive(Params, PartialEq)]
struct ModuleAdminParams {
    module_slug: Option<String>,
}

#[component]
pub fn ModuleAdminPage() -> impl IntoView {
    let params = use_params::<ModuleAdminParams>();
    let enabled_modules = use_enabled_modules_context();

    let route_segment = Signal::derive(move || {
        params.with(|params| {
            params
                .as_ref()
                .ok()
                .and_then(|params| params.module_slug.clone())
                .unwrap_or_default()
        })
    });

    let any_page = Signal::derive(move || page_for_route_segment(&route_segment.get(), None));
    let enabled_page = Signal::derive(move || {
        let enabled = enabled_modules.modules.get();
        page_for_route_segment(&route_segment.get(), Some(&enabled))
    });
    let is_loading = Signal::derive(move || enabled_modules.is_loading.get());

    view! {
        <section class="px-10 py-8">
            {move || {
                if is_loading.get() {
                    return view! {
                        <div class="space-y-4">
                            <div class="h-10 w-64 animate-pulse rounded-xl bg-muted"></div>
                            <div class="h-64 animate-pulse rounded-2xl bg-muted"></div>
                        </div>
                    }
                    .into_any();
                }

                match (any_page.get(), enabled_page.get()) {
                    (_, Some(page)) => (page.render)(),
                    (Some(page), None) => view! {
                        <div class="rounded-2xl border border-border bg-card p-8 shadow-sm">
                            <div class="max-w-2xl space-y-3">
                                <span class="inline-flex items-center rounded-full border border-border px-3 py-1 text-xs font-medium text-muted-foreground">
                                    {page.module_slug.to_string()}
                                </span>
                                <h1 class="text-2xl font-semibold text-card-foreground">{page.title.to_string()}</h1>
                                <p class="text-sm text-muted-foreground">
                                    "Этот модуль зарегистрирован в admin host, но выключен для текущего tenant."
                                </p>
                            </div>
                        </div>
                    }
                    .into_any(),
                    (None, None) => view! {
                        <div class="rounded-2xl border border-border bg-card p-8 shadow-sm">
                            <div class="max-w-2xl space-y-3">
                                <span class="inline-flex items-center rounded-full border border-border px-3 py-1 text-xs font-medium text-muted-foreground">
                                    "module route"
                                </span>
                                <h1 class="text-2xl font-semibold text-card-foreground">"Admin page not found"</h1>
                                <p class="text-sm text-muted-foreground">
                                    "Для этого route segment нет зарегистрированного module-owned admin UI."
                                </p>
                            </div>
                        </div>
                    }
                    .into_any(),
                }
            }}
        </section>
    }
}
