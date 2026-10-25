use leptos::prelude::*;

/// Primary action button, amber. `large` for CTAs.
#[component]
pub fn Button(
    /// Disabled while an action is running.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Bigger padding and text.
    #[prop(optional)]
    large: bool,
    /// Fired on click.
    #[prop(into)]
    on_click: Callback<()>,
    /// Button content.
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class="px-8 bg-amber-500 text-zinc-950 font-bold rounded-full hover:bg-amber-400 transition transform hover:scale-105 active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
            class:py-3=!large
            class:text-base=!large
            class:py-4=large
            class:text-lg=large
            class=("md:px-12", large)
            disabled=disabled
            on:click=move |_| on_click.run(())
        >
            {children()}
        </button>
    }
}
