use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{self, Seek, SeekFrom, Write};

use crate::{AsTag, HEADER_SIZE, TAG_SIZE};

/// Owns a seekable output stream. I/O failure permanently invalidates the writer;
/// callers must discard that output. Interrupted payload writes remain retryable.
/// The stream must support overwriting bytes at a sought position; append-only
/// files are detected when a chunk is completed and rejected.
pub struct Writer<W: Write + Seek> {
    inner: W,
    failure: Option<io::Error>,
    chunk_open: bool,
}

impl<W: Write + Seek> Writer<W> {
    pub fn new(inner: W) -> Self {
        Self {
            inner,
            failure: None,
            chunk_open: false,
        }
    }

    /// Starts a chunk at the current position without truncating the stream.
    ///
    /// String tags longer than eight UTF-8 bytes panic before writing any bytes.
    pub fn add_chunk<T: AsTag>(&mut self, tag: T) -> io::Result<ChunkWriter<'_, W>> {
        self.check_ready()?;
        let tag = tag.as_tag();
        let result = (|| {
            let start = self.inner.stream_position()?;
            let payload_start = start.checked_add(HEADER_SIZE as u64).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "chunk header position overflow",
                )
            })?;
            self.inner.write_u64::<LittleEndian>(tag)?;
            self.inner.write_u64::<LittleEndian>(0)?;
            Ok(payload_start)
        })();
        let payload_start = self.record_result(result)?;
        self.chunk_open = true;
        Ok(ChunkWriter {
            parent: self,
            payload_start,
            completed: false,
        })
    }

    /// Returns the stream, or the first recorded failure.
    ///
    /// Returns an error if a chunk was leaked without completing it.
    /// This does not flush buffered output. Flush the returned stream explicitly
    /// to observe failures from a `BufWriter` or another buffered sink.
    pub fn finish(self) -> io::Result<W> {
        match self.failure {
            Some(error) => Err(error),
            None if self.chunk_open => Err(unfinished_chunk()),
            None => Ok(self.inner),
        }
    }

    fn check_ready(&self) -> io::Result<()> {
        self.check_health()?;
        if self.chunk_open {
            return Err(unfinished_chunk());
        }
        Ok(())
    }

    fn check_health(&self) -> io::Result<()> {
        match &self.failure {
            Some(error) => Err(io::Error::new(error.kind(), error.to_string())),
            None => Ok(()),
        }
    }

    fn record_result<T>(&mut self, result: io::Result<T>) -> io::Result<T> {
        if let Err(error) = &result
            && self.failure.is_none()
        {
            self.failure = Some(io::Error::new(error.kind(), error.to_string()));
        }
        result
    }
}

fn unfinished_chunk() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        "previous chunk was not completed",
    )
}

/// A single streaming payload. Prefer [`Self::finish`] to report backfill errors
/// directly; dropping the chunk performs the same backfill as a compatibility fallback.
#[must_use = "finish the chunk to observe errors when backfilling its length"]
pub struct ChunkWriter<'a, W: Write + Seek> {
    parent: &'a mut Writer<W>,
    payload_start: u64,
    completed: bool,
}

impl<W: Write + Seek> ChunkWriter<'_, W> {
    pub fn finish(mut self) -> io::Result<()> {
        self.complete()
    }

    fn complete(&mut self) -> io::Result<()> {
        // Never retry a failed backfill from Drop: the cursor may be in the header.
        self.completed = true;
        self.parent.check_health()?;
        let result = (|| {
            let end = self.parent.inner.stream_position()?;
            let length = end.checked_sub(self.payload_start).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "stream position precedes chunk payload",
                )
            })?;
            // payload_start was checked before writing the header.
            self.parent
                .inner
                .seek(SeekFrom::Start(self.payload_start - TAG_SIZE as u64))?;
            self.parent.inner.write_u64::<LittleEndian>(length)?;
            // A seekable file may still use append-only writes. Check the actual
            // patched position before restoring it, so this cannot report success
            // after appending the length. Unlike stream_position(), seek flushes a
            // BufWriter and therefore observes where the patch really landed.
            #[allow(clippy::seek_from_current)]
            let patched_end = self.parent.inner.seek(SeekFrom::Current(0))?;
            if patched_end != self.payload_start {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "stream did not overwrite the chunk length; append-only output is unsupported",
                ));
            }
            self.parent.inner.seek(SeekFrom::Start(end))?;
            Ok(())
        })();
        self.parent.record_result(result)?;
        self.parent.chunk_open = false;
        Ok(())
    }
}

impl<W: Write + Seek> Write for ChunkWriter<'_, W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.parent.check_health()?;
        let result = self.parent.inner.write(bytes);
        // Interrupted writes remain retryable under std::io::Write's contract.
        if matches!(&result, Err(error) if error.kind() == io::ErrorKind::Interrupted) {
            return result;
        }
        self.parent.record_result(result)
    }

    fn write_all(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.parent.check_health()?;
        // Delegating also records WriteZero, which the default implementation
        // would generate outside write() after the sink returns Ok(0).
        let result = self.parent.inner.write_all(bytes);
        self.parent.record_result(result)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.parent.check_health()?;
        let result = self.parent.inner.flush();
        self.parent.record_result(result)
    }
}

impl<W: Write + Seek> Drop for ChunkWriter<'_, W> {
    fn drop(&mut self) {
        if !self.completed {
            let _ = self.complete();
        }
    }
}

#[cfg(test)]
#[path = "tests/io_contract.rs"]
mod tests;
