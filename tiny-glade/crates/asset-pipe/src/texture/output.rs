//! Publish complete files; an interrupted write must not destroy the previous output.
use anyhow::{Context, Result};
use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_TEMPORARY: AtomicU64 = AtomicU64::new(0);

struct TemporaryOutput(PathBuf);
impl Drop for TemporaryOutput {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

pub(super) fn replace_file(target: &Path, bytes: &[u8]) -> Result<()> {
    let parent = target.parent().context("Output path has no parent")?;
    std::fs::create_dir_all(parent)
        .with_context(|| format!("Creating output directory {}", parent.display()))?;
    let (mut file, temporary) = loop {
        let sequence = NEXT_TEMPORARY.fetch_add(1, Ordering::Relaxed);
        let path = parent.join(format!(
            ".texture-output.{}.{sequence}.tmp",
            std::process::id()
        ));
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => break (file, TemporaryOutput(path)),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error).context("Creating temporary texture output"),
        }
    };
    let result = file.write_all(bytes);
    // Close before rename or cleanup so the same path works on Windows.
    drop(file);
    result.with_context(|| format!("Writing {}", target.display()))?;
    std::fs::rename(&temporary.0, target)
        .with_context(|| format!("Publishing {}", target.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replacement_preserves_old_output_on_failure_and_cleans_temporary_files() {
        let root = std::env::temp_dir().join(format!("tg-output-test-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let target = root.join("a.texture");
        replace_file(&target, b"old").unwrap();
        replace_file(&target, b"new").unwrap();
        assert_eq!(std::fs::read(&target).unwrap(), b"new");
        let blocked = root.join("directory.texture");
        std::fs::create_dir(&blocked).unwrap();
        std::fs::write(blocked.join("keep"), b"old").unwrap();
        assert!(replace_file(&blocked, b"replacement").is_err());
        assert_eq!(std::fs::read(blocked.join("keep")).unwrap(), b"old");
        assert_eq!(std::fs::read_dir(&root).unwrap().count(), 2);
        std::fs::remove_dir_all(root).unwrap();
    }
}
