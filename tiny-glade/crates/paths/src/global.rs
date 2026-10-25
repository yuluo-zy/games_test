//! Serialized publication of a single account directory set.
use crate::{Paths, find_standard_app_root_path};
use anyhow::{Context, Result};
use std::{
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};

#[derive(Default)]
pub(crate) struct GlobalPaths {
    value: OnceLock<Paths>,
    initialization: Mutex<()>,
}
impl GlobalPaths {
    pub(crate) fn initialize(&self, build: impl FnOnce() -> Result<Paths>) -> Result<()> {
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
static PATHS: GlobalPaths = GlobalPaths {
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

/// Initialize account directories while keeping assets at an explicit installation.
/// This is the source-build host adaptation: the Cargo executable and original
/// installed assets can live in different directories. Publication has exactly the
/// same single-initialization and failure semantics as `initialize_paths`.
pub fn initialize_paths_with_installation(
    steam_id: u64,
    root_override: Option<&Path>,
    installed: &Path,
) -> Result<()> {
    PATHS.initialize(|| {
        let app_root = match root_override {
            Some(root) => std::path::absolute(root)?.join("Tiny Glade"),
            None => find_standard_app_root_path()?,
        };
        Paths::initialize_at(steam_id, app_root, installed.to_owned())
    })
}
