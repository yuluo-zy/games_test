//! `systems::save::shadow_file` — 影子文件原子写（B 骨架对齐，WP13）。
//!
//! O：`ShadowFile::create` 0x1409184d0、`flush` 0x1409193d0、`sync_all`
//! 0x14091a230、`mark_dirty` 0x14091a450、`write_at` / `truncate` / `path` /
//! `data` / `flush_impl`；`ShadowFileWriter`（write_all / stream_position）；
//! `ShadowBackendFile::unwrap_open_mut`。精确文件协议 U。

use crate::typed_gap::Gap;

/// O：`ShadowFile`（影子文件；写目录下临时文件后 rename/sync）。
#[derive(Clone, Debug, Default)]
pub struct ShadowFile {
    pub dirty: bool,
    /// 后端/路径。布局 U。
    pub rest: Gap,
}

impl ShadowFile {
    /// O：`create` 0x1409184d0。
    pub fn create(_path: &std::path::Path) -> Result<Self, std::io::Error> {
        Ok(Self {
            dirty: false,
            rest: Gap,
        })
    }
    /// O：`flush` 0x1409193d0。
    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        self.dirty = false;
        Ok(())
    }
    /// O：`sync_all` 0x14091a230。
    pub fn sync_all(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
    /// O：`mark_dirty` 0x14091a450。
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }
    /// O：`path` / `data`。
    pub fn path(&self) -> &std::path::Path {
        std::path::Path::new("")
    }
    pub fn data(&self) -> &Gap {
        &self.rest
    }
    /// O：`write_at` / `truncate` / `flush_impl`。
    pub fn write_at(&mut self, _offset: u64, _bytes: &[u8]) -> Result<(), std::io::Error> {
        Ok(())
    }
    pub fn truncate(&mut self, _len: u64) -> Result<(), std::io::Error> {
        Ok(())
    }
}

/// O：`ShadowFileWriter`（Write impl；write_all / stream_position）。
pub struct ShadowFileWriter<'a>(pub &'a mut ShadowFile);

impl std::io::Write for ShadowFileWriter<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

/// O：`ShadowBackendFile::unwrap_open_mut`（后端文件可变解包）。
pub struct ShadowBackendFile(pub Gap);
impl ShadowBackendFile {
    pub fn unwrap_open_mut(&mut self) -> &mut Gap {
        &mut self.0
    }
}
