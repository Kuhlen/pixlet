use leptos::prelude::*;

use crate::components::snippets::converter_card_snippet::ConverterCardSnippet;
use crate::components::snippets::features_snippet::FeaturesSnippet;
use crate::components::snippets::footer_snippet::FooterSnippet;
use crate::components::snippets::header_snippet::HeaderSnippet;
use crate::components::snippets::hero_snippet::HeroSnippet;

/// Home Page - Image Converter landing page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen dark-bg">
            <HeaderSnippet />

            <main class="max-w-6xl mx-auto px-4 py-12">
                <HeroSnippet />
                <ConverterCardSnippet />
                <FeaturesSnippet />
            </main>

            <FooterSnippet />
        </div>
    }
}
