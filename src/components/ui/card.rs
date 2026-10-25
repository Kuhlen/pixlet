use leptos::prelude::*;

/// Dark surface with border. Extra attributes from the caller land on the root.
#[component]
pub fn Card(
    /// Optional heading, rendered as `h3`.
    #[prop(optional)]
    title: Option<&'static str>,
    /// Card body.
    children: Children,
) -> impl IntoView {
    view! {
        <div class="card-surface rounded-2xl p-8">
            {title.map(|t| view! { <h3 class="text-2xl font-bold text-zinc-100 mb-6">{t}</h3> })}
            {children()}
        </div>
    }
}
