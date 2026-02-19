use leptos::prelude::*;

/// Header with logo and navigation
#[component]
pub fn HeaderSnippet() -> impl IntoView {
    view! {
        <header class="py-6 px-4">
            <div class="max-w-6xl mx-auto flex items-center justify-between">
                <a href="/" class="flex items-center space-x-3 hover:opacity-90 transition">
                    <div class="w-10 h-10 bg-amber-500 rounded-lg flex items-center justify-center">
                        <svg
                            class="w-6 h-6 text-zinc-950"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                            ></path>
                        </svg>
                    </div>
                    <h1 class="text-2xl font-bold text-zinc-100">"PIXLET"</h1>
                </a>
                <nav class="hidden md:flex space-x-6">
                    <a href="/features" class="text-zinc-400 hover:text-amber-500 transition">
                        "Features"
                    </a>
                    <a href="/about" class="text-zinc-400 hover:text-amber-500 transition">
                        "About"
                    </a>
                </nav>
            </div>
        </header>
    }
}
