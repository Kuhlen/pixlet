use leptos::prelude::*;

use crate::components::snippets::footer_snippet::FooterSnippet;
use crate::components::snippets::header_snippet::HeaderSnippet;

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="min-h-screen dark-bg">
            <HeaderSnippet />

            <main class="max-w-6xl mx-auto px-4 py-12 flex flex-col items-center justify-center min-h-[60vh]">
                <div class="text-center">
                    <div class="w-20 h-20 bg-amber-500/10 rounded-2xl flex items-center justify-center mx-auto mb-8">
                        <svg
                            class="w-10 h-10 text-amber-500"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                            ></path>
                        </svg>
                    </div>
                    <h2 class="text-5xl md:text-7xl font-bold text-amber-500 mb-4">"404"</h2>
                    <h3 class="text-2xl md:text-3xl font-bold text-zinc-100 mb-4">"Page Not Found"</h3>
                    <p class="text-lg md:text-xl text-zinc-400 max-w-md mx-auto mb-8 px-2">
                        "Oops! The page you're looking for doesn't exist or has been moved."
                    </p>
                    <a
                        href="/"
                        class="inline-flex items-center px-6 py-3 bg-amber-500 text-zinc-950 font-semibold rounded-lg hover:bg-amber-400 transition"
                    >
                        <svg
                            class="w-5 h-5 mr-2"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-4 0a1 1 0 01-1-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 01-1 1h-2z"
                            ></path>
                        </svg>
                        "Back to Home"
                    </a>
                </div>
            </main>

            <FooterSnippet />
        </div>
    }
}
