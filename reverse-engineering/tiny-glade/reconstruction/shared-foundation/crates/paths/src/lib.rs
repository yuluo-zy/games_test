//! B Windows path policy, recovered from module 637 and its emitted instructions.
//! The immutable safe storage replaces original leaked path globals; no ABI match is claimed.
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

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
    /// Injection boundary for the operating-system folder provider and current directory.
    /// This does not add an original constructor: it makes the recovered policy testable.
    pub fn initialize_at(steam_id: u64, app_root: PathBuf, installed: PathBuf) -> Result<Self> {
        let config = app_root.join(format!("Steam/{steam_id}"));
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
    /// Original creates this path, but no standalone getter was emitted in B.
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

static PATHS: OnceLock<Paths> = OnceLock::new();
fn paths() -> &'static Paths {
    PATHS.get().expect("paths not initialized")
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

/// Known Saved Games directory, then USERPROFILE/Saved Games, then Tiny Glade.
#[cfg(windows)]
pub fn find_standard_app_root_path() -> Result<PathBuf> {
    choose_standard_app_root(
        known_folders::get_known_folder_path(known_folders::KnownFolder::SavedGames),
        || std::env::var("USERPROFILE"),
    )
}

#[cfg(windows)]
fn choose_standard_app_root(
    saved_games: Option<PathBuf>,
    user_profile: impl FnOnce() -> std::result::Result<String, std::env::VarError>,
) -> Result<PathBuf> {
    let saved_games = saved_games.map(Ok).unwrap_or_else(|| {
        user_profile()
            .context("Reading environment variable USERPROFILE")
            .map(|p| PathBuf::from(p).join("Saved Games"))
    })?;
    Ok(saved_games.join("Tiny Glade"))
}

#[cfg(not(windows))]
pub fn find_standard_app_root_path() -> Result<PathBuf> {
    anyhow::bail!("non-Windows path selection has not been recovered from a matching original")
}

pub fn initialize_paths(steam_id: u64, root_override: Option<&Path>) -> Result<()> {
    let app_root = match root_override {
        Some(root) => root.join("Tiny Glade"),
        None => find_standard_app_root_path()?,
    };
    // Original asks for current_dir after creating the directories.
    let mut paths = Paths::initialize_at(steam_id, app_root, PathBuf::new())?;
    paths.installed = std::env::current_dir().context("current_dir()")?;
    PATHS
        .set(paths)
        .expect("initialize_paths called more than once");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[cfg(windows)]
    fn platform_provider_precedence_and_missing_environment_error() {
        assert_eq!(
            choose_standard_app_root(Some("known-save-games".into()), || panic!(
                "must not read USERPROFILE"
            ))
            .unwrap(),
            PathBuf::from("known-save-games/Tiny Glade")
        );
        assert_eq!(
            choose_standard_app_root(None, || Ok("profile".into())).unwrap(),
            PathBuf::from("profile/Saved Games/Tiny Glade")
        );
        let error =
            choose_standard_app_root(None, || Err(std::env::VarError::NotPresent)).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Reading environment variable USERPROFILE"
        );
        // Real read-only platform call must match its published KnownFolder result.
        if let Some(path) =
            known_folders::get_known_folder_path(known_folders::KnownFolder::SavedGames)
        {
            assert_eq!(
                find_standard_app_root_path().unwrap(),
                path.join("Tiny Glade")
            );
        }
    }
    #[test]
    fn account_paths_and_partial_failure() {
        let root = std::env::temp_dir().join(format!("tg-path-policy-{}", std::process::id()));
        let app = root.join("Tiny Glade");
        let p = Paths::initialize_at(7, app.clone(), PathBuf::from("installation")).unwrap();
        assert_eq!(p.config(), app.join("Steam/7"));
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
        assert!(Paths::initialize_at(8, app, PathBuf::new()).is_err());
        assert!(bad.join("saves").is_dir());
        assert!(!bad.join("tmp").exists());
        // Exact test-owned root, no recursive deletion of computed unverified paths.
        assert!(root.starts_with(std::env::temp_dir()));
        std::fs::remove_dir_all(root).unwrap();
    }
}
