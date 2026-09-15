use leptos::prelude::*;
use leptos_router::components::A;

use crate::app::{ABOUT, FEATURES, HOME};
use crate::components::ui::{Icon, icon};

const LINKS: [(&str, &str); 2] = [(FEATURES, "Features"), (ABOUT, "About")];

/// Logo + navigation. Mobile menu toggles via hamburger.
#[component]
pub fn Navbar() -> impl IntoView {
    let mobile_open = RwSignal::new(false);

    view! {
        <header class="py-6 px-4 relative">
            <div class="max-w-6xl mx-auto flex items-center justify-between">
                <A href=HOME attr:class="flex items-center space-x-3 hover:opacity-90 transition">
                    <div class="w-10 h-10 bg-amber-500 rounded-lg flex items-center justify-center">
                        <Icon d=icon::IMAGE class="w-6 h-6 text-zinc-950" />
                    </div>
                    <h1 class="text-2xl font-bold text-zinc-100">"PIXLET"</h1>
                </A>

                <nav class="hidden md:flex space-x-6">
                    {LINKS
                        .map(|(href, label)| {
                            view! {
                                <A
                                    href=href
                                    attr:class="text-zinc-400 hover:text-amber-500 transition"
                                >
                                    {label}
                                </A>
                            }
                        })
                        .collect_view()}
                </nav>

                <button
                    class="md:hidden text-zinc-400 hover:text-amber-500 transition p-2"
                    aria-label="Toggle menu"
                    aria-expanded=move || mobile_open.get().to_string()
                    on:click=move |_| mobile_open.update(|v| *v = !*v)
                >
                    <Show
                        when=move || mobile_open.get()
                        fallback=|| view! { <Icon d=icon::MENU class="w-6 h-6" /> }
                    >
                        <Icon d=icon::CLOSE class="w-6 h-6" />
                    </Show>
                </button>
            </div>

            <Show when=move || mobile_open.get()>
                <nav class="md:hidden mt-4 mx-auto max-w-6xl card-surface rounded-xl p-4 flex flex-col space-y-3">
                    {LINKS
                        .map(|(href, label)| {
                            view! {
                                <A
                                    href=href
                                    attr:class="text-zinc-400 hover:text-amber-500 transition py-2 px-3 rounded-lg hover:bg-zinc-800"
                                    on:click=move |_| mobile_open.set(false)
                                >
                                    {label}
                                </A>
                            }
                        })
                        .collect_view()}
                </nav>
            </Show>
        </header>
    }
}
