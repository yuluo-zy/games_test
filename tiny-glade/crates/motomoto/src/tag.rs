use byteorder::{ByteOrder, LittleEndian};

use crate::TAG_SIZE;

/// A numeric tag or a string containing at most eight UTF-8 bytes.
pub trait AsTag {
    fn as_tag(&self) -> u64;
}

impl AsTag for u64 {
    fn as_tag(&self) -> u64 {
        *self
    }
}

impl AsTag for &str {
    fn as_tag(&self) -> u64 {
        tag_from_str(self)
    }
}

/// Packs up to eight bytes in little-endian order, padding shorter inputs with zeros.
pub fn tag_from_le_bytes_truncated(bytes: &[u8]) -> u64 {
    let mut padded = [0; TAG_SIZE];
    let size = bytes.len().min(TAG_SIZE);
    padded[..size].copy_from_slice(&bytes[..size]);
    LittleEndian::read_u64(&padded)
}

/// Packs a string's UTF-8 bytes as a tag.
///
/// # Panics
///
/// Panics if the string contains more than eight UTF-8 bytes.
pub fn tag_from_str(tag: &str) -> u64 {
    assert!(
        tag.len() <= TAG_SIZE,
        "tag must contain at most eight UTF-8 bytes"
    );
    tag_from_le_bytes_truncated(tag.as_bytes())
}

/// Packs the first eight UTF-8 bytes, even if truncation splits a code point.
pub fn tag_from_str_trunc(tag: &str) -> u64 {
    tag_from_le_bytes_truncated(tag.as_bytes())
}
