//! Recovered B package-integrity policy. Candidate Rust interface, not original ABI.
use relative_path::RelativePath;
use serde::Deserialize;
use std::{
    collections::HashSet,
    fs,
    hash::{Hash, Hasher},
    path::Path,
};

pub const MANIFEST_PATH: &str = "build-info/manifest.json";
const FNV_INITIAL_STATE: u64 = 0xf52d_ad71_b327_e45a;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct PackageManifest {
    pub version: u32,
    pub files: Vec<(String, String)>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FileIssues {
    pub missing: Vec<String>,
    pub corrupt: Vec<String>,
    pub extra: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageVerificationError {
    InvalidManifestChecksum,
    ManifestRead(String),
    ManifestParse(String),
    UnsupportedVersion(u32),
    Files(FileIssues),
}

/// Raw JSON bytes, including whitespace, affect this gate.
/// BLAKE3 -> Rust Hash<[u8; 32]> -> seeded FNV-1a -> low 16 bits == 0.
/// Hash::hash preserves the array length prefix; Hasher::write alone does not.
pub fn is_valid_manifest_checksum(bytes: &[u8]) -> bool {
    let digest = blake3::hash(bytes);
    let mut hasher = fnv::FnvHasher::with_key(FNV_INITIAL_STATE);
    digest.as_bytes().hash(&mut hasher);
    hasher.finish() as u16 == 0
}

/// Select a package root explicitly rather than changing process current directory.
pub fn verify_package_manifest(root: impl AsRef<Path>) -> Result<(), PackageVerificationError> {
    let root = root.as_ref();
    let bytes = fs::read(root.join(MANIFEST_PATH))
        .map_err(|error| PackageVerificationError::ManifestRead(error.to_string()))?;
    verify_manifest_bytes(root, &bytes)
}

/// Same verification chain with caller supplied bytes, useful for offline packages.
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
    let issues = check_files(root.as_ref(), &manifest);
    if issues.missing.is_empty() && issues.corrupt.is_empty() && issues.extra.is_empty() {
        Ok(())
    } else {
        Err(PackageVerificationError::Files(issues))
    }
}

enum CheckOutcome {
    Valid,
    Missing(String),
    Corrupt(String),
}

fn check_files(root: &Path, manifest: &PackageManifest) -> FileIssues {
    let mut issues = FileIssues::default();
    // Original easy-parallel creates one job per file. Bounded batches retain ordered
    // results and the same stable-filesystem policy without thousands of live threads.
    for batch in manifest.files.chunks(64) {
        let outcomes = easy_parallel::Parallel::new()
            .each(batch, |(name, expected)| {
                if name == MANIFEST_PATH {
                    return CheckOutcome::Valid;
                }
                // 使用原 BLAKE3 的 reader 接口，避免每个并发任务持有整文件。
                // 打开或读取失败仍统一归 Missing，保留原稳定文件系统规则。
                let digest = (|| -> std::io::Result<blake3::Hash> {
                    let mut file = fs::File::open(root.join(name))?;
                    let mut hasher = blake3::Hasher::new();
                    hasher.update_reader(&mut file)?;
                    Ok(hasher.finalize())
                })();
                match digest {
                    Err(_) => CheckOutcome::Missing(name.clone()),
                    Ok(hash) if hash.to_hex().as_str() == expected => CheckOutcome::Valid,
                    Ok(_) => CheckOutcome::Corrupt(name.clone()),
                }
            })
            .run();
        for outcome in outcomes {
            match outcome {
                CheckOutcome::Valid => {}
                CheckOutcome::Missing(name) => issues.missing.push(name),
                CheckOutcome::Corrupt(name) => issues.corrupt.push(name),
            }
        }
    }
    let known = manifest
        .files
        .iter()
        .map(|(name, _)| name.clone())
        .collect();
    issues.extra = find_extra_files(root, &known);
    issues
}

/// Unknown files, recursive directories; ignore root tmp, symlinks and failed
/// directory entries. Skip names that cannot be represented as UTF-8.
/// Membership uses exact stored strings, matching the original String hash set.
pub fn find_extra_files(root: impl AsRef<Path>, known: &HashSet<String>) -> Vec<String> {
    fn walk(
        disk: &Path,
        relative: &RelativePath,
        known: &HashSet<String>,
        extra: &mut Vec<String>,
    ) {
        let Ok(entries) = fs::read_dir(disk) else {
            return;
        };
        for entry in entries.flatten() {
            let Ok(kind) = entry.file_type() else {
                continue;
            };
            let name = entry.file_name();
            let Some(name) = name.to_str() else {
                continue;
            };
            let path = relative.join(name);
            if path == RelativePath::new("tmp") || kind.is_symlink() {
                continue;
            }
            if kind.is_dir() {
                walk(&entry.path(), &path, known, extra);
            } else if !known.contains(path.as_str()) {
                extra.push(path.into_string());
            }
        }
    }
    let mut extra = Vec::new();
    walk(root.as_ref(), RelativePath::new(""), known, &mut extra);
    extra
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
        // 发布源码只嵌入合成输入；真实发行清单由本地运行验证读取。
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
            "original-code-verified synthetic manifest must pass"
        );
        let mut modified = bytes.to_vec();
        modified.push(b' ');
        assert!(
            !is_valid_manifest_checksum(&modified),
            "whitespace is checksum-covered"
        );
    }

    #[test]
    fn stable_disk_aggregates_errors_and_skips_self() {
        let disk = DiskFixture::new();
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
        let issues = check_files(&disk.0, &manifest);
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
    fn extra_directory_errors_are_silent() {
        let disk = DiskFixture::new();
        let known = HashSet::new();
        assert!(find_extra_files(disk.0.join("does-not-exist"), &known).is_empty());
        disk.write("a", b"a");
        assert!(find_extra_files(disk.0.join("a"), &known).is_empty());
    }

    #[test]
    fn schema_contract_matches_original_verify_container() {
        // 预先保存通过校验门的原码对照输入，避免测试重复搜索校验 nonce。
        let contract: serde_json::Value =
            serde_json::from_str(include_str!("../fixtures/manifest-contract.json")).unwrap();
        let disk = DiskFixture::new();
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
    fn full_entry_preserves_cross_batch_errors_and_extra_policy() {
        let contract: serde_json::Value =
            serde_json::from_str(include_str!("../fixtures/manifest-contract.json")).unwrap();
        let disk = DiskFixture::new();
        disk.write("good", b"content");
        disk.write("bad", b"actual");
        disk.write("extra", b"extra");
        disk.write("tmp/ignored", b"ignored");
        disk.write("sub/tmp/nested", b"extra");
        disk.write(
            MANIFEST_PATH,
            contract["ordered_files"]["bytes"]
                .as_str()
                .unwrap()
                .as_bytes(),
        );
        let Err(PackageVerificationError::Files(issues)) = verify_package_manifest(&disk.0) else {
            panic!("expected three file issue categories");
        };
        let expected = &contract["ordered_files"]["expected"];
        assert_eq!(serde_json::json!(issues.missing), expected[0]);
        assert_eq!(serde_json::json!(issues.corrupt), expected[1]);
        let mut extra = issues.extra;
        extra.sort();
        let mut original_extra: Vec<String> = serde_json::from_value(expected[2].clone()).unwrap();
        original_extra.sort();
        assert_eq!(extra, original_extra);
    }
}
