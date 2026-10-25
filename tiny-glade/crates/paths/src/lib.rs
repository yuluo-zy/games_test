//! Stable account directories with fallible, serialized global initialization.
//! Use [`Paths`] for independent accounts or [`try_paths`] before global initialization.
mod global;
mod platform;

pub use global::{
    backup_saves, config, deleted_saves, initialize_paths, initialize_paths_with_installation, installed, preference_root_path, saves,
    standard_app_root_path, tmp, try_paths,
};
pub use platform::find_standard_app_root_path;

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

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

pub fn ensure_path(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();
    std::fs::create_dir_all(path)
        .with_context(|| format!("Creating directory {}", path.display()))?;
    Ok(path.to_path_buf())
}

#[cfg(test)]
mod tests;
