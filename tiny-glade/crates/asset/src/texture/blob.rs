//! Versioned texture archive schema. Field order and numeric tags are wire-format contracts.
use anyhow::{Context, Result, ensure};

#[derive(Clone, Debug, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[archive_attr(repr(C), derive(bytecheck::CheckBytes))]
pub struct TextureAssetBlob {
    pub data: Vec<u8>,
    pub dimensions: [u32; 3],
    pub mips: u16,
    pub layers: u16,
    pub format: u8,
    pub kind: u8,
}
pub const TEXTURE_MAGIC: u32 = 0x48502183;
pub const TEXTURE_VERSION: u32 = 8;
const FILE_HEADER_LEN: usize = 8;

impl TextureAssetBlob {
    /// Validate the file header and archive structure without deserializing its pixels.
    /// This does not validate GPU format tags or the expected pixel count.
    pub fn validate_file(bytes: &[u8]) -> Result<()> {
        with_checked_archive(bytes, |_| ())
    }

    /// Validate and decode an owned texture blob from an arbitrarily aligned byte slice.
    pub fn load(bytes: &[u8]) -> Result<Self> {
        with_checked_archive(bytes, |archived| {
            let Ok(blob) = rkyv::Deserialize::deserialize(archived, &mut rkyv::Infallible);
            blob
        })
    }

    pub fn to_file_bytes(&self) -> Result<Vec<u8>> {
        let payload = rkyv::to_bytes::<_, 256>(self)
            .map_err(|e| anyhow::anyhow!("Texture serialization: {e}"))?;
        let mut bytes = Vec::with_capacity(payload.len() + FILE_HEADER_LEN);
        bytes.extend(TEXTURE_MAGIC.to_le_bytes());
        bytes.extend(TEXTURE_VERSION.to_le_bytes());
        bytes.extend_from_slice(&payload);
        Ok(bytes)
    }
}

fn with_checked_archive<T>(
    bytes: &[u8],
    visit: impl FnOnce(&ArchivedTextureAssetBlob) -> T,
) -> Result<T> {
    let header = bytes
        .get(..FILE_HEADER_LEN)
        .context("Texture file is truncated")?;
    ensure!(
        header[..4] == TEXTURE_MAGIC.to_le_bytes(),
        "Texture magic mismatch"
    );
    ensure!(
        header[4..] == TEXTURE_VERSION.to_le_bytes(),
        "Texture pipeline version mismatch"
    );
    let payload = &bytes[FILE_HEADER_LEN..];
    let Some(root_offset) = payload
        .len()
        .checked_sub(size_of::<ArchivedTextureAssetBlob>())
    else {
        return checked_archive(payload).map(visit);
    };
    // The root is stored at the end; Vec<u8> contents need no stronger alignment.
    // Check its actual address because callers may supply an offset slice.
    let root_address = payload.as_ptr().wrapping_add(root_offset);
    if root_address.align_offset(align_of::<ArchivedTextureAssetBlob>()) == 0 {
        checked_archive(payload).map(visit)
    } else {
        let mut aligned = rkyv::AlignedVec::with_capacity(payload.len());
        aligned.extend_from_slice(payload);
        checked_archive(&aligned).map(visit)
    }
}

fn checked_archive(payload: &[u8]) -> Result<&ArchivedTextureAssetBlob> {
    rkyv::check_archived_root::<TextureAssetBlob>(payload)
        .map_err(|error| anyhow::anyhow!("Invalid texture archive: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aligned_roots_are_borrowed_and_offset_slices_roundtrip() {
        let blob = TextureAssetBlob {
            data: vec![1, 2, 3, 4],
            dimensions: [1, 1, 1],
            mips: 1,
            layers: 1,
            format: 37,
            kind: 0,
        };
        let bytes = blob.to_file_bytes().unwrap();
        let alignment = align_of::<ArchivedTextureAssetBlob>();
        for offset in 0..alignment {
            let mut buffer = rkyv::AlignedVec::new();
            buffer.extend_from_slice(&vec![0; offset]);
            buffer.extend_from_slice(&bytes);
            let input = &buffer[offset..];
            let root_address = input
                .as_ptr()
                .wrapping_add(input.len() - size_of::<ArchivedTextureAssetBlob>())
                as usize;
            let checked_address = with_checked_archive(input, |archive| {
                archive as *const ArchivedTextureAssetBlob as usize
            })
            .unwrap();
            assert_eq!(
                checked_address == root_address,
                root_address.is_multiple_of(alignment)
            );
            assert_eq!(
                TextureAssetBlob::load(input)
                    .unwrap()
                    .to_file_bytes()
                    .unwrap(),
                bytes
            );
        }
    }
}
