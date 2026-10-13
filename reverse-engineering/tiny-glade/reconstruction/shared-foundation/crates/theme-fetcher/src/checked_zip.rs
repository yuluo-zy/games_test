//! Original B checked_zip.rs policy, reconstructed from 0x140d921c0/0x140d939a0.
//! B records the complete extracted destination path, including the caller's root.
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Read, Seek},
    path::Path,
};

pub const CHECKSUM_FILE: &str = ".checksum";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FolderChecksumInfo {
    pub files: Vec<(String, String)>,
    pub crc: String,
}

/// Preserve entry order and concatenate stored path bytes then BLAKE3 hex bytes.
/// The field is named crc in the original, although its algorithm is BLAKE3.
pub fn folder_crc(files: &[(String, String)]) -> String {
    let mut hash = blake3::Hasher::new();
    for (path, checksum) in files {
        hash.update(path.as_bytes());
        hash.update(checksum.as_bytes());
    }
    hash.finalize().to_string()
}

/// Extract in archive order, then produce .checksum. Failure leaves preceding
/// extracted files on disk, as in the original; no rollback is claimed.
pub fn extract_with_checksums<R: Read + Seek>(
    mut archive: zip::ZipArchive<R>,
    folder: &Path,
) -> Result<FolderChecksumInfo> {
    let mut files = Vec::new();
    for index in 0..archive.len() {
        let mut file = archive.by_index(index)?;
        let name = file
            .enclosed_name()
            .context("Invalid file path")?
            .to_owned();
        let target = folder.join(name);
        if file.name().ends_with('/') {
            fs::create_dir_all(&target)?;
            continue;
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        fs::write(&target, &bytes)?;
        // Original Path::to_str().unwrap() after the write.
        let path = target
            .to_str()
            .expect("theme checksum path must be UTF-8")
            .to_owned();
        let checksum = blake3::hash(&bytes).to_string();
        files.push((path, checksum));
    }
    let info = FolderChecksumInfo {
        crc: folder_crc(&files),
        files,
    };
    let output = fs::File::create(folder.join(CHECKSUM_FILE)).context("Creating .checksum file")?;
    serde_json::to_writer(output, &info).context("Writing .checksum file")?;
    Ok(info)
}

/// Validate the manifest CRC first, then open precisely its stored paths.
/// Extra files are ignored; this deliberately does not invent a directory walk.
pub fn verify_folder_checksums(folder: &Path) -> Result<FolderChecksumInfo> {
    let bytes = fs::read(folder.join(CHECKSUM_FILE)).context("Reading .checksum file")?;
    let info: FolderChecksumInfo =
        serde_json::from_slice(&bytes).context("Parsing .checksum file")?;
    if folder_crc(&info.files) != info.crc {
        bail!("CRC check failed");
    }
    for (path, checksum) in &info.files {
        let file = fs::File::open(path).with_context(|| format!("Reading {path}"))?;
        // Original maps first, falls back to fs::read on mmap failure.
        // Caller must not concurrently modify a verified cache file.
        let mapped = unsafe { memmap2::MmapOptions::new().map(&file) };
        let hash = match mapped {
            Ok(bytes) => blake3::hash(&bytes),
            Err(_) => blake3::hash(&fs::read(path)?),
        };
        if hash.to_string() != *checksum {
            bail!("Checksum failed for {path}");
        }
    }
    Ok(info)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{Cursor, Write},
        sync::atomic::{AtomicU64, Ordering},
    };
    static NEXT: AtomicU64 = AtomicU64::new(0);
    fn folder() -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "tg-theme-zip-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&path).unwrap();
        path
    }
    fn archive(entries: &[(&str, &[u8])]) -> zip::ZipArchive<Cursor<Vec<u8>>> {
        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
        for (name, bytes) in entries {
            writer
                .start_file(*name, zip::write::FileOptions::default())
                .unwrap();
            writer.write_all(bytes).unwrap();
        }
        zip::ZipArchive::new(writer.finish().unwrap()).unwrap()
    }
    #[test]
    fn original_path_order_and_crc_are_preserved() {
        let root = folder();
        let info =
            extract_with_checksums(archive(&[("b", b"two"), ("nested/a", b"one")]), &root).unwrap();
        assert_eq!(info.files[0].0, root.join("b").to_str().unwrap());
        assert_eq!(verify_folder_checksums(&root).unwrap(), info);
        fs::write(root.join("extra"), b"ignored").unwrap();
        assert!(verify_folder_checksums(&root).is_ok());
        let mut reversed = info.files.clone();
        reversed.reverse();
        assert_ne!(folder_crc(&reversed), info.crc);
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn rejects_content_and_manifest_corruption() {
        let root = folder();
        let mut info = extract_with_checksums(archive(&[("a", b"one")]), &root).unwrap();
        fs::write(root.join("a"), b"changed").unwrap();
        assert!(
            verify_folder_checksums(&root)
                .unwrap_err()
                .to_string()
                .starts_with("Checksum failed")
        );
        info.files[0].1 = "wrong".into();
        fs::write(root.join(CHECKSUM_FILE), serde_json::to_vec(&info).unwrap()).unwrap();
        assert_eq!(
            verify_folder_checksums(&root).unwrap_err().to_string(),
            "CRC check failed"
        );
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn illegal_entry_fails_without_rolling_back_earlier_files() {
        let root = folder();
        assert!(
            extract_with_checksums(archive(&[("valid", b"one"), ("../outside", b"two")]), &root)
                .is_err()
        );
        assert!(root.join("valid").exists());
        assert!(!root.join(CHECKSUM_FILE).exists());
        fs::remove_dir_all(root).unwrap();
    }
}
