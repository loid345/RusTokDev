use leptos::prelude::*;

#[component]
pub fn Checkbox(
    #[prop(optional)] checked: Option<ReadSignal<bool>>,
    #[prop(optional)] set_checked: Option<WriteSignal<bool>>,
    #[prop(default = false)] indeterminate: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] name: String,
) -> impl IntoView {
    view! {
        <input
            type="checkbox"
            id=id
            name=name
            disabled=disabled
            class=format!(
                "h-4 w-4 rounded-[var(--iu-radius-sm)] border border-[hsl(var(--iu-border))] \
                 bg-[hsl(var(--iu-bg))] text-[hsl(var(--iu-primary))] \
                 focus-visible:outline-none focus-visible:ring-2 \
                 focus-visible:ring-[hsl(var(--iu-primary))] focus-visible:ring-offset-2 \
                 disabled:cursor-not-allowed disabled:opacity-50 \
                 checked:bg-[hsl(var(--iu-primary))] checked:border-[hsl(var(--iu-primary))] {}",
                class
            )
            prop:checked=move || checked.map(|c| c.get()).unwrap_or(false)
            prop:indeterminate=indeterminate
            on:change=move |ev| {
                if let Some(set) = set_checked {
                    use leptos::ev::Event;
                    let target = ev.target().unwrap();
                    let input = target.unchecked_into::<web_sys::HtmlInputElement>();
                    set.set(input.checked());
                }
            }
        />
    }
}
