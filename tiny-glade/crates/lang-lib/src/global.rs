//! Lazily configured process-wide facade; queries never initialize it.
use crate::{Localization, service::DEFAULT_LOCALE};
use anyhow::{Context, Result};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock},
};

static GLOBAL: OnceLock<Localization> = OnceLock::new();
// Share one lock for explicit/default initialization. Failed initialization must
// remain retryable, while queries stay independent of cwd/environment capture.
static GLOBAL_INITIALIZATION: Mutex<()> = Mutex::new(());
/// Configure the global service before its first load or path query.
pub fn configure(asset_root: impl AsRef<Path>, fallback_enabled: bool) -> Result<()> {
    let _lock = GLOBAL_INITIALIZATION
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    anyhow::ensure!(GLOBAL.get().is_none(), "localization already configured");
    GLOBAL
        .set(Localization::new(asset_root, fallback_enabled)?)
        .map_err(|_| anyhow::anyhow!("localization already configured"))
}
fn global() -> Result<&'static Localization> {
    if let Some(service) = GLOBAL.get() {
        return Ok(service);
    }
    let _lock = GLOBAL_INITIALIZATION
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    if GLOBAL.get().is_none() {
        let service = Localization::new(
            "assets/lang",
            std::env::var_os("TG_DISABLE_LANG_FALLBACK").is_none(),
        )?;
        let _ = GLOBAL.set(service);
    }
    GLOBAL.get().context("localization not configured")
}
pub fn lang_pack_path(locale: &str) -> Result<PathBuf> {
    global()?.lang_pack_path(locale)
}
pub fn load_locale(locale: &str) -> Result<()> {
    let service = global()?;
    match paths::preference_root_path() {
        Ok(root) => service.load_locale_and_save(locale, &root),
        Err(error) => {
            service.load_locale(locale)?;
            crate::diagnostics::warn(|| {
                log::warn!("Unable to locate preference directory: {error:#}");
            });
            Ok(())
        }
    }
}
pub fn load_last_used_locale() -> Result<()> {
    let service = global()?;
    match paths::preference_root_path() {
        Ok(root) => service.load_last_used_locale(&root),
        Err(_) => service
            .load_locale(DEFAULT_LOCALE)
            .context("Loading default locale without preference directory"),
    }
}
/// Before configuration/loading, return an owned copy of the key. Lookup does
/// not implicitly capture the cwd or environment configuration.
pub fn lookup(key: &str) -> Arc<str> {
    GLOBAL
        .get()
        .map_or_else(|| Arc::from(key), |s| s.lookup(key))
}
pub fn game_locale() -> Arc<str> {
    GLOBAL
        .get()
        .map_or_else(|| Arc::from(DEFAULT_LOCALE), Localization::game_locale)
}
