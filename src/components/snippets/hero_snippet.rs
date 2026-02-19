use leptos::prelude::*;

/// Hero section with title and subtitle
#[component]
pub fn HeroSnippet() -> impl IntoView {
    view! {
        <div class="text-center mb-12">
            <h2 class="text-5xl font-bold text-zinc-100 mb-4">"Convert Images Instantly"</h2>
            <p class="text-xl text-zinc-400 max-w-2xl mx-auto">
                "Transform your images to any format in seconds. Fast, secure, and completely free."
            </p>
        </div>
    }
}
