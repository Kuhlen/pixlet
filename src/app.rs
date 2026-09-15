use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

use crate::components::layout::AppShell;
use crate::pages::{about::About, features::Features, home::Home, not_found::NotFound};

// The only place paths are written. Pages and navbar import these consts.
pub const HOME: &str = "/";
pub const FEATURES: &str = "/features";
pub const ABOUT: &str = "/about";

/// Root: meta tags + router.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />
        <Title text="Pixlet - Convert Your Images Instantly" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=NotFound>
                <ParentRoute path=path!("/") view=AppShell>
                    <Route path=path!("") view=Home />
                    <Route path=path!("features") view=Features />
                    <Route path=path!("about") view=About />
                    // wildcard inside the shell so 404 keeps navbar + footer
                    <Route path=path!("*any") view=NotFound />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
