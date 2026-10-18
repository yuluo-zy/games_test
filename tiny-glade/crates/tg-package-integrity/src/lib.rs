#![doc = include_str!("../README.md")]
use relative_path::RelativePath;
use serde::Deserialize;
use std::{
    collections::HashSet,
    fs,
    hash::Hasher,
    io::{self, Read},
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
};
use unicode_normalization::UnicodeNormalization;

pub const MANIFEST_PATH: &str = "build-info/manifest.json";
const FNV_INITIAL_STATE: u64 = 0xf52d_ad71_b327_e45a;

/// Versioned manifest with ordered `(relative path, lowercase BLAKE3 digest)` entries.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct PackageManifest {
    pub version: u32,
    pub files: Vec<(String, String)>,
}

/// Complete file differences; missing/corrupt follow manifest order, extras are sorted.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FileIssues {
    pub missing: Vec<String>,
    pub corrupt: Vec<String>,
    pub extra: Vec<String>,
}

/// Failure at a verification stage; later stages are not evaluated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageVerificationError {
    InvalidManifestChecksum,
    ManifestRead(String),
    ManifestParse(String),
    UnsupportedVersion(u32),
    Files(FileIssues),
    InvalidManifestPath(String),
    InvalidManifestEntry { path: String, reason: &'static str },
    Filesystem(String),
}

impl std::fmt::Display for PackageVerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidManifestChecksum => f.write_str("manifest checksum is invalid"),
            Self::ManifestRead(message) => write!(f, "cannot read manifest: {message}"),
            Self::ManifestParse(message) => write!(f, "cannot parse manifest: {message}"),
            Self::UnsupportedVersion(version) => {
                write!(f, "unsupported manifest version: {version}")
            }
            Self::Files(issues) => write!(
                f,
                "package file issues: {} missing, {} corrupt, {} extra",
                issues.missing.len(),
                issues.corrupt.len(),
                issues.extra.len()
            ),
            Self::InvalidManifestPath(path) => write!(f, "invalid manifest path: {path:?}"),
            Self::InvalidManifestEntry { path, reason } => {
                write!(f, "invalid manifest entry {path:?}: {reason}")
            }
            Self::Filesystem(message) => write!(f, "cannot inspect package filesystem: {message}"),
        }
    }
}

impl std::error::Error for PackageVerificationError {}

/// Raw JSON bytes, including whitespace, affect this gate.
/// BLAKE3 -> little-endian u64 length (32) and digest -> seeded FNV-1a.
/// The low 16 bits must be zero. The length encoding is fixed across platforms.
pub fn is_valid_manifest_checksum(bytes: &[u8]) -> bool {
    let digest = blake3::hash(bytes);
    let mut hasher = fnv::FnvHasher::with_key(FNV_INITIAL_STATE);
    hasher.write(&32_u64.to_le_bytes());
    hasher.write(digest.as_bytes());
    hasher.finish() as u16 == 0
}

/// Select a package root explicitly rather than changing process current directory.
pub fn verify_package_manifest(root: impl AsRef<Path>) -> Result<(), PackageVerificationError> {
    let root = root.as_ref();
    let mut bytes = Vec::new();
    open_package_file(root, MANIFEST_PATH)
        .and_then(|mut file| file.read_to_end(&mut bytes))
        .map_err(|error| PackageVerificationError::ManifestRead(error.to_string()))?;
    verify_manifest_bytes(root, &bytes)
}

