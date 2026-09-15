use leptos::prelude::*;

use crate::components::ui::{Badge, Button, Card};
use crate::core::mass_clicker;

const STACK: [&str; 5] = [
    "Rust",
    "Leptos 0.8",
    "WebAssembly",
    "Tailwind CSS v4",
    "Trunk",
];

const FACTS: [(&str, &str); 4] = [
    ("0", "Servers harmed in the making of this app"),
    ("100%", "Of pixels converted with mass"),
    ("6", "Image formats supported (and counting... maybe)"),
    (
        "\u{221E}",
        "Conversions you can do (we literally can't stop you)",
    ),
];

/// About page: story, stack, facts, clicker easter egg.
#[component]
pub fn About() -> impl IntoView {
    let clicks = RwSignal::new(0u64);

    view! {
        <main class="max-w-4xl mx-auto px-4 py-12">
            <div class="text-center mb-16">
                <h2 class="text-3xl md:text-5xl font-bold text-zinc-100 mb-4">"About Pixlet"</h2>
                <p class="text-lg md:text-xl text-zinc-400">
                    "The image converter that probably didn't need to exist, but here we are."
                </p>
            </div>

            <div class="space-y-8">
                <Card title="The Story">
                    <div class="space-y-4 text-zinc-400 leading-relaxed">
                        <p>
                            "Pixlet was born out of a simple desire: to learn WebAssembly by actually building something with it. Not a hello-world, not a counter app - but something that does real work in the browser."
                        </p>
                        <p>
                            "The idea was straightforward: take Rust's excellent "
                            <span class="text-amber-500 font-mono">"image"</span>
                            " crate, compile it to WASM, slap a reactive UI on top with "
                            <span class="text-amber-500 font-mono">"Leptos"</span>
                            ", and see if we can convert images entirely client-side without ever talking to a server."
                        </p>
                        <p>
                            "Turns out, you can. And it's surprisingly fast. The whole app is a static site - no backend, no API, no database. Just HTML, CSS, and a chunk of WebAssembly doing the heavy lifting."
                        </p>
                        <p>
                            "Is it over-engineered for an image converter? Probably. Did we learn a ton about WASM, Leptos, and the Rust web ecosystem? Absolutely. And that was the whole point."
                        </p>
                    </div>
                </Card>

                <Card title="Tech Stack">
                    <div class="flex flex-wrap gap-3">
                        {STACK.map(|name| view! { <Badge>{name}</Badge> }).collect_view()}
                        <Badge muted=true>"0 lines of JavaScript"</Badge>
                    </div>
                </Card>

                <Card title="Fun Facts">
                    <div class="grid sm:grid-cols-2 gap-6">
                        {FACTS
                            .map(|(value, caption)| {
                                view! {
                                    <div class="text-center p-4">
                                        <div class="text-4xl font-bold text-amber-500 mb-2">
                                            {value}
                                        </div>
                                        <p class="text-zinc-400">{caption}</p>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </Card>

                <Card>
                    <h3 class="text-2xl font-bold text-zinc-100 mb-2">
                        "The Electron Mass Clicker"
                    </h3>
                    <p class="text-zinc-500 mb-6">
                        "Ever wondered how many mass of an electron you can accumulate by clicking a button? No? Well, now you can find out."
                    </p>
                    <div class="text-center">
                        <Button large=true on_click=move |_| clicks.update(|c| *c += 1)>
                            "Click to Add Mass"
                        </Button>
                        <div class="mt-6 space-y-2">
                            <p class="text-zinc-400">
                                "Clicks: "
                                <span class="text-amber-500 font-bold font-mono">
                                    {move || clicks.get()}
                                </span>
                            </p>
                            <p class="text-zinc-400">
                                "Total mass: "
                                <span class="text-amber-500 font-bold font-mono">
                                    {move || mass_clicker::total_mass(clicks.get())}
                                </span> " kg"
                            </p>
                            <p class="text-zinc-500 text-sm italic">
                                {move || mass_clicker::message(clicks.get())}
                            </p>
                        </div>
                    </div>
                </Card>

                <div class="text-center text-zinc-600 text-sm">
                    <p>"Built with mass and mass of mass by someone learning Rust + WASM."</p>
                    <p class="mt-1">"No mass were mass during the mass of this mass."</p>
                </div>
            </div>
        </main>
    }
}
