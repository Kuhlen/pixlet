use leptos::prelude::*;
use leptos_router::components::A;

use crate::app::HOME;
use crate::components::ui::{FeatureCard, Icon, icon};

const FEATURES: [(&str, &str, &str); 6] = [
    (
        icon::BOLT,
        "Lightning Fast",
        "Powered by WebAssembly, Pixlet converts your images at near-native speed directly in the browser. No waiting for server round-trips.",
    ),
    (
        icon::LOCK,
        "100% Private",
        "Your files never leave your device. All image processing happens locally in your browser - no data is ever uploaded to any server.",
    ),
    (
        icon::COIN,
        "Completely Free",
        "No subscriptions, no hidden fees, no premium tiers. Convert as many images as you want, forever. We mean it.",
    ),
    (
        icon::IMAGE,
        "Multiple Formats",
        "Support for JPG, PNG, WebP, GIF, BMP, and TIFF. Convert between any of these formats with just a few clicks.",
    ),
    (
        icon::BAN,
        "No Upload Required",
        "Unlike traditional converters, your files stay on your machine. Drag, drop, convert, download - all without a single network request.",
    ),
    (
        icon::SPARKLES,
        "Zero Server Dependencies",
        "No backend, no API keys, no database. Just a static site with a WASM binary doing all the heavy lifting. Works offline too.",
    ),
];

const STEPS: [(&str, &str); 3] = [
    (
        "Drop Your Image",
        "Drag and drop or click to select the image you want to convert.",
    ),
    (
        "Pick a Format",
        "Choose your desired output format and adjust quality settings if needed.",
    ),
    (
        "Download",
        "Hit convert and your file downloads instantly. That's it. No account needed.",
    ),
];

/// Features page: card grid, how it works, CTA to home.
#[component]
pub fn Features() -> impl IntoView {
    view! {
        <main class="max-w-6xl mx-auto px-4 py-12">
            <div class="text-center mb-16">
                <h2 class="text-3xl md:text-5xl font-bold text-zinc-100 mb-4">"Why Pixlet?"</h2>
                <p class="text-lg md:text-xl text-zinc-400 max-w-2xl mx-auto px-2">
                    "A powerful image converter that runs entirely in your browser. No servers, no uploads, no nonsense."
                </p>
            </div>

            <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8 max-w-5xl mx-auto">
                {FEATURES
                    .map(|(icon, title, description)| {
                        view! { <FeatureCard icon title description /> }
                    })
                    .collect_view()}
            </div>

            <div class="mt-24 text-center">
                <h2 class="text-3xl font-bold text-zinc-100 mb-12">"How It Works"</h2>
                <div class="grid md:grid-cols-3 gap-8 max-w-4xl mx-auto">
                    {STEPS
                        .iter()
                        .enumerate()
                        .map(|(i, (title, description))| {
                            view! {
                                <div>
                                    <div class="w-12 h-12 bg-amber-500 text-zinc-950 rounded-full flex items-center justify-center mx-auto mb-4 text-xl font-bold">
                                        {i + 1}
                                    </div>
                                    <h4 class="text-lg font-semibold text-zinc-100 mb-2">
                                        {*title}
                                    </h4>
                                    <p class="text-zinc-400">{*description}</p>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>

            <div class="mt-24 text-center">
                <A
                    href=HOME
                    attr:class="inline-flex items-center px-8 py-4 bg-amber-500 text-zinc-950 font-bold text-lg rounded-full hover:bg-amber-400 transition transform hover:scale-105"
                >
                    "Start Converting"
                    <Icon d=icon::ARROW_RIGHT class="w-5 h-5 ml-2" />
                </A>
            </div>
        </main>
    }
}