/// Verify supplied manifest bytes against the package rooted at `root`.
///
/// The bytes are not written to disk. A listed manifest still must exist as a
/// regular file, but its contents are exempt from digest comparison.
pub fn verify_manifest_bytes(
    root: impl AsRef<Path>,
    bytes: &[u8],
) -> Result<(), PackageVerificationError> {
    if !is_valid_manifest_checksum(bytes) {
        return Err(PackageVerificationError::InvalidManifestChecksum);
    }
    let manifest: PackageManifest = serde_json::from_slice(bytes)
        .map_err(|error| PackageVerificationError::ManifestParse(error.to_string()))?;
    if manifest.version != 1 {
        return Err(PackageVerificationError::UnsupportedVersion(
            manifest.version,
        ));
    }
    validate_entries(&manifest)?;
    let issues = check_files(root.as_ref(), &manifest)
        .map_err(|error| PackageVerificationError::Filesystem(error.to_string()))?;
    if issues.missing.is_empty() && issues.corrupt.is_empty() && issues.extra.is_empty() {
        Ok(())
    } else {
        Err(PackageVerificationError::Files(issues))
    }
}

fn validate_entries(manifest: &PackageManifest) -> Result<(), PackageVerificationError> {
    let mut names = HashSet::with_capacity(manifest.files.len());
    for (name, digest) in &manifest.files {
        if !is_package_path(name) {
            return Err(PackageVerificationError::InvalidManifestPath(name.clone()));
        }
        // Avoid listing one filesystem identity through multiple spellings.
        let identity: String = name
            .nfc()
            .flat_map(char::to_lowercase)
            .flat_map(char::to_uppercase)
            .nfc()
            .collect();
        let reason = if !names.insert(identity) {
            Some("duplicate path, case alias or Unicode normalization alias")
        } else if name != MANIFEST_PATH
            && (digest.len() != 64
                || !digest
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)))
        {
            Some("expected a lowercase 64-digit BLAKE3 hash")
        } else {
            None
        };
        if let Some(reason) = reason {
            return Err(PackageVerificationError::InvalidManifestEntry {
                path: name.clone(),
                reason,
            });
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum CheckOutcome {
    Valid,
    Missing,
    Corrupt,
}

// Manifest paths are portable, unambiguous names, never OS path expressions.
fn is_package_path(name: &str) -> bool {
    name.split('/').all(|part| {
        if part.is_empty()
            || part == "."
            || part == ".."
            || part.ends_with([' ', '.'])
            || part.chars().any(|c| {
                c.is_control() || matches!(c, '\\' | ':' | '<' | '>' | '"' | '|' | '?' | '*')
            })
        {
            return false;
        }
        let stem = part
            .split('.')
            .next()
            .unwrap_or_default()
            .to_ascii_uppercase();
        !matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
            && !matches!(
                stem.strip_prefix("COM")
                    .or_else(|| stem.strip_prefix("LPT")),
                Some("1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³")
            )
    })
}

// Do not follow links or open devices/FIFOs. The caller owns a stable package
// tree; preventing concurrent filesystem replacement requires OS-specific APIs.
fn open_package_file(root: &Path, name: &str) -> io::Result<fs::File> {
    let mut path = root.to_path_buf();
    let mut parts = name.split('/').peekable();
    while let Some(part) = parts.next() {
        path.push(part);
        let kind = fs::symlink_metadata(&path)?.file_type();
        if kind.is_symlink()
            || (parts.peek().is_some() && !kind.is_dir())
            || (parts.peek().is_none() && !kind.is_file())
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("not a regular package file: {}", path.display()),
            ));
        }
    }
    fs::File::open(path)
}

fn check_file(root: &Path, name: &str, expected: &str) -> CheckOutcome {
    if name == MANIFEST_PATH {
        // The self digest is exempt, but listing it still promises a regular file.
        return match open_package_file(root, name) {
            Ok(_) => CheckOutcome::Valid,
            Err(_) => CheckOutcome::Missing,
        };
    }
    // Stream hashes so each worker holds only a small read buffer.
    let digest = (|| -> io::Result<blake3::Hash> {
        let mut file = open_package_file(root, name)?;
        let mut hasher = blake3::Hasher::new();
        hasher.update_reader(&mut file)?;
        Ok(hasher.finalize())
    })();
    match digest {
        Err(_) => CheckOutcome::Missing,
        Ok(hash) if hash.to_hex().as_str() == expected => CheckOutcome::Valid,
        Ok(_) => CheckOutcome::Corrupt,
    }
}

