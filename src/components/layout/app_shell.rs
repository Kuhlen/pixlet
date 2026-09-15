use leptos::prelude::*;
use leptos_router::components::Outlet;

use super::{Footer, Navbar};

/// Layout for every page, mounted via `ParentRoute`. Pages render into `Outlet`.
#[component]
pub fn AppShell() -> impl IntoView {
    view! {
        <div class="min-h-screen dark-bg">
            <Navbar />
            <Outlet />
            <Footer />
        </div>
    }
}
