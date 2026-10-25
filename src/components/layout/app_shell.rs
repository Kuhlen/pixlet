use leptos::prelude::*;
use leptos_router::components::Outlet;

use super::{Footer, Navbar};

/// Layout for every page, mounted via `ParentRoute`. Pages render into `Outlet`.
/// Hrefs come from `app.rs`, layout never imports route paths.
#[component]
pub fn AppShell(home: &'static str, features: &'static str, about: &'static str) -> impl IntoView {
    view! {
        <div class="min-h-screen dark-bg">
            <Navbar home features about />
            <Outlet />
            <Footer />
        </div>
    }
}
