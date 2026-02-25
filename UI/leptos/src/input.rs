use leptos::prelude::*;

use crate::types::Size;

#[component]
pub fn Input(
    #[prop(default = "text")] r#type: &'static str,
    #[prop(default = Size::Md)] size: Size,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] invalid: bool,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional)] value: Option<ReadSignal<String>>,
    #[prop(optional)] set_value: Option<WriteSignal<String>>,
    #[prop(optional)] prefix: Option<AnyView>,
    #[prop(optional)] suffix: Option<AnyView>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] name: String,
) -> impl IntoView {
    let size_cls = match size {
        Size::Sm => "h-8 text-xs px-2",
        Size::Md => "h-9 text-sm px-3",
        Size::Lg | Size::Icon => "h-10 text-base px-4",
    };

    let state_cls = if invalid {
        "border-[hsl(var(--iu-danger))] focus-visible:ring-[hsl(var(--iu-danger))]"
    } else {
        "border-[hsl(var(--iu-border))] focus-visible:ring-[hsl(var(--iu-primary))]"
    };

    let input_cls = format!(
        "w-full rounded-[var(--iu-radius-md)] border bg-[hsl(var(--iu-bg))] text-[hsl(var(--iu-fg))] \
         transition-colors focus-visible:outline-none focus-visible:ring-2 \
         disabled:cursor-not-allowed disabled:opacity-50 {} {} {}",
        size_cls, state_cls, class
    );

    let has_affix = prefix.is_some() || suffix.is_some();

    if has_affix {
        view! {
            <div class="relative flex items-center">
                {prefix.map(|p| view! {
                    <span class="pointer-events-none absolute left-3 text-[hsl(var(--iu-muted-fg))]">{p}</span>
                })}
                <input
                    type=r#type
                    class=format!("{} {}", input_cls, if prefix.is_some() { "pl-9" } else { "" })
                    disabled=disabled
                    aria-invalid=invalid
                    placeholder=placeholder
                    name=name
                    prop:value=move || value.map(|v| v.get()).unwrap_or_default()
                    on:input=move |ev| {
                        if let Some(set) = set_value {
                            set.set(event_target_value(&ev));
                        }
                    }
                />
                {suffix.map(|s| view! {
                    <span class="pointer-events-none absolute right-3 text-[hsl(var(--iu-muted-fg))]">{s}</span>
                })}
            </div>
        }.into_any()
    } else {
        view! {
            <input
                type=r#type
                class=input_cls
                disabled=disabled
                aria-invalid=invalid
                placeholder=placeholder
                name=name
                prop:value=move || value.map(|v| v.get()).unwrap_or_default()
                on:input=move |ev| {
                    if let Some(set) = set_value {
                        set.set(event_target_value(&ev));
                    }
                }
            />
        }.into_any()
    }
}
