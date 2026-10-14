//! Stable application directories and serialized, fallible global initialization.
//! Convenience getters require successful initialization; use `try_paths` otherwise.
use anyhow::{Context, Result};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone)]
pub struct Paths {
    config: PathBuf,
    saves: PathBuf,
    beta_saves: PathBuf,
    backup_saves: PathBuf,
    tmp: PathBuf,
    deleted_saves: PathBuf,
    installed: PathBuf,
    standard_app_root: PathBuf,
}

impl Paths {
    /// Build an independent set of account directories from explicit locations.
    /// Both paths must be nonempty; relative paths are fixed against the current cwd.
    pub fn initialize_at(steam_id: u64, app_root: PathBuf, installed: PathBuf) -> Result<Self> {
        // Resolve all inputs before making filesystem changes.
        let app_root = std::path::absolute(app_root).context("Resolving application root")?;
        let installed =
            std::path::absolute(installed).context("Resolving installation directory")?;
        let config = app_root.join("Steam").join(steam_id.to_string());
        // Creation order is observable: failures leave earlier directories on disk.
        let saves = ensure_path(config.join("saves"))?;
        let beta_saves = ensure_path(config.join("beta-saves"))?;
        let tmp = ensure_path(config.join("tmp"))?;
        let deleted_saves = ensure_path(config.join("deleted-saves"))?;
        let backup_saves = ensure_path(config.join("backup-saves"))?;
        let config = ensure_path(config)?;
        Ok(Self {
            config,
            saves,
            beta_saves,
            backup_saves,
            tmp,
            deleted_saves,
            installed,
            standard_app_root: app_root,
        })
    }
    pub fn config(&self) -> &Path {
        &self.config
    }
    pub fn saves(&self) -> &Path {
        &self.saves
    }
    /// Save directory reserved for beta-version data.
    pub fn beta_saves(&self) -> &Path {
        &self.beta_saves
    }
    pub fn backup_saves(&self) -> &Path {
        &self.backup_saves
    }
    pub fn tmp(&self) -> &Path {
        &self.tmp
    }
    pub fn deleted_saves(&self) -> &Path {
        &self.deleted_saves
    }
    pub fn installed(&self) -> &Path {
        &self.installed
    }
    pub fn standard_app_root_path(&self) -> &Path {
        &self.standard_app_root
    }
}

#[derive(Default)]
struct PathStorage {
    value: OnceLock<Paths>,
    initialization: Mutex<()>,
}
impl PathStorage {
    fn initialize(&self, build: impl FnOnce() -> Result<Paths>) -> Result<()> {
        let _lock = self
            .initialization
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        anyhow::ensure!(self.value.get().is_none(), "paths already initialized");
        let paths = build()?;
        self.value
            .set(paths)
            .map_err(|_| anyhow::anyhow!("paths already initialized"))
    }
}
static PATHS: PathStorage = PathStorage {
    value: OnceLock::new(),
    initialization: Mutex::new(()),
};
pub fn try_paths() -> Result<&'static Paths> {
    PATHS.value.get().context("paths not initialized")
}
fn paths() -> &'static Paths {
    try_paths().expect("paths not initialized")
}
/// Use the configured override when initialized, otherwise the platform default.
pub fn preference_root_path() -> Result<PathBuf> {
    match PATHS.value.get() {
        Some(paths) => Ok(paths.standard_app_root_path().to_owned()),
        None => find_standard_app_root_path(),
    }
}
pub fn config() -> &'static Path {
    paths().config()
}
pub fn saves() -> &'static Path {
    paths().saves()
}
pub fn backup_saves() -> &'static Path {
    paths().backup_saves()
}
pub fn tmp() -> &'static Path {
    paths().tmp()
}
pub fn deleted_saves() -> &'static Path {
    paths().deleted_saves()
}
pub fn installed() -> &'static Path {
    paths().installed()
}
pub fn standard_app_root_path() -> &'static Path {
    paths().standard_app_root_path()
}

