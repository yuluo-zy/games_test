//! Tagged, length-prefixed chunks over seekable streams.
//!
//! Each chunk contains an eight-byte little-endian tag, an eight-byte payload
//! length, and the payload. Readers borrow payloads without allocating. Writers
//! backfill lengths when a chunk is finished or dropped.

mod reader;
mod tag;
mod writer;

pub use reader::{Chunk, ChunkIterator, Reader};
pub use tag::{AsTag, tag_from_le_bytes_truncated, tag_from_str, tag_from_str_trunc};
pub use writer::{ChunkWriter, Writer};

const TAG_SIZE: usize = 8;
const HEADER_SIZE: usize = 16;

#[cfg(test)]
mod tests;
