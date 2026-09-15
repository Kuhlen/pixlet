mod converter_card;
mod feature_grid;
mod hero;

use leptos::prelude::*;

use converter_card::ConverterCard;
use feature_grid::FeatureGrid;
use hero::Hero;

/// Landing page with the converter.
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main class="max-w-6xl mx-auto px-4 py-12">
            <Hero />
            <ConverterCard />
            <FeatureGrid />
        </main>
    }
}
