use leptos::prelude::*;

/// Small pill label. Amber by default, `muted` for gray.
#[component]
pub fn Badge(
    /// Gray variant for secondary info.
    #[prop(optional)]
    muted: bool,
    /// Label text.
    children: Children,
) -> impl IntoView {
    view! {
        <span
            class="px-4 py-2 border rounded-full font-semibold"
            class=("bg-amber-500/10", !muted)
            class=("border-amber-500/30", !muted)
            class:text-amber-500=!muted
            class:bg-zinc-800=muted
            class:border-zinc-700=muted
            class:text-zinc-400=muted
        >
            {children()}
        </span>
    }
}
