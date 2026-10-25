//! Pure business rules. No leptos, no web_sys. Runs under native `cargo test`.

mod error;

pub mod convert;
pub mod format;
pub mod mass_clicker;
pub mod source;
pub mod stage;

pub use error::CoreError;
