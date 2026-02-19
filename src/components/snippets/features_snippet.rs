use leptos::prelude::*;

/// Features section with 3 feature cards
#[component]
pub fn FeaturesSnippet() -> impl IntoView {
    view! {
        <div class="grid md:grid-cols-3 gap-8 mt-16 max-w-5xl mx-auto">
            // Lightning Fast
            <div class="card-surface rounded-2xl p-6 text-center">
                <div class="w-16 h-16 bg-amber-500/10 border border-amber-500/20 rounded-full flex items-center justify-center mx-auto mb-4">
                    <svg
                        class="w-8 h-8 text-amber-500"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M13 10V3L4 14h7v7l9-11h-7z"
                        ></path>
                    </svg>
                </div>
                <h3 class="text-xl font-bold text-zinc-100 mb-2">"Lightning Fast"</h3>
                <p class="text-zinc-400">
                    "Convert images in seconds with our optimized processing engine"
                </p>
            </div>

            // 100% Secure
            <div class="card-surface rounded-2xl p-6 text-center">
                <div class="w-16 h-16 bg-amber-500/10 border border-amber-500/20 rounded-full flex items-center justify-center mx-auto mb-4">
                    <svg
                        class="w-8 h-8 text-amber-500"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                        ></path>
                    </svg>
                </div>
                <h3 class="text-xl font-bold text-zinc-100 mb-2">"100% Secure"</h3>
                <p class="text-zinc-400">
                    "Your files are processed locally and never stored on our servers"
                </p>
            </div>

            // Completely Free
            <div class="card-surface rounded-2xl p-6 text-center">
                <div class="w-16 h-16 bg-amber-500/10 border border-amber-500/20 rounded-full flex items-center justify-center mx-auto mb-4">
                    <svg
                        class="w-8 h-8 text-amber-500"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                        ></path>
                    </svg>
                </div>
                <h3 class="text-xl font-bold text-zinc-100 mb-2">"Completely Free"</h3>
                <p class="text-zinc-400">
                    "No subscriptions, no hidden fees. Convert unlimited images for free"
                </p>
            </div>
        </div>
    }
}
