use leptos::children::Children;
use leptos::prelude::*;

#[component]
pub fn Label(
    #[prop(default = false)] required: bool,
    #[prop(optional)] r#for: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let base_classes = "block text-sm font-medium text-gray-700";
    let additional_classes = class.unwrap_or("");
    let full_class = format!("{} {}", base_classes, additional_classes);

    view! {
        <label
            for=r#for.unwrap_or("")
            class=full_class
        >
            {children()}
            {move || required.then(|| view! { <span class="text-red-500 ml-1">"*"</span> })}
        </label>
    }
}
