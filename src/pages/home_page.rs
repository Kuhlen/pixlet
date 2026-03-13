use leptos::prelude::*;

use crate::components::converter_card::ConverterCardComponent;
use crate::components::features::FeaturesComponent;
use crate::components::hero::HeroComponent;
use crate::components::snippets::footer::FooterSnippet;
use crate::components::snippets::header::HeaderSnippet;

/// Home Page - Image Converter landing page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen dark-bg">
            <HeaderSnippet />

            <main class="max-w-6xl mx-auto px-4 py-12">
                <HeroComponent />
                <ConverterCardComponent />
                <FeaturesComponent />
            </main>

            <FooterSnippet />
        </div>
    }
}
