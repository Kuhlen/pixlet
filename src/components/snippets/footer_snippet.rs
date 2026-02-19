use leptos::prelude::*;

/// Footer section
#[component]
pub fn FooterSnippet() -> impl IntoView {
    view! {
        <footer class="mt-16 py-8 text-center">
            <p class="text-zinc-500">"\u{00A9} 2026 Pixlet. All rights reserved."</p>
        </footer>
    }
}
