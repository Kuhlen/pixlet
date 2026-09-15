use leptos::prelude::*;
use leptos_router::components::A;

use crate::app::HOME;
use crate::components::ui::{Icon, icon};

/// 404 fallback.
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <main class="max-w-6xl mx-auto px-4 py-12 flex flex-col items-center justify-center min-h-[60vh]">
            <div class="text-center">
                <div class="w-20 h-20 bg-amber-500/10 rounded-2xl flex items-center justify-center mx-auto mb-8">
                    <Icon d=icon::FACE_SAD class="w-10 h-10 text-amber-500" />
                </div>
                <h2 class="text-5xl md:text-7xl font-bold text-amber-500 mb-4">"404"</h2>
                <h3 class="text-2xl md:text-3xl font-bold text-zinc-100 mb-4">"Page Not Found"</h3>
                <p class="text-lg md:text-xl text-zinc-400 max-w-md mx-auto mb-8 px-2">
                    "Oops! The page you're looking for doesn't exist or has been moved."
                </p>
                <A
                    href=HOME
                    attr:class="inline-flex items-center px-6 py-3 bg-amber-500 text-zinc-950 font-semibold rounded-lg hover:bg-amber-400 transition"
                >
                    <Icon d=icon::HOME class="w-5 h-5 mr-2" />
                    "Back to Home"
                </A>
            </div>
        </main>
    }
}
