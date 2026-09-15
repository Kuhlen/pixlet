use leptos::prelude::*;

use super::Icon;

/// Feature card: round icon, title, description.
#[component]
pub fn FeatureCard(
    /// Icon path from `ui::icon`.
    icon: &'static str,
    /// Card heading.
    title: &'static str,
    /// Body text.
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="card-surface rounded-2xl p-8 text-center hover:border-amber-500/50 transition">
            <div class="w-16 h-16 bg-amber-500/10 border border-amber-500/20 rounded-full flex items-center justify-center mx-auto mb-5">
                <Icon d=icon class="w-8 h-8 text-amber-500" />
            </div>
            <h3 class="text-xl font-bold text-zinc-100 mb-3">{title}</h3>
            <p class="text-zinc-400 leading-relaxed">{description}</p>
        </div>
    }
}
