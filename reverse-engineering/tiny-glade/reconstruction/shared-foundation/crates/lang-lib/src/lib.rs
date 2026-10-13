//! Owned translation results and transactional language-pack replacement.
pub mod language_library;
use anyhow::{Context, Result};
use arc_swap::ArcSwapOption;
pub use language_library::LanguageLibrary;
use language_library::{MissingCache, validate_locale};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex, OnceLock,
        atomic::{AtomicU64, Ordering},
    },
};

const DEFAULT_LOCALE: &str = "en-US";
struct LocaleSnapshot {
    primary: Arc<LanguageLibrary>,
    fallback: Option<Arc<LanguageLibrary>>,
    missing: MissingCache,
}
/// An independent localization service. The asset directory is resolved once,
/// and fallback policy is explicit and fixed for this service's lifetime.
pub struct Localization {
    asset_root: PathBuf,
    fallback_enabled: bool,
    snapshot: ArcSwapOption<LocaleSnapshot>,
    loading: Mutex<()>,
}
impl Localization {
    pub fn new(asset_root: impl AsRef<Path>, fallback_enabled: bool) -> Result<Self> {
        Ok(Self {
            asset_root: std::path::absolute(asset_root)?,
            fallback_enabled,
            snapshot: ArcSwapOption::empty(),
            loading: Mutex::new(()),
        })
    }
    pub fn lang_pack_path(&self, locale: &str) -> Result<PathBuf> {
        validate_locale(locale)?;
        Ok(self.asset_root.join(format!("{locale}.yml")))
    }
    fn prepare(&self, locale: &str) -> Result<LocaleSnapshot> {
        validate_locale(locale)?;
        let primary = match LanguageLibrary::try_from_yml(
            self.lang_pack_path(locale)?,
            locale.to_owned(),
        ) {
            Ok(library) => library,
            Err(request_error) if locale != DEFAULT_LOCALE => LanguageLibrary::try_from_yml(
                self.lang_pack_path(DEFAULT_LOCALE)?,
                DEFAULT_LOCALE.to_owned(),
            )
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
                LanguageLibrary::try_from_yml(
                    self.lang_pack_path(DEFAULT_LOCALE)?,
                    DEFAULT_LOCALE.to_owned(),
                )
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
        let (actual, save_error) = {
            let _lock = self.loading.lock().unwrap_or_else(|e| e.into_inner());
            let snapshot = self.prepare(locale)?;
            let actual = snapshot.primary.locale();
            self.snapshot.store(Some(Arc::new(snapshot)));
            let save_error = root.and_then(|root| save_preference(root, locale).err());
            (actual, save_error)
        };
        if &*actual != locale {
            log::warn!("Couldn't load locale {locale}; loaded {actual} instead");
        }
        if let Some(error) = save_error {
            log::warn!("Unable to write last_used_locale: {error:#}");
        }
        Ok(())
    }
    pub fn load_last_used_locale(&self, root: &Path) -> Result<()> {
        let _lock = self.loading.lock().unwrap_or_else(|e| e.into_inner());
        let stored = std::fs::read_to_string(root.join("last_used_locale")).unwrap_or_default();
        let requested = stored.trim();
        let locale = if validate_locale(requested).is_ok() {
            requested
        } else {
            DEFAULT_LOCALE
        };
        let snapshot = self.prepare(locale).context("Loading last used locale")?;
        self.snapshot.store(Some(Arc::new(snapshot)));
        Ok(())
    }
    pub fn lookup(&self, key: &str) -> Arc<str> {
        let snapshot = self.snapshot.load();
        let Some(snapshot) = snapshot.as_ref() else {
            return Arc::from(key);
        };
        if let Some(value) = snapshot.primary.try_get(key) {
            return value;
        }
        match &snapshot.fallback {
            Some(fallback) => fallback
                .try_get(key)
                .unwrap_or_else(|| snapshot.missing.get(key)),
            None => Arc::from(key),
        }
    }
    pub fn game_locale(&self) -> Arc<str> {
        self.snapshot
            .load()
            .as_ref()
            .map_or_else(|| Arc::from(DEFAULT_LOCALE), |s| s.primary.locale())
    }
}

static GLOBAL: OnceLock<Localization> = OnceLock::new();
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
            log::warn!("Unable to locate preference directory: {error:#}");
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

static TEMP_ID: AtomicU64 = AtomicU64::new(0);
struct TemporaryPreference(PathBuf);
impl Drop for TemporaryPreference {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
fn save_preference(root: &Path, locale: &str) -> Result<()> {
    std::fs::create_dir_all(root).context("Creating preference directory")?;
    replace_preference(
        &root.join("last_used_locale"),
        |file| {
            file.write_all(locale.as_bytes())?;
            file.sync_all()
        },
        |source, target| std::fs::rename(source, target),
    )
}
fn replace_preference(
    target: &Path,
    write: impl FnOnce(&mut File) -> std::io::Result<()>,
    replace: impl FnOnce(&Path, &Path) -> std::io::Result<()>,
) -> Result<()> {
    let parent = target.parent().context("Preference path has no parent")?;
    let (mut file, temporary) = loop {
        let id = TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path = parent.join(format!(".last_used_locale.{}.{id}.tmp", std::process::id()));
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => break (file, TemporaryPreference(path)),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error).context("Creating temporary preference file"),
        }
    };
    let result = write(&mut file);
    // Close on all paths before cleanup/rename (required on Windows).
    drop(file);
    result.context("Writing preference file")?;
    replace(&temporary.0, target).context("Replacing preference file")?;
    Ok(())
}

#[cfg(test)]
mod tests;
