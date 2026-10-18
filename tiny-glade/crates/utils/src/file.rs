//! Timestamped backups that preserve source names and never replace an existing backup.
use crate::local_time::system_time_to_local_time;
use std::{
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io,
    path::{Path, PathBuf},
    time::SystemTime,
};

pub fn date_time_string_for_filename_at(now: SystemTime) -> String {
    let now = system_time_to_local_time(now);
    format!(
        "{:04}-{:02}-{:02} {:02}-{:02}-{:02}",
        now.year(),
        u8::from(now.month()),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    )
}

pub fn date_time_string_for_filename() -> String {
    date_time_string_for_filename_at(SystemTime::now())
}

/// Keep the original extension and append `-<timestamp>.json` in the same directory.
///
/// Non-Unicode names are supported. This only formats a path; use
/// [`copy_file_to_backup`] to reserve a unique destination before writing.
///
/// # Panics
/// Panics when `path` has no file name, for example a filesystem root.
pub fn generate_backup_file_path_at(path: &Path, now: SystemTime) -> PathBuf {
    backup_path(path, &date_time_string_for_filename_at(now), 0)
        .expect("backup source must have a file name")
}

pub fn generate_backup_file_path(path: &Path) -> PathBuf {
    generate_backup_file_path_at(path, SystemTime::now())
}

fn backup_path(path: &Path, timestamp: &str, collision: u64) -> io::Result<PathBuf> {
    let mut name = OsString::from(path.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "backup source has no file name",
        )
    })?);
    name.push("-");
    name.push(timestamp);
    if collision != 0 {
        name.push(format!("-{collision}"));
    }
    name.push(".json");
    Ok(path.with_file_name(name))
}

/// Copy a file to a uniquely reserved, timestamped sibling, preserving permissions.
///
/// Calls within the same second use numbered suffixes. `create_new` also prevents
/// overwriting a destination created concurrently or following an existing symlink.
/// A failed write removes its partial destination on a best-effort basis.
/// This copies the source stream; it cannot snapshot concurrent source writes.
pub fn copy_file_to_backup(path: &Path) -> io::Result<()> {
    copy_file_to_backup_at(path, SystemTime::now()).map(|_| ())
}

fn copy_file_to_backup_at(path: &Path, now: SystemTime) -> io::Result<PathBuf> {
    let mut source = File::open(path)?;
    let permissions = source.metadata()?.permissions();
    let timestamp = date_time_string_for_filename_at(now);
    let mut collision = 0u64;
    let (destination, mut output) = loop {
        let destination = backup_path(path, &timestamp, collision)?;
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&destination)
        {
            Ok(output) => break (destination, output),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                collision = collision.checked_add(1).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::AlreadyExists, "backup names exhausted")
                })?;
            }
            Err(error) => return Err(error),
        }
    };
    let result = (|| {
        io::copy(&mut source, &mut output)?;
        output.sync_all()?;
        output.set_permissions(permissions)
    })();
    // Windows requires closing the handle before deleting a failed copy.
    drop(output);
    if let Err(error) = result {
        let _ = fs::remove_file(&destination);
        return Err(error);
    }
    Ok(destination)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "utils-backup-{}-{}",
                std::process::id(),
                NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn backup_names_preserve_extensions_and_support_pre_epoch_dates() {
        assert_eq!(
            generate_backup_file_path_at(Path::new("saves/a.ron"), SystemTime::UNIX_EPOCH),
            PathBuf::from("saves/a.ron-1970-01-01 00-00-00.json")
        );
        assert_eq!(
            date_time_string_for_filename_at(
                SystemTime::UNIX_EPOCH - std::time::Duration::from_secs(1)
            ),
            "1969-12-31 23-59-59"
        );
        assert!(backup_path(Path::new("/"), "timestamp", 0).is_err());
    }

    #[test]
    fn repeated_and_concurrent_backups_never_replace_previous_contents() {
        let fixture = Fixture::new();
        let source = fixture.0.join("save.ron");
        fs::write(&source, b"first version").unwrap();
        let original = copy_file_to_backup_at(&source, SystemTime::UNIX_EPOCH).unwrap();
        fs::write(&source, b"new version").unwrap();
        let copies = std::thread::scope(|scope| {
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    scope.spawn(|| copy_file_to_backup_at(&source, SystemTime::UNIX_EPOCH).unwrap())
                })
                .collect();
            handles
                .into_iter()
                .map(|h| h.join().unwrap())
                .collect::<Vec<_>>()
        });
        assert_eq!(fs::read(&original).unwrap(), b"first version");
        assert_eq!(
            fs::metadata(&original).unwrap().permissions().readonly(),
            fs::metadata(&source).unwrap().permissions().readonly()
        );
        let unique: std::collections::HashSet<_> = copies.iter().collect();
        assert_eq!(unique.len(), 4);
        for copy in copies {
            assert_eq!(fs::read(copy).unwrap(), b"new version");
        }
        assert_eq!(fs::read_dir(&fixture.0).unwrap().count(), 6);
        assert!(copy_file_to_backup(&fixture.0.join("missing")).is_err());
        assert_eq!(fs::read_dir(&fixture.0).unwrap().count(), 6);
        // A directory is not a copyable source. Remove any reserved partial output.
        let failed_destination = generate_backup_file_path_at(&fixture.0, SystemTime::UNIX_EPOCH);
        assert!(copy_file_to_backup_at(&fixture.0, SystemTime::UNIX_EPOCH).is_err());
        assert!(!failed_destination.exists());
    }

    #[test]
    #[cfg(unix)]
    fn backups_do_not_follow_destination_symlinks() {
        use std::os::unix::fs::symlink;
        let fixture = Fixture::new();
        let source = fixture.0.join("source");
        fs::write(&source, b"source").unwrap();
        let victim = fixture.0.join("unrelated");
        fs::write(&victim, b"keep").unwrap();
        let name = generate_backup_file_path_at(&source, SystemTime::UNIX_EPOCH);
        symlink(&victim, &name).unwrap();
        let backup = copy_file_to_backup_at(&source, SystemTime::UNIX_EPOCH).unwrap();
        assert_ne!(backup, name);
        assert_eq!(fs::read(backup).unwrap(), b"source");
        assert_eq!(fs::read(victim).unwrap(), b"keep");
    }
    #[test]
    #[cfg(unix)]
    fn backup_path_keeps_non_unicode_filename_bytes() {
        use std::os::unix::ffi::{OsStrExt, OsStringExt};
        // Check path formatting without requiring filesystem support for invalid UTF-8 names.
        let path = PathBuf::from(OsString::from_vec(vec![b's', 0xff]));
        let backup = generate_backup_file_path_at(&path, SystemTime::UNIX_EPOCH);
        let mut expected = vec![b's', 0xff];
        expected.extend_from_slice(b"-1970-01-01 00-00-00.json");
        assert_eq!(backup.as_os_str().as_bytes(), expected);
    }
}
