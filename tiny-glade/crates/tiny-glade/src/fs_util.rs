//! `tiny_glade::fs_util` — filesystem helpers.
//!
//! Recovered from B `crates/tiny-glade/src/fs_util.rs`.
//! `copy_folder` is a real business function (recursive directory copy); it is
//! not a derive/registration glue function.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Recursively copy `from` into `to`.
///
/// Reconstructed control flow (B `tiny_glade::fs_util::copy_folder`,
/// VA `0x140355800`):
/// 1. `std::fs::create_dir_all(to)` and propagate errors with context.
/// 2. `std::fs::read_dir(from)` and propagate errors with context.
/// 3. For every entry:
///    - `file_type()` errors are propagated;
///    - when the entry is a directory *and not* a symlink → recurse;
///    - otherwise → `std::fs::copy(entry.path(), to.join(file_name))`.
///
/// The original Windows code tested `FILE_ATTRIBUTE_DIRECTORY (0x10)` and the
/// reparse-point/symlink flags, which is exactly the `is_dir() &&
/// !is_symlink()` shape above (std itself performs the attribute checks).
pub fn copy_folder(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();

    std::fs::create_dir_all(to).with_context(|| format!("Failed to create {to:?}"))?;
    let entries = std::fs::read_dir(from).with_context(|| format!("Failed to read {from:?}"))?;

    for entry in entries {
        let entry = entry.with_context(|| format!("Failed to read directory entry in {from:?}"))?;
        let file_type = entry
            .file_type()
            .with_context(|| format!("Failed to query file type of {entry:?}"))?;
        let destination = to.join(entry.file_name());

        if file_type.is_dir() && !file_type.is_symlink() {
            copy_folder(entry.path(), destination)?;
        } else {
            std::fs::copy(entry.path(), &destination)
                .with_context(|| format!("Failed to copy {:?} to {destination:?}", entry.path()))?;
        }
    }

    Ok(())
}

/// Path-typed shim used by callers that already hold `PathBuf`s, mirroring the
/// original monomorphisations (`<PathBuf, PathBuf>` / `<&PathBuf, PathBuf>`).
pub fn copy_folder_paths(from: PathBuf, to: PathBuf) -> Result<()> {
    copy_folder(from, to)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_nested_tree() {
        let source = tempfile::tempdir().unwrap();
        let destination = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(source.path().join("a/b")).unwrap();
        std::fs::write(source.path().join("a/one.txt"), b"one").unwrap();
        std::fs::write(source.path().join("a/b/two.txt"), b"two").unwrap();

        copy_folder(source.path(), destination.path()).unwrap();

        assert_eq!(
            std::fs::read(destination.path().join("a/one.txt")).unwrap(),
            b"one"
        );
        assert_eq!(
            std::fs::read(destination.path().join("a/b/two.txt")).unwrap(),
            b"two"
        );
    }
}
