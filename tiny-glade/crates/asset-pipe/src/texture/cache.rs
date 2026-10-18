use anyhow::{Context, Result};
use asset::texture::{blob::TextureAssetBlob, desc::TextureAssetDesc};
use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Read, path::PathBuf, time::SystemTime};

pub(super) const PIPELINE_REVISION: u32 = 2;

/// Ordered leaf dependencies, including content hashes so edits that preserve
/// timestamps or file sizes cannot silently reuse an old texture.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextureSourceStamp {
    pub path: PathBuf,
    pub modified: SystemTime,
    pub len: u64,
    pub content_hash: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextureAssetMeta {
    /// Invalidates cached outputs when processing rules change.
    #[serde(default)]
    pub pipeline_revision: u32,
    /// Detects cache corruption even when the archive remains structurally valid.
    #[serde(default)]
    pub output_hash: Option<u64>,
    pub desc: TextureAssetDesc,
    pub src_mtime: SystemTime,
    /// Old metadata has no stamps and is rebuilt once rather than trusted.
    #[serde(default)]
    pub sources: Vec<TextureSourceStamp>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TextureAssetLibraryMeta {
    pub assets: HashMap<RelativePathBuf, TextureAssetMeta>,
}

/// A cache entry must match every source dependency and contain a valid archive.
pub fn cache_is_current(
    desc: &TextureAssetDesc,
    mtime: SystemTime,
    sources: &[TextureSourceStamp],
    previous: Option<&TextureAssetMeta>,
    bytes: &[u8],
) -> bool {
    !sources.is_empty()
        && previous.is_some_and(|p| {
            p.pipeline_revision == PIPELINE_REVISION
                && p.desc == *desc
                && p.src_mtime == mtime
                && p.sources == sources
        })
        && previous
            .and_then(|metadata| metadata.output_hash)
            .is_some_and(|hash| hash == xxhash_rust::xxh3::xxh3_64(bytes))
        && TextureAssetBlob::validate_file(bytes).is_ok()
}

pub(super) fn source_stamps(desc: &TextureAssetDesc) -> Result<Vec<TextureSourceStamp>> {
    desc.source_files()
        .into_iter()
        .map(|path| {
            let mut file =
                std::fs::File::open(path).with_context(|| format!("Opening source {path:?}"))?;
            let metadata = file.metadata()?;
            let mut hash = xxhash_rust::xxh3::Xxh3::new();
            let mut buffer = [0u8; 16 * 1024];
            loop {
                let read = match file.read(&mut buffer) {
                    Ok(read) => read,
                    Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                    Err(error) => {
                        return Err(error).with_context(|| format!("Hashing source {path:?}"));
                    }
                };
                if read == 0 {
                    break;
                }
                hash.update(&buffer[..read]);
            }
            Ok(TextureSourceStamp {
                path: path.to_owned(),
                modified: metadata.modified()?,
                len: metadata.len(),
                content_hash: hash.digest(),
            })
        })
        .collect()
}
