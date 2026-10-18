//! Platform directory selection without filesystem mutation.
use anyhow::{Context, Result};
use std::{ffi::OsString, path::PathBuf};

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
pub(crate) fn choose_windows_root(
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
pub(crate) fn choose_unix_root(
    home: Option<OsString>,
    xdg: Option<OsString>,
    macos: bool,
) -> Result<PathBuf> {
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
