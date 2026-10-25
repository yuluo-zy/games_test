//! Owned translation results and transactional language-pack replacement.
//!
//! Use [`Localization`] for independent services, or configure the process-wide
//! facade before its first load. Returned strings remain valid after switching
//! locales; a failed load leaves the previous snapshot available to readers.
mod diagnostics;
mod global;
pub mod language_library;
mod missing_cache;
mod preference;
mod service;

pub use global::{
    configure, game_locale, lang_pack_path, load_last_used_locale, load_locale, lookup,
};
pub use language_library::LanguageLibrary;
pub use service::Localization;
