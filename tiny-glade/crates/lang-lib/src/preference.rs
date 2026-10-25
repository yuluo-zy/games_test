//! Best-effort preferences use same-directory temporary files and atomic replacement.
use crate::language_library::validate_locale;
use anyhow::{Context, Result};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

const FILE_NAME: &str = "last_used_locale";

/// Missing, unreadable and invalid preferences all select the default locale.
pub(crate) fn read(root: &Path) -> Option<String> {
    let stored = std::fs::read_to_string(root.join(FILE_NAME)).ok()?;
    let locale = stored.trim();
    validate_locale(locale).ok()?;
    Some(locale.to_owned())
}

static TEMP_ID: AtomicU64 = AtomicU64::new(0);
struct TemporaryPreference(PathBuf);
impl Drop for TemporaryPreference {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
pub(crate) fn save(root: &Path, locale: &str) -> Result<()> {
    std::fs::create_dir_all(root).context("Creating preference directory")?;
    replace_preference(
        &root.join(FILE_NAME),
        |file| {
            file.write_all(locale.as_bytes())?;
            file.sync_all()
        },
        |source, target| std::fs::rename(source, target),
    )
}
pub(crate) fn replace_preference(
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
