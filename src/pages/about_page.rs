use leptos::prelude::*;

use crate::components::snippets::footer_snippet::FooterSnippet;
use crate::components::snippets::header_snippet::HeaderSnippet;

/// About Page
#[component]
pub fn About() -> impl IntoView {
    let (click_count, set_click_count) = signal(0u64);
    let (electron_mass, _) = signal(9.109e-31_f64); // mass of an electron in kg

    let total_mass = move || {
        let clicks = click_count.get();
        let mass = clicks as f64 * electron_mass.get();
        if clicks == 0 {
            "0".to_string()
        } else {
            format!("{:.3e}", mass)
        }
    };

    let click_message = move || {
        let clicks = click_count.get();
        match clicks {
            0 => "Click the button to start massing!".to_string(),
            1..=9 => "Keep going...".to_string(),
            10..=49 => "You're getting somewhere!".to_string(),
            50..=99 => "That's a lot of electrons!".to_string(),
            100..=499 => "Are you okay?".to_string(),
            500..=999 => "This is concerning.".to_string(),
            _ => "You absolute legend.".to_string(),
        }
    };

    view! {
        <div class="min-h-screen dark-bg">
            <HeaderSnippet />

            <main class="max-w-4xl mx-auto px-4 py-12">
                // Hero
                <div class="text-center mb-16">
                    <h2 class="text-5xl font-bold text-zinc-100 mb-4">"About Pixlet"</h2>
                    <p class="text-xl text-zinc-400">
                        "The image converter that probably didn't need to exist, but here we are."
                    </p>
                </div>

                // Story section
                <div class="card-surface rounded-2xl p-8 mb-8">
                    <h3 class="text-2xl font-bold text-zinc-100 mb-4">"The Story"</h3>
                    <div class="space-y-4 text-zinc-400 leading-relaxed">
                        <p>
                            "Pixlet was born out of a simple desire: to learn WebAssembly by actually building something with it. Not a hello-world, not a counter app — but something that does real work in the browser."
                        </p>
                        <p>
                            "The idea was straightforward: take Rust's excellent "
                            <span class="text-amber-500 font-mono">"image"</span>
                            " crate, compile it to WASM, slap a reactive UI on top with "
                            <span class="text-amber-500 font-mono">"Leptos"</span>
                            ", and see if we can convert images entirely client-side without ever talking to a server."
                        </p>
                        <p>
                            "Turns out, you can. And it's surprisingly fast. The whole app is a static site — no backend, no API, no database. Just HTML, CSS, and a chunk of WebAssembly doing the heavy lifting."
                        </p>
                        <p>
                            "Is it over-engineered for an image converter? Probably. Did we learn a ton about WASM, Leptos, and the Rust web ecosystem? Absolutely. And that was the whole point."
                        </p>
                    </div>
                </div>

                // Tech Stack
                <div class="card-surface rounded-2xl p-8 mb-8">
                    <h3 class="text-2xl font-bold text-zinc-100 mb-6">"Tech Stack"</h3>
                    <div class="flex flex-wrap gap-3">
                        <span class="px-4 py-2 bg-amber-500/10 border border-amber-500/30 text-amber-500 rounded-full font-semibold">
                            "Rust"
                        </span>
                        <span class="px-4 py-2 bg-amber-500/10 border border-amber-500/30 text-amber-500 rounded-full font-semibold">
                            "Leptos 0.8"
                        </span>
                        <span class="px-4 py-2 bg-amber-500/10 border border-amber-500/30 text-amber-500 rounded-full font-semibold">
                            "WebAssembly"
                        </span>
                        <span class="px-4 py-2 bg-amber-500/10 border border-amber-500/30 text-amber-500 rounded-full font-semibold">
                            "Tailwind CSS v4"
                        </span>
                        <span class="px-4 py-2 bg-amber-500/10 border border-amber-500/30 text-amber-500 rounded-full font-semibold">
                            "Trunk"
                        </span>
                        <span class="px-4 py-2 bg-zinc-800 border border-zinc-700 text-zinc-400 rounded-full font-semibold">
                            "0 lines of JavaScript"
                        </span>
                    </div>
                </div>

                // Fun Facts
                <div class="card-surface rounded-2xl p-8 mb-8">
                    <h3 class="text-2xl font-bold text-zinc-100 mb-6">"Fun Facts"</h3>
                    <div class="grid sm:grid-cols-2 gap-6">
                        <div class="text-center p-4">
                            <div class="text-4xl font-bold text-amber-500 mb-2">"0"</div>
                            <p class="text-zinc-400">"Servers harmed in the making of this app"</p>
                        </div>
                        <div class="text-center p-4">
                            <div class="text-4xl font-bold text-amber-500 mb-2">"100%"</div>
                            <p class="text-zinc-400">"Of pixels converted with mass"</p>
                        </div>
                        <div class="text-center p-4">
                            <div class="text-4xl font-bold text-amber-500 mb-2">"6"</div>
                            <p class="text-zinc-400">"Image formats supported (and counting... maybe)"</p>
                        </div>
                        <div class="text-center p-4">
                            <div class="text-4xl font-bold text-amber-500 mb-2">"∞"</div>
                            <p class="text-zinc-400">"Conversions you can do (we literally can't stop you)"</p>
                        </div>
                    </div>
                </div>

                // Easter Egg: Electron mass clicker
                <div class="card-surface rounded-2xl p-8 mb-8">
                    <h3 class="text-2xl font-bold text-zinc-100 mb-2">"The Electron Mass Clicker"</h3>
                    <p class="text-zinc-500 mb-6">"Ever wondered how many mass of an electron you can accumulate by clicking a button? No? Well, now you can find out."</p>

                    <div class="text-center">
                        <button
                            on:click=move |_| set_click_count.update(|c| *c += 1)
                            class="px-8 py-4 bg-amber-500 text-zinc-950 font-bold text-lg rounded-full hover:bg-amber-400 transition transform hover:scale-105 active:scale-95"
                        >
                            "Click to Add Mass"
                        </button>

                        <div class="mt-6 space-y-2">
                            <p class="text-zinc-400">
                                "Clicks: "
                                <span class="text-amber-500 font-bold font-mono">{move || click_count.get()}</span>
                            </p>
                            <p class="text-zinc-400">
                                "Total mass: "
                                <span class="text-amber-500 font-bold font-mono">{total_mass}</span>
                                " kg"
                            </p>
                            <p class="text-zinc-500 text-sm italic">{click_message}</p>
                        </div>
                    </div>
                </div>

                // Disclaimer
                <div class="text-center text-zinc-600 text-sm">
                    <p>"Built with mass and mass of mass by someone learning Rust + WASM."</p>
                    <p class="mt-1">"No mass were mass during the mass of this mass."</p>
                </div>
            </main>

            <FooterSnippet />
        </div>
    }
}