fn check_claimed_files(
    root: &Path,
    manifest: &PackageManifest,
    next: &AtomicUsize,
) -> Vec<(usize, CheckOutcome)> {
    let mut failures = Vec::new();
    loop {
        let index = next.fetch_add(1, Ordering::Relaxed);
        let Some((name, expected)) = manifest.files.get(index) else {
            break;
        };
        let outcome = check_file(root, name, expected);
        if !matches!(outcome, CheckOutcome::Valid) {
            failures.push((index, outcome));
        }
    }
    failures
}

fn check_files(root: &Path, manifest: &PackageManifest) -> io::Result<FileIssues> {
    check_files_with_workers(
        root,
        manifest,
        std::thread::available_parallelism().map_or(1, usize::from),
    )
}

fn check_files_with_workers(
    root: &Path,
    manifest: &PackageManifest,
    worker_limit: usize,
) -> io::Result<FileIssues> {
    let worker_count = worker_limit.clamp(1, 64).min(manifest.files.len().max(1));
    let next = AtomicUsize::new(0);
    // Reuse one bounded worker group across the entire manifest. Dynamic claims
    // keep a worker processing small files while another reads a larger file.
    let completed = std::thread::scope(|scope| {
        let mut workers = Vec::with_capacity(worker_count.saturating_sub(1));
        for _ in 1..worker_count {
            match std::thread::Builder::new()
                .spawn_scoped(scope, || check_claimed_files(root, manifest, &next))
            {
                Ok(worker) => workers.push(worker),
                // Existing workers and this thread can finish every remaining
                // index; thread exhaustion need not abort package verification.
                Err(_) => break,
            }
        }
        let mut completed = check_claimed_files(root, manifest, &next);
        for worker in workers {
            completed.extend(worker.join().expect("package verification worker panicked"));
        }
        completed
    });
    let mut outcomes = vec![CheckOutcome::Valid; manifest.files.len()];
    for (index, outcome) in completed {
        outcomes[index] = outcome;
    }
    let mut issues = FileIssues::default();
    for ((name, _), outcome) in manifest.files.iter().zip(outcomes) {
        match outcome {
            CheckOutcome::Valid => {}
            CheckOutcome::Missing => issues.missing.push(name.clone()),
            CheckOutcome::Corrupt => issues.corrupt.push(name.clone()),
        }
    }
    let known = manifest
        .files
        .iter()
        .map(|(name, _)| name.clone())
        .collect();
    issues.extra = find_extra_files(root, &known)?;
    Ok(issues)
}

