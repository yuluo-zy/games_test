//! B locale loading, hot replacement, and two-stage lookup policy.
pub mod language_library;
use anyhow::{Context, Result};
use arc_swap::ArcSwapOption;
pub use language_library::LanguageLibrary;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::sync::Arc;

const DEFAULT_LOCALE: &str = "en-US";
static LIBRARY: Lazy<ArcSwapOption<LanguageLibrary>> = Lazy::new(ArcSwapOption::empty);
static FALLBACK: Lazy<ArcSwapOption<LanguageLibrary>> = Lazy::new(ArcSwapOption::empty);

pub fn lang_pack_path(locale: &str) -> PathBuf {
    PathBuf::from("assets/lang/").join(format!("{locale}.yml"))
}
fn load_locale_impl(locale: &str) -> Result<()> {
    let library = match LanguageLibrary::try_from_yml(lang_pack_path(locale), locale.to_owned()) {
        Ok(library) => library,
        Err(error) => {
            log::warn!(
                "Couldn't load locale {locale}: {error}. Loading the default instead: {DEFAULT_LOCALE}"
            );
            LanguageLibrary::try_from_yml(
                lang_pack_path(DEFAULT_LOCALE),
                DEFAULT_LOCALE.to_owned(),
            )?
        }
    };
    LIBRARY.store(Some(Arc::new(library)));
    // B calls std::env::_var, so any Unicode value disables fallback (even "0").
    // Missing or non-Unicode values take the error branch and load fallback.
    // The original leaves the preceding fallback alive on later disabled reloads.
    if std::env::var("TG_DISABLE_LANG_FALLBACK").is_err() {
        let fallback = LanguageLibrary::try_from_yml(
            lang_pack_path(DEFAULT_LOCALE),
            DEFAULT_LOCALE.to_owned(),
        )
        .unwrap_or_else(|e| panic!("Failed to load fallback locale {DEFAULT_LOCALE}: {e}"));
        FALLBACK.store(Some(Arc::new(fallback)));
    }
    Ok(())
}
pub fn load_locale(locale: &str) -> Result<()> {
    load_locale_impl(locale)?;
    // Save the requested locale even if its pack had to fall back to English.
    if let Err(e) = std::fs::write(
        paths::standard_app_root_path().join("last_used_locale"),
        locale,
    ) {
        log::warn!("Unable to write last_used_locale: {e}");
    }
    Ok(())
}
pub fn load_last_used_locale() -> Result<()> {
    let locale = paths::find_standard_app_root_path()
        .ok()
        .and_then(|path| std::fs::read_to_string(path.join("last_used_locale")).ok())
        .unwrap_or_else(|| DEFAULT_LOCALE.to_owned());
    load_locale_impl(&locale).context("Loading last used locale")
}
/// Before initialization or without a fallback, the original returns the input key.
pub fn lookup(key: &str) -> &str {
    let library = LIBRARY.load();
    let Some(library) = library.as_ref() else {
        log::warn!("Language library not loaded yet. Unable to lookup {key}");
        return key;
    };
    if let Some(value) = library.try_get(key) {
        return value;
    }
    let fallback = FALLBACK.load();
    match fallback.as_ref() {
        Some(fallback) => fallback.get(key),
        None => {
            log::warn!("Fallback language library not loaded yet. Unable to lookup {key}");
            key
        }
    }
}
pub fn game_locale() -> &'static str {
    let library = LIBRARY.load();
    library.as_ref().map_or_else(
        || {
            log::warn!("Language library not loaded yet. Unable to lookup game language");
            DEFAULT_LOCALE
        },
        |library| library.locale(),
    )
}