pub fn ensure_path(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();
    std::fs::create_dir_all(path)
        .with_context(|| format!("Creating directory {}", path.display()))?;
    Ok(path.to_path_buf())
}

/// Platform default: Saved Games on Windows, Application Support on macOS,
/// XDG data home (or ~/.local/share) on Linux. Does not create directories.
pub fn find_standard_app_root_path() -> Result<PathBuf> {
    #[cfg(windows)]
    let root = choose_windows_root(
        known_folders::get_known_folder_path(known_folders::KnownFolder::SavedGames),
        || std::env::var_os("USERPROFILE"),
    )?;
    #[cfg(target_os = "macos")]
    let root = choose_unix_root(std::env::var_os("HOME"), None, true)?;
    #[cfg(target_os = "linux")]
    let root = choose_unix_root(
        std::env::var_os("HOME"),
        std::env::var_os("XDG_DATA_HOME"),
        false,
    )?;
    #[cfg(not(any(windows, target_os = "macos", target_os = "linux")))]
    anyhow::bail!("default application directories are unsupported on this platform");
    #[cfg(any(windows, target_os = "macos", target_os = "linux"))]
    Ok(root)
}
#[cfg(any(windows, test))]
fn choose_windows_root(
    saved_games: Option<PathBuf>,
    profile: impl FnOnce() -> Option<OsString>,
) -> Result<PathBuf> {
    let root = match saved_games {
        Some(root) => root,
        None => absolute_home(profile(), "USERPROFILE")?.join("Saved Games"),
    };
    anyhow::ensure!(root.is_absolute(), "Saved Games directory must be absolute");
    Ok(root.join("Tiny Glade"))
}
#[cfg(any(target_os = "macos", target_os = "linux", test))]
fn choose_unix_root(home: Option<OsString>, xdg: Option<OsString>, macos: bool) -> Result<PathBuf> {
    if !macos && let Some(path) = xdg.map(PathBuf::from).filter(|p| p.is_absolute()) {
        return Ok(path.join("Tiny Glade"));
    }
    let home = absolute_home(home, "HOME")?;
    Ok(if macos {
        home.join("Library/Application Support")
    } else {
        home.join(".local/share")
    }
    .join("Tiny Glade"))
}
fn absolute_home(home: Option<OsString>, name: &str) -> Result<PathBuf> {
    let home = PathBuf::from(home.with_context(|| format!("Reading environment variable {name}"))?);
    anyhow::ensure!(home.is_absolute(), "{name} must be an absolute directory");
    Ok(home)
}

