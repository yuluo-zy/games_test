use byteorder::{ByteOrder, LittleEndian};
use std::{io, iter::FusedIterator};

use crate::{HEADER_SIZE, TAG_SIZE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chunk<'a> {
    pub tag: u64,
    pub data: &'a [u8],
}

/// An immutable chunk container. Each iterator traverses it independently.
pub struct Reader<'a> {
    data: &'a [u8],
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn iter(&self) -> ChunkIterator<'a> {
        ChunkIterator {
            remaining: self.data,
        }
    }
}

/// Borrows payloads and stops permanently after EOF or the first malformed chunk.
pub struct ChunkIterator<'a> {
    remaining: &'a [u8],
}

impl<'a> ChunkIterator<'a> {
    /// Returns `UnexpectedEof` for a truncated header or payload, including an
    /// encoded length larger than the addressable input. The error is emitted once.
    pub fn try_next(&mut self) -> io::Result<Option<Chunk<'a>>> {
        // Taking the input first makes every error terminal without a second flag.
        let remaining = std::mem::take(&mut self.remaining);
        if remaining.is_empty() {
            return Ok(None);
        }
        if remaining.len() < HEADER_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "truncated chunk header",
            ));
        }

        let (header, payload) = remaining.split_at(HEADER_SIZE);
        let length = usize::try_from(LittleEndian::read_u64(&header[TAG_SIZE..]))
            .ok()
            .filter(|&length| length <= payload.len())
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::UnexpectedEof, "truncated chunk payload")
            })?;
        let (data, tail) = payload.split_at(length);
        self.remaining = tail;
        Ok(Some(Chunk {
            tag: LittleEndian::read_u64(&header[..TAG_SIZE]),
            data,
        }))
    }
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = io::Result<Chunk<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next().transpose()
    }
}

impl FusedIterator for ChunkIterator<'_> {}
