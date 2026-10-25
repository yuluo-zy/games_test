//! B-version chunk protocol recovered from original code; see BUSINESS_RULES.md.
//! Primitive encoding and stream operations reuse byteorder 1.5.0 and std IO.
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Cursor, Seek, SeekFrom, Write};

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
pub fn tag_from_le_bytes_truncated(bytes: &[u8]) -> u64 {
    let mut padded = [0; 8];
    let size = bytes.len().min(8);
    padded[..size].copy_from_slice(&bytes[..size]);
    LittleEndian::read_u64(&padded)
}
pub fn tag_from_str(s: &str) -> u64 {
    assert!(s.len() <= 8, "tag must contain at most eight UTF-8 bytes");
    tag_from_le_bytes_truncated(s.as_bytes())
}
pub fn tag_from_str_trunc(s: &str) -> u64 {
    tag_from_le_bytes_truncated(s.as_bytes())
}

/// Owns the stream and carries failures encountered during a ChunkWriter's drop.
pub struct Writer<W: Write + Seek> {
    inner: W,
    deferred_error: Option<io::Error>,
}
impl<W: Write + Seek> Writer<W> {
    pub fn new(inner: W) -> Self {
        Self {
            inner,
            deferred_error: None,
        }
    }
    pub fn add_chunk<T: AsTag>(&mut self, tag: T) -> io::Result<ChunkWriter<'_, W>> {
        if let Some(error) = self.deferred_error.take() {
            return Err(error);
        }
        let start = self.inner.stream_position()?;
        self.inner.write_u64::<LittleEndian>(tag.as_tag())?;
        self.inner.write_u64::<LittleEndian>(0)?;
        Ok(ChunkWriter {
            parent: self,
            start,
        })
    }
    /// Convenience API for callers which need to surface final drop errors.
    /// This is a reconstruction extension, not a proven original entry point.
    pub fn finish(mut self) -> io::Result<W> {
        if let Some(error) = self.deferred_error.take() {
            return Err(error);
        }
        Ok(self.inner)
    }
}
pub struct ChunkWriter<'a, W: Write + Seek> {
    parent: &'a mut Writer<W>,
    start: u64,
}
impl<W: Write + Seek> Write for ChunkWriter<'_, W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.parent.inner.write(bytes)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.parent.inner.flush()
    }
}
impl<W: Write + Seek> ChunkWriter<'_, W> {
    fn seek_back_and_write_length(&mut self) -> io::Result<()> {
        let end = self.parent.inner.stream_position()?;
        self.parent
            .inner
            .seek(SeekFrom::Start(self.start.wrapping_add(8)))?;
        self.parent
            .inner
            .write_u64::<LittleEndian>(end.wrapping_sub(self.start).wrapping_sub(16))?;
        self.parent.inner.seek(SeekFrom::Start(end))?;
        Ok(())
    }
}
impl<W: Write + Seek> Drop for ChunkWriter<'_, W> {
    fn drop(&mut self) {
        self.parent.deferred_error = self.seek_back_and_write_length().err();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chunk<'a> {
    pub tag: u64,
    pub data: &'a [u8],
}
pub struct Reader<'a> {
    data: &'a [u8],
}
impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }
    pub fn iter(&self) -> ChunkIterator<'a> {
        ChunkIterator {
            cursor: Cursor::new(self.data),
        }
    }
}
pub struct ChunkIterator<'a> {
    cursor: Cursor<&'a [u8]>,
}
impl<'a> ChunkIterator<'a> {
    pub fn try_next(&mut self) -> io::Result<Option<Chunk<'a>>> {
        let start = self.cursor.position().wrapping_add(16);
        // The original stops on equality, including a final empty chunk.
        if start >= self.cursor.get_ref().len() as u64 {
            return Ok(None);
        }
        let tag = self.cursor.read_u64::<LittleEndian>()?;
        let length = self.cursor.read_u64::<LittleEndian>()?;
        let end = self.cursor.seek(SeekFrom::Current(length as i64))?;
        let data = *self.cursor.get_ref();
        if end > data.len() as u64 {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        // A high-bit length which seeks backwards preserves the original panic.
        Ok(Some(Chunk {
            tag,
            data: &data[start as usize..end as usize],
        }))
    }
}
impl<'a> Iterator for ChunkIterator<'a> {
    type Item = io::Result<Chunk<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.try_next().transpose()
    }
}

#[cfg(test)]
mod tests;
