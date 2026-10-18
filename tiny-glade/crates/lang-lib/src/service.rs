//! Independent services publish complete snapshots after serialized disk loads.
use crate::{
    LanguageLibrary, language_library::validate_locale, missing_cache::MissingCache, preference,
};
use anyhow::{Context, Result};
use arc_swap::ArcSwapOption;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

pub(crate) const DEFAULT_LOCALE: &str = "en-US";
struct LocaleSnapshot {
    primary: Arc<LanguageLibrary>,
    fallback: Option<Arc<LanguageLibrary>>,
    missing: MissingCache,
}
impl LocaleSnapshot {
    fn lookup(&self, key: &str) -> Arc<str> {
        if let Some(value) = self.primary.try_get(key) {
            return value;
        }
        match &self.fallback {
            Some(fallback) => fallback
                .try_get(key)
                .unwrap_or_else(|| self.missing.get(key)),
            None => Arc::from(key),
        }
    }
}

/// An independent localization service. The asset directory is resolved once,
/// and fallback policy is explicit and fixed for this service's lifetime.
pub struct Localization {
    asset_root: PathBuf,
    fallback_enabled: bool,
    snapshot: ArcSwapOption<LocaleSnapshot>,
    // Serialize publication and preference writes so concurrent loads cannot save
    // a preference for a snapshot that has already been replaced.
    load_lock: Mutex<()>,
}
impl Localization {
    /// Resolve the asset directory once without reading any language packs.
    /// `fallback_enabled` controls missing keys; a failed pack load always tries
    /// `en-US`, even when key fallback is disabled.
    pub fn new(asset_root: impl AsRef<Path>, fallback_enabled: bool) -> Result<Self> {
        let asset_root = asset_root.as_ref();
        let asset_root = std::path::absolute(asset_root)
            .with_context(|| format!("Resolving language directory {}", asset_root.display()))?;
        Ok(Self {
            asset_root,
            fallback_enabled,
            snapshot: ArcSwapOption::empty(),
            load_lock: Mutex::new(()),
        })
    }
    /// Return the pack path after rejecting identifiers that could escape the root.
    pub fn lang_pack_path(&self, locale: &str) -> Result<PathBuf> {
        validate_locale(locale)?;
        Ok(self.asset_root.join(format!("{locale}.yml")))
    }
    fn read_pack(&self, locale: &str) -> Result<LanguageLibrary> {
        LanguageLibrary::try_from_yml(self.lang_pack_path(locale)?, locale.to_owned())
    }

    fn prepare_snapshot(&self, locale: &str) -> Result<LocaleSnapshot> {
        validate_locale(locale)?;
        let primary = match self.read_pack(locale) {
            Ok(library) => library,
            Err(request_error) if locale != DEFAULT_LOCALE => self
                .read_pack(DEFAULT_LOCALE)
                .with_context(|| {
                    format!(
                        "Unable to load requested locale {locale} ({request_error:#}) or default locale"
                    )
                })?,
            Err(error) => return Err(error),
        };
        let primary = Arc::new(primary);
        let fallback = if !self.fallback_enabled {
            None
        } else if &*primary.locale() == DEFAULT_LOCALE {
            Some(Arc::clone(&primary))
        } else {
            Some(Arc::new(
                self.read_pack(DEFAULT_LOCALE)
                    .context("Loading fallback locale")?,
            ))
        };
        Ok(LocaleSnapshot {
            primary,
            fallback,
            missing: MissingCache::default(),
        })
    }
    /// Load without persisting a preference. On failure the old snapshot survives.
    pub fn load_locale(&self, locale: &str) -> Result<()> {
        self.load(locale, None)
    }
    /// Preference persistence is best effort. Successful replacement remains
    /// committed on a filesystem error; the diagnostic is emitted after unlocking.
    pub fn load_locale_and_save(&self, locale: &str, root: &Path) -> Result<()> {
        self.load(locale, Some(root))
    }
    fn load(&self, locale: &str, root: Option<&Path>) -> Result<()> {
        let (actual_locale, save_error) = {
            let _lock = self.load_lock.lock().unwrap_or_else(|e| e.into_inner());
            let snapshot = self.prepare_snapshot(locale)?;
            let actual_locale = snapshot.primary.locale();
            self.snapshot.store(Some(Arc::new(snapshot)));
            let save_error = root.and_then(|root| preference::save(root, locale).err());
            (actual_locale, save_error)
        };
        if &*actual_locale != locale {
            crate::diagnostics::warn(|| {
                log::warn!("Couldn't load locale {locale}; loaded {actual_locale} instead");
            });
        }
        if let Some(error) = save_error {
            crate::diagnostics::warn(|| log::warn!("Unable to write last_used_locale: {error:#}"));
        }
        Ok(())
    }
    /// Restore a valid stored preference, or use the default for an unreadable file.
    /// Required pack failures preserve the current snapshot.
    pub fn load_last_used_locale(&self, root: &Path) -> Result<()> {
        let _lock = self.load_lock.lock().unwrap_or_else(|e| e.into_inner());
        let locale = preference::read(root).unwrap_or_else(|| DEFAULT_LOCALE.to_owned());
        let snapshot = self
            .prepare_snapshot(&locale)
            .context("Loading last used locale")?;
        self.snapshot.store(Some(Arc::new(snapshot)));
        Ok(())
    }
    /// Query one complete snapshot without waiting for pack loading or saving.
    pub fn lookup(&self, key: &str) -> Arc<str> {
        let snapshot = self.snapshot.load();
        let Some(snapshot) = snapshot.as_ref() else {
            return Arc::from(key);
        };
        snapshot.lookup(key)
    }

    /// The loaded locale, which may differ from a failed requested locale.
    pub fn game_locale(&self) -> Arc<str> {
        self.snapshot
            .load()
            .as_ref()
            .map_or_else(|| Arc::from(DEFAULT_LOCALE), |s| s.primary.locale())
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