/// Recursively list unlisted files in deterministic order, excluding root `tmp`.
///
/// Returns an error when any directory cannot be inspected, a filename is not
/// UTF-8, or a symlink/special file is encountered. An incomplete scan cannot
/// establish integrity. `known` uses exact forward-slash relative names.
pub fn find_extra_files(
    root: impl AsRef<Path>,
    known: &HashSet<String>,
) -> io::Result<Vec<String>> {
    let mut extra = Vec::new();
    let mut pending = vec![(
        root.as_ref().to_path_buf(),
        RelativePath::new("").to_owned(),
    )];
    while let Some((disk, relative)) = pending.pop() {
        // Finish this directory before opening a child. Keeping ReadDir alive
        // through recursive calls would consume one descriptor per depth level.
        for entry in fs::read_dir(&disk)? {
            let entry = entry?;
            let kind = entry.file_type()?;
            let name = entry.file_name();
            let name = name.to_str().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("non-UTF-8 package filename: {}", entry.path().display()),
                )
            })?;
            let path = relative.join(name);
            if path == RelativePath::new("tmp") {
                continue;
            }
            if kind.is_dir() {
                pending.push((entry.path(), path));
            } else if !kind.is_file() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("not a regular package file: {}", entry.path().display()),
                ));
            } else if !known.contains(path.as_str()) {
                extra.push(path.into_string());
            }
        }
    }
    extra.sort_unstable();
    Ok(extra)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct DiskFixture(std::path::PathBuf);
    impl DiskFixture {
        fn new() -> Self {
            let root = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("target/test-fixtures")
                .join(format!(
                    "{}-{}",
                    std::process::id(),
                    NEXT.fetch_add(1, Ordering::Relaxed)
                ));
            fs::create_dir_all(&root).unwrap();
            Self(root)
        }
        fn write(&self, name: &str, content: &[u8]) {
            let file = self.0.join(name);
            fs::create_dir_all(file.parent().unwrap()).unwrap();
            fs::write(file, content).unwrap();
        }
    }
    impl Drop for DiskFixture {
        fn drop(&mut self) {
            let base = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("target/test-fixtures")
                .canonicalize()
                .unwrap();
            let target = self.0.canonicalize().unwrap();
            assert!(
                target.starts_with(&base) && target != base,
                "fixture cleanup must stay inside its owned directory"
            );
            fs::remove_dir_all(&target).unwrap();
        }
    }
    fn expected(bytes: &[u8]) -> String {
        blake3::hash(bytes).to_hex().to_string()
    }

    // The low-16 checksum is a format gate; vary ignored whitespace to make a
    // controlled candidate fixture pass it. This helper does not implement a hash.
    fn checked_bytes(json: &[u8]) -> Vec<u8> {
        let mut bytes = json.to_vec();
        for nonce in 0..1_000_000_u32 {
            bytes.truncate(json.len());
            for bit in 0..20 {
                bytes.push(if nonce & (1 << bit) == 0 { b' ' } else { b'\t' });
            }
            if is_valid_manifest_checksum(&bytes) {
                return bytes;
            }
        }
        panic!("fixture checksum search exhausted");
    }

    #[test]
    fn synthetic_manifest_schema_and_raw_checksum() {
        // A checksum-valid synthetic input keeps this test independent of package data.
        let contract: serde_json::Value =
            serde_json::from_str(include_str!("../fixtures/manifest-contract.json")).unwrap();
        let case = contract["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|case| case["name"] == "success")
            .unwrap();
        let bytes = case["bytes"].as_str().unwrap().as_bytes();
        let manifest: PackageManifest = serde_json::from_slice(bytes).unwrap();
        assert_eq!(manifest.version, 1);
        assert_eq!(manifest.files.len(), 1);
        assert_eq!(manifest.files.last().unwrap().0, MANIFEST_PATH);
        assert!(
            is_valid_manifest_checksum(bytes),
            "saved synthetic manifest must pass the checksum gate"
        );
        let mut modified = bytes.to_vec();
        modified.push(b' ');
        assert!(
            !is_valid_manifest_checksum(&modified),
            "whitespace is checksum-covered"
        );
    }

    #[test]
    fn stable_disk_aggregates_errors_and_skips_self_digest() {
        let disk = DiskFixture::new();
        disk.write(MANIFEST_PATH, b"self digest is exempt");
        disk.write("good", b"good");
        disk.write("damaged", b"damaged");
        disk.write("uppercase", b"good");
        disk.write("extra", b"extra");
        disk.write("tmp/transient", b"ignored");
        disk.write("sub/tmp/transient", b"extra");
        disk.write("file.tmp", b"extra");
        let manifest = PackageManifest {
            version: 1,
            files: vec![
                ("good".into(), expected(b"good")),
                ("missing".into(), expected(b"missing")),
                ("damaged".into(), expected(b"expected")),
                ("uppercase".into(), expected(b"good").to_uppercase()),
                ("missing".into(), "invalid hash text".into()),
                (
                    MANIFEST_PATH.into(),
                    "self hash is deliberately irrelevant".into(),
                ),
            ],
        };
        let issues = check_files(&disk.0, &manifest).unwrap();
        assert_eq!(issues.missing, ["missing", "missing"]);
        assert_eq!(issues.corrupt, ["damaged", "uppercase"]);
        let mut extra = issues.extra;
        extra.sort();
        assert_eq!(extra, ["extra", "file.tmp", "sub/tmp/transient"]);
    }

    #[test]
    fn manifest_read_gate_parse_version_and_full_success() {
        let disk = DiskFixture::new();
        assert!(matches!(
            verify_package_manifest(&disk.0),
            Err(PackageVerificationError::ManifestRead(_))
        ));
        assert_eq!(
            verify_manifest_bytes(&disk.0, b"not json"),
            Err(PackageVerificationError::InvalidManifestChecksum)
        );
        let invalid = checked_bytes(b"not json");
        assert!(matches!(
            verify_manifest_bytes(&disk.0, &invalid),
            Err(PackageVerificationError::ManifestParse(_))
        ));
        let unsupported = checked_bytes(br#"{"version":2,"files":[]}"#);
        assert_eq!(
            verify_manifest_bytes(&disk.0, &unsupported),
            Err(PackageVerificationError::UnsupportedVersion(2))
        );
        disk.write("good", b"good");
        let json = serde_json::to_vec(&serde_json::json!({
            "version": 1, "files": [["good", expected(b"good")], [MANIFEST_PATH, "ignored"]]
        }))
        .unwrap();
        let valid = checked_bytes(&json);
        disk.write(MANIFEST_PATH, &valid);
        assert_eq!(verify_package_manifest(&disk.0), Ok(()));
        disk.write("good", b"changed");
        assert_eq!(
            verify_package_manifest(&disk.0),
            Err(PackageVerificationError::Files(FileIssues {
                corrupt: vec!["good".into()],
                ..Default::default()
            }))
        );
    }

    #[test]
    fn manifest_self_entry_requires_a_regular_file() {
        let disk = DiskFixture::new();
        let bytes =
            checked_bytes(br#"{"version":1,"files":[["build-info/manifest.json","ignored"]]}"#);
        let missing = PackageVerificationError::Files(FileIssues {
            missing: vec![MANIFEST_PATH.into()],
            ..Default::default()
        });
        assert_eq!(verify_manifest_bytes(&disk.0, &bytes), Err(missing.clone()));
        fs::create_dir_all(disk.0.join(MANIFEST_PATH)).unwrap();
        assert_eq!(verify_manifest_bytes(&disk.0, &bytes), Err(missing));
        fs::remove_dir(disk.0.join(MANIFEST_PATH)).unwrap();
        disk.write(MANIFEST_PATH, b"caller-supplied bytes need not match");
        assert_eq!(verify_manifest_bytes(&disk.0, &bytes), Ok(()));
    }

    #[test]
    fn incomplete_scans_fail_verification() {
        let disk = DiskFixture::new();
        let known = HashSet::new();
        assert!(find_extra_files(disk.0.join("does-not-exist"), &known).is_err());
        disk.write("a", b"a");
        assert!(find_extra_files(disk.0.join("a"), &known).is_err());
        let bytes = checked_bytes(br#"{"version":1,"files":[]}"#);
        assert!(matches!(
            verify_manifest_bytes(disk.0.join("absent"), &bytes),
            Err(PackageVerificationError::Filesystem(_))
        ));
    }

    #[test]
    fn schema_contract_matches_saved_manifest_cases() {
        // Saved checksum-valid inputs keep schema tests independent of nonce search.
        let contract: serde_json::Value =
            serde_json::from_str(include_str!("../fixtures/manifest-contract.json")).unwrap();
        let disk = DiskFixture::new();
        disk.write(MANIFEST_PATH, b"self digest is exempt");
        for case in contract["cases"].as_array().unwrap() {
            let bytes = case["bytes"].as_str().unwrap().as_bytes();
            assert!(is_valid_manifest_checksum(bytes), "{}", case["name"]);
            let actual = match verify_manifest_bytes(&disk.0, bytes) {
                Ok(()) => serde_json::json!({"kind":"ok"}),
                Err(PackageVerificationError::ManifestParse(message)) => {
                    serde_json::json!({"kind":"parse","message":message})
                }
                Err(PackageVerificationError::UnsupportedVersion(version)) => {
                    serde_json::json!({"kind":"version","version":version})
                }
                other => panic!("unexpected {}: {other:?}", case["name"]),
            };
            assert_eq!(actual, case["expected"], "{}", case["name"]);
        }
    }

    #[test]
    fn mixed_file_issues_preserve_manifest_order_and_extra_policy() {
        let disk = DiskFixture::new();
        disk.write(MANIFEST_PATH, b"self digest is exempt");
        disk.write("good", b"content");
        disk.write("bad", b"actual");
        disk.write("extra", b"extra");
        disk.write("tmp/ignored", b"ignored");
        disk.write("sub/tmp/nested", b"extra");
        let mut files = vec![
            (MANIFEST_PATH.to_string(), "ignored".to_string()),
            ("good".into(), expected(b"content")),
            ("bad".into(), expected(b"different")),
        ];
        let missing: Vec<_> = (0..130).map(|i| format!("missing-{i}")).collect();
        files.extend(
            missing
                .iter()
                .map(|name| (name.clone(), expected(b"missing"))),
        );
        let manifest = PackageManifest { version: 1, files };
        let expected = FileIssues {
            missing,
            corrupt: vec!["bad".into()],
            extra: vec!["extra".into(), "sub/tmp/nested".into()],
        };
        // The caller drains the same index queue when no worker can be created.
        assert_eq!(
            check_files_with_workers(&disk.0, &manifest, 1).unwrap(),
            expected
        );
        assert_eq!(
            check_files_with_workers(&disk.0, &manifest, 8).unwrap(),
            expected
        );
    }

    #[test]
    fn manifest_paths_cannot_escape_or_use_platform_aliases() {
        let disk = DiskFixture::new();
        for name in [
            "",
            "/outside",
            "../outside",
            "a/../outside",
            "./file",
            "a//b",
            "a/",
            "a\\b",
            "C:/file",
            "file:stream",
            "file.",
            "a /b",
            "NUL",
            "aux.txt",
            "COM1",
        ] {
            let bytes = checked_bytes(
                &serde_json::to_vec(&serde_json::json!({
                    "version": 1, "files": [[name, "irrelevant"]],
                }))
                .unwrap(),
            );
            assert_eq!(
                verify_manifest_bytes(&disk.0, &bytes),
                Err(PackageVerificationError::InvalidManifestPath(name.into())),
                "{name}"
            );
        }
        assert!(is_package_path("assets/场景.bin"));
    }

    #[test]
    fn manifest_entries_reject_ambiguous_paths_and_malformed_hashes() {
        let hash = expected(b"data");
        for files in [
            vec![
                ("asset".into(), hash.clone()),
                ("asset".into(), hash.clone()),
            ],
            vec![
                ("Asset".into(), hash.clone()),
                ("asset".into(), hash.clone()),
            ],
            vec![("Ä".into(), hash.clone()), ("ä".into(), hash.clone())],
            vec![("ẞ".into(), hash.clone()), ("ß".into(), hash.clone())],
            vec![("σ".into(), hash.clone()), ("ς".into(), hash.clone())],
            vec![("ſ".into(), hash.clone()), ("S".into(), hash.clone())],
            vec![("ı".into(), hash.clone()), ("I".into(), hash.clone())],
            vec![
                ("é".into(), hash.clone()),
                ("e\u{301}".into(), hash.clone()),
            ],
            vec![("asset".into(), String::new())],
            vec![("asset".into(), hash.to_uppercase())],
            vec![("asset".into(), "z".repeat(64))],
        ] {
            assert!(matches!(
                validate_entries(&PackageManifest { version: 1, files }),
                Err(PackageVerificationError::InvalidManifestEntry { .. })
            ));
        }
        assert!(
            validate_entries(&PackageManifest {
                version: 1,
                files: vec![
                    (MANIFEST_PATH.into(), String::new()),
                    ("asset".into(), hash)
                ],
            })
            .is_ok()
        );
    }

    #[cfg(unix)]
    #[test]
    fn deep_scan_uses_bounded_directory_descriptors() {
        const CHILD_ROOT: &str = "TG_INTEGRITY_DEEP_SCAN_ROOT";
        if let Some(root) = std::env::var_os(CHILD_ROOT) {
            let expected = format!("{}file", "d/".repeat(100));
            assert_eq!(find_extra_files(root, &HashSet::new()).unwrap(), [expected]);
            return;
        }
        let disk = DiskFixture::new();
        disk.write(&format!("{}file", "d/".repeat(100)), b"data");
        let output = std::process::Command::new("/bin/sh")
            .args(["-c", r#"ulimit -n 64 && exec "$@""#, "integrity-test"])
            .arg(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "tests::deep_scan_uses_bounded_directory_descriptors",
                "--test-threads=1",
            ])
            .env(CHILD_ROOT, &disk.0)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn unknown_fields_are_ignored_and_version_must_be_u32() {
        // Unknown fields remain forward-compatible; version type errors fail
        // deserialization before the supported-version check.
        let disk = DiskFixture::new();
        disk.write("good", b"good");
        let json = serde_json::to_vec(&serde_json::json!({
            "version": 1,
            "files": [["good", expected(b"good")], [MANIFEST_PATH, "ignored"]],
            "unknown_nested": {"flag": true, "arr": [1, 2]},
        }))
        .unwrap();
        let valid = checked_bytes(&json);
        disk.write(MANIFEST_PATH, &valid);
        assert_eq!(verify_package_manifest(&disk.0), Ok(()));

        let type_disk = DiskFixture::new();
        type_disk.write("good", b"good");
        let float_version = checked_bytes(br#"{"version":1.5,"files":[]}"#);
        type_disk.write(MANIFEST_PATH, &float_version);
        assert!(matches!(
            verify_package_manifest(&type_disk.0),
            Err(PackageVerificationError::ManifestParse(_))
        ));

        let string_disk = DiskFixture::new();
        string_disk.write("good", b"good");
        let string_version = checked_bytes(br#"{"version":"1","files":[]}"#);
        string_disk.write(MANIFEST_PATH, &string_version);
        assert!(matches!(
            verify_package_manifest(&string_disk.0),
            Err(PackageVerificationError::ManifestParse(_))
        ));
    }

    #[cfg(unix)]
    #[test]
    fn package_links_cannot_be_followed_or_silently_accepted() {
        use std::os::unix::fs::symlink;
        let disk = DiskFixture::new();
        disk.write("real", b"data");
        disk.write("sub/file", b"data");
        symlink(disk.0.join("real"), disk.0.join("link-file")).unwrap();
        symlink(disk.0.join("sub"), disk.0.join("link-dir")).unwrap();
        assert!(open_package_file(&disk.0, "link-file").is_err());
        assert!(open_package_file(&disk.0, "link-dir/file").is_err());
        assert!(find_extra_files(&disk.0, &HashSet::new()).is_err());
    }

    // APFS rejects ill-formed UTF-8 names before they reach the directory scanner.
    #[cfg(target_os = "linux")]
    #[test]
    fn non_utf8_names_cannot_be_silently_accepted() {
        use std::{ffi::OsString, os::unix::ffi::OsStringExt};
        let disk = DiskFixture::new();
        fs::write(disk.0.join(OsString::from_vec(vec![0xff])), b"data").unwrap();
        assert!(find_extra_files(&disk.0, &HashSet::new()).is_err());
    }
}
