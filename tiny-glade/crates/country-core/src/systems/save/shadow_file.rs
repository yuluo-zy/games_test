//! B 0x1409184d0 / 0x1409193d0 / 0x14091a230.
//! Creation eagerly writes a same-directory NamedTempFile and persists it. On
//! failure the full memory image survives, all blocks stay dirty and flush retries.
use crate::resources::save_file::{ShadowBuffer, DIRTY_BLOCK_SIZE};
use anyhow::{Context, Result};
use std::{
    fs::File,
    io::{Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct ShadowFile {
    path: PathBuf,
    directory: PathBuf,
    pub(crate) buffer: ShadowBuffer,
    backend: Option<File>,
    // Native +0x78 is an Option<anyhow::Error>, NOT a flushed-block counter.
    last_error: Option<anyhow::Error>,
}

impl ShadowFile {
    pub fn create(data: Vec<u8>, directory: impl AsRef<Path>, filename: impl AsRef<Path>) -> Self {
        let directory = directory.as_ref().to_path_buf();
        let path = directory.join(filename);
        let mut this = Self {
            path,
            directory,
            buffer: ShadowBuffer {
                data,
                ..Default::default()
            },
            backend: None,
            last_error: None,
        };
        if let Err(error) = this.rewrite() {
            log::error!("Could not write save file: {error:#}");
            this.last_error = Some(error);
            this.buffer
                .dirty
                .mark_dirty(0, this.buffer.data.len() as u64);
        }
        this
    }

    fn rewrite(&mut self) -> Result<()> {
        // Native builder: prefix .tmp, suffix empty, random length 6. This is
        // a random same-directory tempfile, not the literal target + '.tmp'.
        let mut temporary = tempfile::Builder::new()
            .prefix(".tmp")
            .rand_bytes(6)
            .tempfile_in(&self.directory)
            .context("Create temp file")?;
        temporary
            .write_all(&self.buffer.data)
            .context("Write data")?;
        temporary.as_file().sync_all().context("Sync data")?;
        self.backend = Some(
            temporary
                .persist(&self.path)
                .map_err(|error| error.error)
                .context("Persist temp file")?,
        );
        self.buffer.dirty.clear();
        Ok(())
    }

    pub fn data(&self) -> &[u8] {
        &self.buffer.data
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn last_error(&self) -> Option<&anyhow::Error> {
        self.last_error.as_ref()
    }
    pub fn is_open(&self) -> bool {
        self.backend.is_some()
    }
    pub fn dirty_blocks(&self) -> impl Iterator<Item = usize> + '_ {
        self.buffer.dirty.dirty_blocks()
    }

    pub fn write_at(&mut self, offset: u64, bytes: &[u8]) {
        self.buffer.write_at(offset, bytes);
        self.buffer
            .dirty
            .mark_dirty(offset, self.buffer.append_offset);
    }

    pub fn truncate(&mut self, length: u64) {
        assert!(
            length <= self.buffer.data.len() as u64,
            "ShadowFile truncate must not extend the file"
        );
        self.buffer.data.truncate(length as usize);
        self.buffer.append_offset = 0;
        self.buffer.dirty.truncate_for_file_length(length);
    }

    pub fn writer_at(&mut self, offset: u64) -> ShadowFileWriter<'_> {
        self.buffer.append_offset = offset;
        ShadowFileWriter(self)
    }

    fn flush_impl(&mut self) -> Result<()> {
        let Some(file) = &mut self.backend else {
            return self.rewrite();
        };
        // Coalesce adjacent dirty bits into one seek/write, clamp the final run
        // to the current data length. Dirty bits clear only after set_len succeeds.
        let mut blocks = self.buffer.dirty.dirty_blocks().peekable();
        while let Some(start_block) = blocks.next() {
            let mut end_block = start_block + 1;
            while blocks.peek() == Some(&end_block) {
                blocks.next();
                end_block += 1;
            }
            let start = start_block * DIRTY_BLOCK_SIZE as usize;
            let end = (end_block * DIRTY_BLOCK_SIZE as usize).min(self.buffer.data.len());
            file.seek(SeekFrom::Start(start as u64))
                .context("Seek dirty block")?;
            file.write_all(&self.buffer.data[start..end])
                .context("Write dirty data")?;
        }
        drop(blocks);
        file.set_len(self.buffer.data.len() as u64)
            .context("Setting file length")?;
        self.buffer.dirty.clear();
        Ok(())
    }

    pub fn flush(&mut self) -> bool {
        self.last_error = self.flush_impl().err();
        self.last_error.is_none()
    }

    pub fn sync_all(&mut self) -> bool {
        if !self.flush() {
            return false;
        }
        self.last_error = self
            .backend
            .as_ref()
            .expect("ShadowBackendFile is not open")
            .sync_all()
            .context("Asking the OS to sync writes to disk")
            .err();
        self.last_error.is_none()
    }
}

pub struct ShadowFileWriter<'a>(&'a mut ShadowFile);
impl ShadowFileWriter<'_> {
    pub fn write_all(&mut self, bytes: &[u8]) {
        self.0.write_at(self.0.buffer.append_offset, bytes);
    }
    pub fn stream_position(&self) -> u64 {
        self.0.buffer.append_offset
    }
}
