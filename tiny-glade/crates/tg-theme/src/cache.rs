use crate::model::ContentId;
use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{Read, Write},
    path::{Component, Path, PathBuf},
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, SystemTime},
};

const MANIFEST: &str = ".tg-manifest.json";
static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug)]
pub struct CacheLimits {
    pub max_archive_bytes: u64,
    pub max_files: usize,
    pub max_file_bytes: u64,
    pub max_unpacked_bytes: u64,
}
impl Default for CacheLimits {
    fn default() -> Self {
        Self {
            max_archive_bytes: 64 * 1024 * 1024,
            max_files: 4096,
            max_file_bytes: 64 * 1024 * 1024,
            max_unpacked_bytes: 256 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Default)]
pub struct TrimReport {
    pub removed: Vec<PathBuf>,
    pub errors: Vec<(PathBuf, String)>,
}

pub struct Cache {
    root: PathBuf,
    limits: CacheLimits,
    operations: Mutex<()>,
    _ownership: fs::File,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    version: u32,
    cid: String,
    files: BTreeMap<String, FileInfo>,
    directories: BTreeSet<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileInfo {
    size: u64,
    hash: String,
}

struct Temporary(PathBuf);
impl Drop for Temporary {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

impl Cache {
    pub fn new(root: PathBuf, limits: CacheLimits) -> Result<Self> {
        if let Ok(metadata) = fs::symlink_metadata(&root) {
            ensure!(
                !metadata.file_type().is_symlink(),
                "theme cache root cannot be a symlink"
            );
        }
        ensure!(
            limits.max_archive_bytes > 0
                && limits.max_files > 0
                && limits.max_file_bytes > 0
                && limits.max_unpacked_bytes > 0,
            "theme cache limits must be positive"
        );
        ensure!(
            limits.max_file_bytes <= limits.max_unpacked_bytes,
            "theme file limit exceeds total limit"
        );
        fs::create_dir_all(&root).context("creating theme cache")?;
        let root = fs::canonicalize(root)?;
        let lock_path = root.join(".owner-lock");
        if let Ok(metadata) = fs::symlink_metadata(&lock_path) {
            ensure!(
                metadata.is_file() && !metadata.file_type().is_symlink(),
                "invalid theme cache lock file"
            );
        }
        let ownership = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(lock_path)?;
        ownership
            .try_lock()
            .context("theme cache is already owned")?;
        Ok(Self {
            root,
            limits,
            operations: Mutex::new(()),
            _ownership: ownership,
        })
    }
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Return only a fully verified package, discarding corrupt or obsolete entries.
    pub fn lookup(&self, cid: &ContentId) -> Result<Option<PathBuf>> {
        let _guard = self
            .operations
            .lock()
            .map_err(|_| anyhow::anyhow!("theme cache lock poisoned"))?;
        self.lookup_inner(cid)
    }
    fn lookup_inner(&self, cid: &ContentId) -> Result<Option<PathBuf>> {
        let path = self.root.join(cid.as_str());
        match fs::symlink_metadata(&path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(e.into()),
            Ok(meta) if !meta.is_dir() || meta.file_type().is_symlink() => {
                fs::remove_file(&path)?;
                return Ok(None);
            }
            Ok(_) => {}
        }
        if self.verify(&path, cid).is_err() {
            fs::remove_dir_all(&path).context("discarding invalid theme cache entry")?;
            return Ok(None);
        }
        Ok(Some(path))
    }

