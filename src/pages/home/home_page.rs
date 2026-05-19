use leptos::prelude::*;

use crate::components::snippets::footer::FooterSnippet;
use crate::components::snippets::header::HeaderSnippet;
use crate::pages::home::converter_card::ConverterCardComponent;
use crate::pages::home::features::FeaturesComponent;
use crate::pages::home::hero::HeroComponent;

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