/// `root_override` is a parent directory; `Tiny Glade` is appended.
/// Repeated calls return an error before producing filesystem side effects.
pub fn initialize_paths(steam_id: u64, root_override: Option<&Path>) -> Result<()> {
    PATHS.initialize(|| {
        let app_root = match root_override {
            Some(root) => std::path::absolute(root)?.join("Tiny Glade"),
            None => find_standard_app_root_path()?,
        };
        let executable = std::env::current_exe().context("Finding executable location")?;
        let installed = executable
            .parent()
            .context("Executable has no parent directory")?
            .to_owned();
        Paths::initialize_at(steam_id, app_root, installed)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    fn root() -> PathBuf {
        std::env::temp_dir().join(format!(
            "tg-path-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ))
    }
    #[test]
    fn platform_policies_and_missing_environment() {
        let home = std::env::temp_dir().join("test-home");
        assert_eq!(
            choose_windows_root(Some(home.clone()), || panic!("must not query profile")).unwrap(),
            home.join("Tiny Glade")
        );
        assert_eq!(
            choose_windows_root(None, || Some(home.clone().into_os_string())).unwrap(),
            home.join("Saved Games/Tiny Glade")
        );
        assert!(choose_windows_root(None, || None).is_err());
        assert_eq!(
            choose_unix_root(Some(home.clone().into_os_string()), None, true).unwrap(),
            home.join("Library/Application Support/Tiny Glade")
        );
        assert_eq!(
            choose_unix_root(
                Some(home.clone().into_os_string()),
                Some("relative".into()),
                false
            )
            .unwrap(),
            home.join(".local/share/Tiny Glade")
        );
        assert_eq!(
            choose_unix_root(None, Some(home.clone().into_os_string()), false).unwrap(),
            home.join("Tiny Glade")
        );
        assert!(choose_unix_root(None, None, false).is_err());
        assert!(choose_unix_root(Some("".into()), None, true).is_err());
    }
    #[test]
    #[cfg(unix)]
    fn non_unicode_home_is_supported() {
        use std::os::unix::ffi::OsStringExt;
        let home = std::env::temp_dir().join(OsString::from_vec(vec![b'h', 255]));
        assert_eq!(
            choose_unix_root(Some(home.clone().into_os_string()), None, false).unwrap(),
            home.join(".local/share/Tiny Glade")
        );
    }
    #[test]
    #[cfg(windows)]
    fn non_unicode_windows_profile_is_supported() {
        use std::os::windows::ffi::OsStringExt;
        let home = std::env::temp_dir().join(OsString::from_wide(&[b'h' as u16, 0xd800]));
        assert_eq!(
            choose_windows_root(None, || Some(home.clone().into_os_string())).unwrap(),
            home.join("Saved Games/Tiny Glade")
        );
    }
    #[test]
    fn invalid_injected_path_fails_before_creating_directories() {
        let root = root();
        assert!(Paths::initialize_at(1, root.clone(), PathBuf::new()).is_err());
        assert!(!root.exists());
    }
    #[test]
    fn account_paths_and_partial_failure() {
        let root = root();
        let app = root.join("Tiny Glade");
        let p = Paths::initialize_at(7, app.clone(), PathBuf::from("installation")).unwrap();
        assert_eq!(p.config(), app.join("Steam/7"));
        assert!(p.installed().is_absolute());
        for name in [
            "saves",
            "beta-saves",
            "tmp",
            "deleted-saves",
            "backup-saves",
        ] {
            assert!(p.config().join(name).is_dir());
        }
        let bad = app.join("Steam/8");
        std::fs::create_dir_all(&bad).unwrap();
        std::fs::write(bad.join("beta-saves"), b"file blocks folder").unwrap();
        assert!(Paths::initialize_at(8, app, root.clone()).is_err());
        assert!(bad.join("saves").is_dir());
        assert!(!bad.join("tmp").exists());
        std::fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn failed_initialization_can_retry_and_reinitialization_has_no_side_effects() {
        let storage = PathStorage::default();
        assert!(
            storage
                .initialize(|| anyhow::bail!("injected failure"))
                .is_err()
        );
        let root = root();
        storage
            .initialize(|| Paths::initialize_at(1, root.clone(), root.clone()))
            .unwrap();
        assert!(
            storage
                .initialize(|| panic!("second builder must not run"))
                .is_err()
        );
        std::fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn concurrent_initialization_runs_one_builder() {
        let storage = std::sync::Arc::new(PathStorage::default());
        let root = root();
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(8));
        let joins: Vec<_> = (0..8)
            .map(|id| {
                let storage = storage.clone();
                let root = root.clone();
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    barrier.wait();
                    storage
                        .initialize(|| {
                            Paths::initialize_at(id, root.join(id.to_string()), root.clone())
                        })
                        .is_ok()
                })
            })
            .collect();
        assert_eq!(
            joins
                .into_iter()
                .map(|join| join.join().unwrap())
                .filter(|ok| *ok)
                .count(),
            1
        );
        assert_eq!(std::fs::read_dir(&root).unwrap().count(), 1);
        std::fs::remove_dir_all(root).unwrap();
    }
}