    pub fn install(&self, cid: &ContentId, mut reader: impl Read) -> Result<PathBuf> {
        let _guard = self
            .operations
            .lock()
            .map_err(|_| anyhow::anyhow!("theme cache lock poisoned"))?;
        // A corrupt published entry must be replaced instead of poisoning retries.
        if let Some(path) = self.lookup_inner(cid)? {
            return Ok(path);
        }
        let temp = self.temporary()?;
        let archive_path = temp.0.join("download.zip");
        let mut output = fs::File::create(&archive_path)?;
        let mut hasher = blake3::Hasher::new();
        let mut copied = 0u64;
        let mut buffer = [0u8; 32 * 1024];
        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            copied = copied
                .checked_add(count as u64)
                .context("archive size overflow")?;
            ensure!(
                copied <= self.limits.max_archive_bytes,
                "theme archive exceeds size limit"
            );
            hasher.update(&buffer[..count]);
            output.write_all(&buffer[..count])?;
        }
        output.sync_all()?;
        drop(output);
        ensure!(
            hasher.finalize().to_hex().as_str() == cid.as_str(),
            "theme archive checksum mismatch"
        );
        let payload = temp.0.join("payload");
        fs::create_dir(&payload)?;
        self.extract(&archive_path, &payload, cid)?;
        self.verify(&payload, cid)?;
        let destination = self.root.join(cid.as_str());
        fs::rename(&payload, &destination).context("publishing theme cache entry")?;
        Ok(destination)
    }

    fn temporary(&self) -> Result<Temporary> {
        for _ in 0..100 {
            let path = self.root.join(format!(
                ".tmp-{}-{}",
                std::process::id(),
                NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Ok(Temporary(path)),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(e.into()),
            }
        }
        bail!("unable to allocate theme staging directory")
    }

    fn extract(&self, archive: &Path, folder: &Path, cid: &ContentId) -> Result<()> {
        let mut zip = zip::ZipArchive::new(fs::File::open(archive)?)?;
        ensure!(
            zip.len() <= self.limits.max_files,
            "too many theme archive entries"
        );
        let mut manifest = Manifest {
            version: 1,
            cid: cid.as_str().into(),
            files: BTreeMap::new(),
            directories: BTreeSet::new(),
        };
        let mut names = BTreeSet::new();
        let mut aliases = BTreeMap::new();
        let mut total = 0u64;
        for index in 0..zip.len() {
            let mut entry = zip.by_index(index)?;
            let name = entry.name().trim_end_matches('/').to_owned();
            safe_name(&name)?;
            register_spelling(&mut aliases, &name)?;
            ensure!(
                names.insert(name.to_lowercase()),
                "duplicate theme archive path"
            );
            if let Some(mode) = entry.unix_mode() {
                let kind = mode & 0o170000;
                ensure!(
                    kind == 0 || kind == 0o100000 || kind == 0o040000,
                    "unsupported theme archive file type"
                );
            }
            let target = folder.join(&name);
            if entry.is_dir() {
                ensure!(entry.size() == 0, "theme directory entry contains data");
                let mut byte = [0u8; 1];
                ensure!(
                    entry.read(&mut byte)? == 0,
                    "theme directory entry contains data"
                );
                fs::create_dir_all(&target)?;
                add_directories(&mut manifest.directories, &name, true);
                ensure!(
                    manifest
                        .files
                        .len()
                        .saturating_add(manifest.directories.len())
                        <= self.limits.max_files,
                    "too many extracted theme paths"
                );
                continue;
            }
            ensure!(
                entry.size() <= self.limits.max_file_bytes,
                "theme file exceeds size limit"
            );
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&target)?;
            let mut hasher = blake3::Hasher::new();
            let mut size = 0u64;
            let mut buffer = [0u8; 32 * 1024];
            loop {
                let count = entry.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                size = size
                    .checked_add(count as u64)
                    .context("theme file size overflow")?;
                total = total
                    .checked_add(count as u64)
                    .context("theme total size overflow")?;
                ensure!(
                    size <= self.limits.max_file_bytes && total <= self.limits.max_unpacked_bytes,
                    "theme unpacked size limit exceeded"
                );
                hasher.update(&buffer[..count]);
                file.write_all(&buffer[..count])?;
            }
            ensure!(size == entry.size(), "theme ZIP uncompressed size mismatch");
            file.sync_all()?;
            add_directories(&mut manifest.directories, &name, false);
            manifest.files.insert(
                name,
                FileInfo {
                    size,
                    hash: hasher.finalize().to_string(),
                },
            );
            ensure!(
                manifest
                    .files
                    .len()
                    .saturating_add(manifest.directories.len())
                    <= self.limits.max_files,
                "too many extracted theme paths"
            );
        }
        ensure!(
            manifest
                .files
                .len()
                .saturating_add(manifest.directories.len())
                <= self.limits.max_files,
            "too many extracted theme paths"
        );
        let mut file = fs::File::create(folder.join(MANIFEST))?;
        serde_json::to_writer(&mut file, &manifest)?;
        file.sync_all()?;
        Ok(())
    }

