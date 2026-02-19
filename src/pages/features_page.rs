use leptos::prelude::*;

use crate::components::snippets::footer_snippet::FooterSnippet;
use crate::components::snippets::header_snippet::HeaderSnippet;

/// Features Page
#[component]
pub fn Features() -> impl IntoView {
    view! {
        <div class="min-h-screen dark-bg">
            <HeaderSnippet />

            <main class="max-w-6xl mx-auto px-4 py-12">
                // Hero
                <div class="text-center mb-16">
                    <h2 class="text-3xl md:text-5xl font-bold text-zinc-100 mb-4">"Why Pixlet?"</h2>
                    <p class="text-lg md:text-xl text-zinc-400 max-w-2xl mx-auto px-2">
                        "A powerful image converter that runs entirely in your browser. No servers, no uploads, no nonsense."
                    </p>
                </div>

                // Feature cards grid
                <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8 max-w-5xl mx-auto">

                    // Lightning Fast
                    <div class="card-surface rounded-2xl p-8 text-center hover:border-amber-500/50 transition">
                        <div class="w-16 h-16 bg-amber-500/10 border border-amber-500/20 rounded-full flex items-center justify-center mx-auto mb-5">
                            <svg class="w-8 h-8 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                            </svg>
                        </div>
                        <h3 class="text-xl font-bold text-zinc-100 mb-3">"Lightning Fast"</h3>
                        <p class="text-zinc-400 leading-relaxed">
                            "Powered by WebAssembly, Pixlet converts your images at near-native speed directly in the browser. No waiting for server round-trips."
                        </p>
                    </div>

                    // 100% Private
                    <div class="card-surface rounded-2xl p-8 text-center hover:border-amber-500/50 transition">
                        <div class="w-16 h-16 bg-amber-500/10 border border-amber-500/20 rounded-full flex items-center justify-center mx-auto mb-5">
                            <svg class="w-8 h-8 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
                            </svg>
                        </div>
                        <h3 class="text-xl font-bold text-zinc-100 mb-3">"100% Private"</h3>
                        <p class="text-zinc-400 leading-relaxed">
                            "Your files never leave your device. All image processing happens locally in your browser — no data is ever uploaded to any server."
                        </p>
                    </div>

                    // Completely Free
                    <div class="card-surface rounded-2xl p-8 text-center hover:border-amber-500/50 transition">
                        <div class="w-16 h-16 bg-amber-500/10 border border-amber-500/20 rounded-full flex items-center justify-center mx-auto mb-5">
                            <svg class="w-8 h-8 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                            </svg>
                        </div>
                        <h3 class="text-xl font-bold text-zinc-100 mb-3">"Completely Free"</h3>
                        <p class="text-zinc-400 leading-relaxed">
                            "No subscriptions, no hidden fees, no premium tiers. Convert as many images as you want, forever. We mean it."
                        </p>
                    </div>

                    // Multiple Formats
                    <div class="card-surface rounded-2xl p-8 text-center hover:border-amber-500/50 transition">
                        <div class="w-16 h-16 bg-amber-500/10 border border-amber-500/20 rounded-full flex items-center justify-center mx-auto mb-5">
                            <svg class="w-8 h-8 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                            </svg>
                        </div>
                        <h3 class="text-xl font-bold text-zinc-100 mb-3">"Multiple Formats"</h3>
                        <p class="text-zinc-400 leading-relaxed">
                            "Support for JPG, PNG, WebP, GIF, BMP, and TIFF. Convert between any of these formats with just a few clicks."
                        </p>
                    </div>

                    // No Upload Required
                    <div class="card-surface rounded-2xl p-8 text-center hover:border-amber-500/50 transition">
                        <div class="w-16 h-16 bg-amber-500/10 border border-amber-500/20 rounded-full flex items-center justify-center mx-auto mb-5">
                            <svg class="w-8 h-8 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636"></path>
                            </svg>
                        </div>
                        <h3 class="text-xl font-bold text-zinc-100 mb-3">"No Upload Required"</h3>
                        <p class="text-zinc-400 leading-relaxed">
                            "Unlike traditional converters, your files stay on your machine. Drag, drop, convert, download — all without a single network request."
                        </p>
                    </div>

                    // Zero Dependencies
                    <div class="card-surface rounded-2xl p-8 text-center hover:border-amber-500/50 transition">
                        <div class="w-16 h-16 bg-amber-500/10 border border-amber-500/20 rounded-full flex items-center justify-center mx-auto mb-5">
                            <svg class="w-8 h-8 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"></path>
                            </svg>
                        </div>
                        <h3 class="text-xl font-bold text-zinc-100 mb-3">"Zero Server Dependencies"</h3>
                        <p class="text-zinc-400 leading-relaxed">
                            "No backend, no API keys, no database. Just a static site with a WASM binary doing all the heavy lifting. Works offline too."
                        </p>
                    </div>
                </div>

                // How it works section
                <div class="mt-24 text-center">
                    <h2 class="text-3xl font-bold text-zinc-100 mb-12">"How It Works"</h2>
                    <div class="grid md:grid-cols-3 gap-8 max-w-4xl mx-auto">
                        <div>
                            <div class="w-12 h-12 bg-amber-500 text-zinc-950 rounded-full flex items-center justify-center mx-auto mb-4 text-xl font-bold">"1"</div>
                            <h4 class="text-lg font-semibold text-zinc-100 mb-2">"Drop Your Image"</h4>
                            <p class="text-zinc-400">"Drag and drop or click to select the image you want to convert."</p>
                        </div>
                        <div>
                            <div class="w-12 h-12 bg-amber-500 text-zinc-950 rounded-full flex items-center justify-center mx-auto mb-4 text-xl font-bold">"2"</div>
                            <h4 class="text-lg font-semibold text-zinc-100 mb-2">"Pick a Format"</h4>
                            <p class="text-zinc-400">"Choose your desired output format and adjust quality settings if needed."</p>
                        </div>
                        <div>
                            <div class="w-12 h-12 bg-amber-500 text-zinc-950 rounded-full flex items-center justify-center mx-auto mb-4 text-xl font-bold">"3"</div>
                            <h4 class="text-lg font-semibold text-zinc-100 mb-2">"Download"</h4>
                            <p class="text-zinc-400">"Hit convert and your file downloads instantly. That's it. No account needed."</p>
                        </div>
                    </div>
                </div>

                // CTA
                <div class="mt-24 text-center">
                    <a
                        href="/"
                        class="inline-flex items-center px-8 py-4 bg-amber-500 text-zinc-950 font-bold text-lg rounded-full hover:bg-amber-400 transition transform hover:scale-105"
                    >
                        "Start Converting"
                        <svg class="w-5 h-5 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6"></path>
                        </svg>
                    </a>
                </div>
            </main>

            <FooterSnippet />
        </div>
    }
}
