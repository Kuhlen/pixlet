use leptos::prelude::*;

use crate::components::ui::{FeatureCard, icon};

const FEATURES: [(&str, &str, &str); 3] = [
    (
        icon::BOLT,
        "Lightning Fast",
        "Convert images in seconds with our optimized processing engine",
    ),
    (
        icon::LOCK,
        "100% Secure",
        "Your files are processed locally and never stored on our servers",
    ),
    (
        icon::COIN,
        "Completely Free",
        "No subscriptions, no hidden fees. Convert unlimited images for free",
    ),
];

/// Three short feature cards below the converter.
#[component]
pub fn FeatureGrid() -> impl IntoView {
    view! {
        <div class="grid md:grid-cols-3 gap-8 mt-16 max-w-5xl mx-auto">
            {FEATURES
                .map(|(icon, title, description)| {
                    view! { <FeatureCard icon title description /> }
                })
                .collect_view()}
        </div>
    }
}