    fn verify(&self, folder: &Path, cid: &ContentId) -> Result<()> {
        let manifest_path = folder.join(MANIFEST);
        let meta = fs::symlink_metadata(&manifest_path)?;
        ensure!(
            meta.is_file() && !meta.file_type().is_symlink(),
            "invalid theme manifest"
        );
        // Paths are at most 4096 bytes and exclude JSON escaping characters.
        // Tie manifest memory to the filesystem entry budget, not ZIP size.
        let manifest_limit = (self.limits.max_files as u64)
            .saturating_mul(4224)
            .saturating_add(1024);
        ensure!(
            meta.len() <= manifest_limit,
            "theme manifest exceeds size limit"
        );
        let manifest: Manifest = serde_json::from_reader(
            fs::File::open(manifest_path)?.take(manifest_limit.saturating_add(1)),
        )?;
        ensure!(
            manifest.version == 1 && manifest.cid == cid.as_str(),
            "theme manifest identity mismatch"
        );
        ensure!(
            manifest
                .files
                .len()
                .saturating_add(manifest.directories.len())
                <= self.limits.max_files,
            "too many cached theme paths"
        );
        let mut expected = BTreeSet::from([MANIFEST.to_owned()]);
        let mut aliases = BTreeMap::new();
        for name in manifest.directories.iter().chain(manifest.files.keys()) {
            safe_name(name)?;
            register_spelling(&mut aliases, name)?;
            let mut parents = BTreeSet::new();
            add_directories(&mut parents, name, false);
            ensure!(
                parents.is_subset(&manifest.directories),
                "theme manifest omits parent directory"
            );
            ensure!(
                expected.insert(name.clone()),
                "duplicate theme manifest path"
            );
        }
        // Reject links and excess/deep paths before opening any manifest-selected file.
        let mut actual = BTreeSet::new();
        collect_paths(folder, folder, &mut actual, expected.len(), 0)?;
        ensure!(actual == expected, "unexpected theme cache contents");
        for name in &manifest.directories {
            ensure!(
                fs::symlink_metadata(folder.join(name))?.is_dir(),
                "invalid theme directory"
            );
        }
        let mut total = 0u64;
        for (name, info) in &manifest.files {
            safe_name(name)?;
            let path = folder.join(name);
            let metadata = fs::symlink_metadata(&path)?;
            ensure!(
                metadata.is_file() && !metadata.file_type().is_symlink(),
                "invalid theme file"
            );
            ensure!(
                metadata.len() == info.size && info.size <= self.limits.max_file_bytes,
                "theme file size mismatch"
            );
            total = total
                .checked_add(info.size)
                .context("theme total size overflow")?;
            ensure!(
                total <= self.limits.max_unpacked_bytes,
                "theme cache exceeds size limit"
            );
            let mut file = fs::File::open(path)?.take(info.size.saturating_add(1));
            let mut hasher = blake3::Hasher::new();
            std::io::copy(&mut file, &mut hasher)?;
            ensure!(
                hasher.finalize().to_hex().as_str() == info.hash,
                "theme file checksum mismatch"
            );
        }
        Ok(())
    }

    pub fn trim(&self, now: SystemTime, max_age: Duration) -> Result<TrimReport> {
        let _guard = self
            .operations
            .lock()
            .map_err(|_| anyhow::anyhow!("theme cache lock poisoned"))?;
        let mut report = TrimReport::default();
        for entry in fs::read_dir(&self.root)? {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    report.errors.push((self.root.clone(), e.to_string()));
                    continue;
                }
            };
            let path = entry.path();
            // Root ownership and the operation mutex exclude active installers.
            if entry.file_name() == ".owner-lock" {
                continue;
            }
            let process = || -> Result<bool> {
                let name = entry.file_name().to_string_lossy().into_owned();
                let cid = name.parse::<ContentId>();
                let metadata = fs::symlink_metadata(&path)?;
                let old = metadata
                    .modified()
                    .ok()
                    .and_then(|time| now.duration_since(time).ok())
                    .is_some_and(|age| age > max_age);
                let valid = cid.as_ref().ok().is_some_and(|cid| {
                    metadata.is_dir()
                        && !metadata.file_type().is_symlink()
                        && self.verify(&path, cid).is_ok()
                });
                if old || !valid {
                    if metadata.is_dir() && !metadata.file_type().is_symlink() {
                        fs::remove_dir_all(&path)?;
                    } else {
                        fs::remove_file(&path)?;
                    }
                    return Ok(true);
                }
                Ok(false)
            };
            match process() {
                Ok(true) => report.removed.push(path),
                Ok(false) => {}
                Err(e) => report.errors.push((path, e.to_string())),
            }
        }
        Ok(report)
    }
}

fn safe_name(name: &str) -> Result<()> {
    ensure!(
        name.len() <= 4096 && name.split('/').count() <= 64,
        "theme path exceeds length limit"
    );
    ensure!(
        !name.is_empty()
            && !name.contains(['\\', ':', '<', '>', '"', '|', '?', '*'])
            && !name.chars().any(char::is_control),
        "invalid theme path"
    );
    ensure!(
        name.split('/').all(|part| !part.is_empty()
            && part != "."
            && part != ".."
            && !part.ends_with(['.', ' '])),
        "invalid theme path component"
    );
    for part in name.split('/') {
        let stem = part.split('.').next().unwrap_or("").to_ascii_uppercase();
        let numbered = stem
            .strip_prefix("COM")
            .or_else(|| stem.strip_prefix("LPT"));
        ensure!(
            !matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
                && !numbered.is_some_and(
                    |suffix| suffix.len() == 1 && matches!(suffix.as_bytes()[0], b'1'..=b'9')
                ),
            "reserved Windows theme path"
        );
    }
    ensure!(
        Path::new(name)
            .components()
            .all(|part| matches!(part, Component::Normal(_))),
        "theme path escapes package"
    );
    ensure!(
        !name.eq_ignore_ascii_case(MANIFEST) && !name.eq_ignore_ascii_case(".checksum"),
        "reserved theme path"
    );
    Ok(())
}
fn register_spelling(aliases: &mut BTreeMap<String, String>, name: &str) -> Result<()> {
    let mut current = String::new();
    for part in name.split('/') {
        if !current.is_empty() {
            current.push('/');
        }
        current.push_str(part);
        let canonical = aliases
            .entry(current.to_lowercase())
            .or_insert_with(|| current.clone());
        ensure!(*canonical == current, "case alias in theme archive path");
    }
    Ok(())
}
fn add_directories(directories: &mut BTreeSet<String>, name: &str, include_self: bool) {
    let mut path = if include_self {
        Some(Path::new(name))
    } else {
        Path::new(name).parent()
    };
    while let Some(value) = path.filter(|value| !value.as_os_str().is_empty()) {
        directories.insert(value.to_string_lossy().replace('\\', "/"));
        path = value.parent();
    }
}
fn collect_paths(
    root: &Path,
    folder: &Path,
    paths: &mut BTreeSet<String>,
    limit: usize,
    depth: usize,
) -> Result<()> {
    ensure!(depth <= 64, "theme cache exceeds directory depth limit");
    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();
        let meta = fs::symlink_metadata(&path)?;
        ensure!(
            !meta.file_type().is_symlink() && (meta.is_file() || meta.is_dir()),
            "unsupported cached theme file type"
        );
        paths.insert(
            path.strip_prefix(root)?
                .to_str()
                .context("non-UTF-8 theme path")?
                .replace('\\', "/"),
        );
        ensure!(paths.len() <= limit, "unexpected theme cache contents");
        if meta.is_dir() {
            collect_paths(root, &path, paths, limit, depth + 1)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    struct TestRoot(PathBuf);
    impl TestRoot {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "tg-cache-test-{}-{}",
                std::process::id(),
                NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&root).unwrap();
            Self(root)
        }
        fn cache(&self, limits: CacheLimits) -> Cache {
            Cache::new(self.0.clone(), limits).unwrap()
        }
    }
    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn archive(names: &[&str]) -> Vec<u8> {
        let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
        for name in names {
            zip.start_file(*name, zip::write::FileOptions::default())
                .unwrap();
            zip.write_all(b"theme data").unwrap();
        }
        zip.finish().unwrap().into_inner()
    }
    fn cid(bytes: &[u8]) -> ContentId {
        blake3::hash(bytes).to_string().parse().unwrap()
    }

    #[test]
    fn ownership_lock_releases_and_trim_recovers_abandoned_staging() {
        let root = TestRoot::new();
        let cache = root.cache(CacheLimits::default());
        assert!(Cache::new(root.0.clone(), CacheLimits::default()).is_err());
        fs::create_dir(cache.root().join(".tmp-abandoned")).unwrap();
        assert_eq!(
            cache
                .trim(SystemTime::now(), Duration::from_secs(60))
                .unwrap()
                .removed
                .len(),
            1
        );
        drop(cache);
        let _reopened = root.cache(CacheLimits::default());
    }
    #[test]
    fn installs_verifies_and_recovers_corrupt_or_extra_contents() {
        let root = TestRoot::new();
        let cache = root.cache(CacheLimits::default());
        let bytes = archive(&["nested/theme.json"]);
        let id = cid(&bytes);
        let path = cache.install(&id, bytes.as_slice()).unwrap();
        assert_eq!(cache.lookup(&id).unwrap(), Some(path.clone()));
        fs::write(path.join("extra"), b"unexpected").unwrap();
        assert_eq!(cache.lookup(&id).unwrap(), None);
        let path = cache.install(&id, bytes.as_slice()).unwrap();
        fs::write(path.join("nested/theme.json"), b"corrupted!").unwrap();
        assert_eq!(cache.lookup(&id).unwrap(), None);
        cache.install(&id, bytes.as_slice()).unwrap();
        let report = cache
            .trim(
                SystemTime::now() + Duration::from_secs(60),
                Duration::from_secs(1),
            )
            .unwrap();
        assert_eq!(report.removed, vec![path]);
    }
    #[test]
    fn rejects_bad_archives_without_publishing_or_leaking_staging() {
        let root = TestRoot::new();
        let cache = root.cache(CacheLimits::default());
        for names in [
            vec!["valid", "../outside"],
            vec!["duplicate", "duplicate"],
            vec![".tg-manifest.json"],
            vec![".checksum"],
            vec!["A/x", "a/y"],
            vec!["CON.txt"],
            vec!["nested/NUL"],
            vec!["COM1"],
            vec!["LPT9.log"],
            vec!["bad?.json"],
            vec!["bad\u{1}.json"],
        ] {
            let bytes = archive(&names);
            let id = cid(&bytes);
            assert!(cache.install(&id, bytes.as_slice()).is_err());
            assert_eq!(fs::read_dir(cache.root()).unwrap().count(), 1);
        }
        let bytes = archive(&["valid"]);
        assert!(cache.install(&cid(b"other"), bytes.as_slice()).is_err());
        assert_eq!(fs::read_dir(cache.root()).unwrap().count(), 1);
    }
    #[test]
    fn rejects_directory_payload_and_inconsistent_zip_size() {
        let root = TestRoot::new();
        let cache = root.cache(CacheLimits::default());
        // A normal explicit empty directory remains a supported package entry.
        let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
        zip.add_directory("empty/", zip::write::FileOptions::default())
            .unwrap();
        let empty = zip.finish().unwrap().into_inner();
        let path = cache.install(&cid(&empty), empty.as_slice()).unwrap();
        assert!(path.join("empty").is_dir());
        let mut corrupt_directory = empty.clone();
        let central = corrupt_directory
            .windows(4)
            .position(|bytes| bytes == b"PK\x01\x02")
            .unwrap();
        corrupt_directory[central + 16..central + 20].copy_from_slice(&1u32.to_le_bytes());
        assert!(
            cache
                .install(&cid(&corrupt_directory), corrupt_directory.as_slice())
                .is_err()
        );
        let directory_payload = archive(&["directory/"]);
        assert!(
            cache
                .install(&cid(&directory_payload), directory_payload.as_slice())
                .is_err(),
            "directory payload must be rejected"
        );
        let mut wrong_size = archive(&["file"]);
        let central = wrong_size
            .windows(4)
            .position(|bytes| bytes == b"PK\x01\x02")
            .unwrap();
        wrong_size[central + 24..central + 28].copy_from_slice(&1u32.to_le_bytes());
        assert!(
            cache
                .install(&cid(&wrong_size), wrong_size.as_slice())
                .is_err(),
            "wrong uncompressed size must be rejected"
        );
        assert!(fs::read_dir(cache.root()).unwrap().all(|entry| {
            !entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with(".tmp-")
        }));
    }
    #[test]
    fn enforces_archive_file_and_total_limits() {
        let bytes = archive(&["a", "b"]);
        let id = cid(&bytes);
        let limits = [
            CacheLimits {
                max_archive_bytes: 1,
                ..Default::default()
            },
            CacheLimits {
                max_files: 1,
                ..Default::default()
            },
            CacheLimits {
                max_file_bytes: 1,
                ..Default::default()
            },
            CacheLimits {
                max_file_bytes: 15,
                max_unpacked_bytes: 15,
                ..Default::default()
            },
        ];
        for limit in limits {
            let root = TestRoot::new();
            let cache = root.cache(limit);
            assert!(cache.install(&id, bytes.as_slice()).is_err());
            assert_eq!(fs::read_dir(cache.root()).unwrap().count(), 1);
        }
    }
    #[cfg(unix)]
    #[test]
    fn rejects_manifest_parent_links_before_reading_files_and_bounds_tree_depth() {
        use std::os::unix::fs::symlink;
        let root = TestRoot::new();
        let cache = root.cache(CacheLimits::default());
        let outside = TestRoot::new();
        fs::write(outside.0.join("secret"), b"outside").unwrap();
        let id = cid(b"manifest-link");
        let folder = cache.root().join(id.as_str());
        fs::create_dir(&folder).unwrap();
        symlink(&outside.0, folder.join("parent")).unwrap();
        let mut manifest = Manifest {
            version: 1,
            cid: id.as_str().into(),
            files: BTreeMap::from([(
                "parent/secret".into(),
                FileInfo {
                    size: 7,
                    hash: blake3::hash(b"outside").to_string(),
                },
            )]),
            directories: BTreeSet::new(),
        };
        fs::write(
            folder.join(MANIFEST),
            serde_json::to_vec(&manifest).unwrap(),
        )
        .unwrap();
        assert!(
            cache
                .verify(&folder, &id)
                .unwrap_err()
                .to_string()
                .contains("omits parent")
        );
        manifest.directories.insert("parent".into());
        fs::write(
            folder.join(MANIFEST),
            serde_json::to_vec(&manifest).unwrap(),
        )
        .unwrap();
        assert!(
            cache
                .verify(&folder, &id)
                .unwrap_err()
                .to_string()
                .contains("unsupported cached")
        );
        assert_eq!(cache.lookup(&id).unwrap(), None);
        assert!(outside.0.join("secret").exists());
        let deep = root.0.join("deep");
        fs::create_dir(&deep).unwrap();
        let mut path = deep.clone();
        for _ in 0..66 {
            path.push("d");
            fs::create_dir(&path).unwrap();
        }
        assert!(
            collect_paths(&deep, &deep, &mut BTreeSet::new(), 100, 0)
                .unwrap_err()
                .to_string()
                .contains("depth")
        );
    }
    #[cfg(unix)]
    #[test]
    fn rejects_symlink_roots_and_discards_symlink_entries_without_touching_target() {
        use std::os::unix::fs::symlink;
        let root = TestRoot::new();
        let target = TestRoot::new();
        let link = root.0.join("link");
        symlink(&target.0, &link).unwrap();
        assert!(Cache::new(link, CacheLimits::default()).is_err());
        let cache = root.cache(CacheLimits::default());
        let id = cid(b"symlink");
        symlink(&target.0, cache.root().join(id.as_str())).unwrap();
        fs::write(target.0.join("keep"), b"keep").unwrap();
        assert_eq!(cache.lookup(&id).unwrap(), None);
        assert!(target.0.join("keep").exists());
    }
}
