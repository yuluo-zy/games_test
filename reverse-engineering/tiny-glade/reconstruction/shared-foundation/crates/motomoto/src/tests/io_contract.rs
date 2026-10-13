use super::*;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_FILE: AtomicU64 = AtomicU64::new(0);
struct ScratchFile(PathBuf);
impl ScratchFile {
    fn new() -> (Self, File) {
        let name = format!(
            "motomoto-io-{}-{}.bin",
            std::process::id(),
            NEXT_FILE.fetch_add(1, Ordering::Relaxed)
        );
        let path = std::env::temp_dir().join(name);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        (Self(path), file)
    }
}
impl Drop for ScratchFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
fn disk_roundtrip<W: Write + Seek>(mut stream: W, scratch: &ScratchFile) {
    stream.write_all(b"prefix").unwrap();
    let mut writer = Writer::new(stream);
    writer.add_chunk("empty").unwrap();
    {
        let mut chunk = writer.add_chunk("data").unwrap();
        chunk.write_all(&vec![0x5a; 131_073]).unwrap();
        chunk.flush().unwrap();
        chunk.write_all(b"tail").unwrap();
    }
    writer.add_chunk(99u64).unwrap().write_all(b"end").unwrap();
    writer.add_chunk("lastzero").unwrap();
    let mut stream = writer.finish().unwrap();
    let end = stream.stream_position().unwrap();
    stream.flush().unwrap();
    drop(stream);
    let mut data = Vec::new();
    File::open(&scratch.0)
        .unwrap()
        .read_to_end(&mut data)
        .unwrap();
    assert_eq!(end, data.len() as u64);
    assert_eq!(&data[..6], b"prefix");
    let chunks = Reader::new(&data[6..])
        .iter()
        .collect::<io::Result<Vec<_>>>()
        .unwrap();
    assert_eq!(chunks.len(), 3); // 原规则忽略末尾恰好 16 字节的空段。
    assert_eq!(
        chunks[0],
        Chunk {
            tag: tag_from_str("empty"),
            data: b""
        }
    );
    assert_eq!(chunks[1].data.len(), 131_077);
    assert!(chunks[1].data[..131_073].iter().all(|v| *v == 0x5a));
    assert_eq!(&chunks[1].data[131_073..], b"tail");
    assert_eq!(
        chunks[2],
        Chunk {
            tag: 99,
            data: b"end"
        }
    );
    for chunk in chunks {
        assert!(chunk.data.as_ptr() >= data.as_ptr());
    }
}
#[test]
fn actual_file_roundtrip_nonzero_start() {
    let (scratch, file) = ScratchFile::new();
    disk_roundtrip(file, &scratch);
}
#[test]
fn actual_bufwriter_roundtrip_nonzero_start() {
    let (scratch, file) = ScratchFile::new();
    disk_roundtrip(BufWriter::with_capacity(31, file), &scratch);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operation {
    Position,
    Tag,
    Length,
    Payload,
    PatchSeek,
    PatchWrite,
    Restore,
    Flush,
}
struct FaultIo {
    cursor: Cursor<Vec<u8>>,
    fault: Option<Operation>,
    patching: bool,
    writes: usize,
    log: Vec<Operation>,
}
impl FaultIo {
    fn new() -> Self {
        Self {
            cursor: Cursor::new(Vec::new()),
            fault: None,
            patching: false,
            writes: 0,
            log: Vec::new(),
        }
    }
    fn visit(&mut self, op: Operation) -> io::Result<()> {
        self.log.push(op);
        if self.fault == Some(op) {
            self.fault = None;
            return Err(io::Error::other(format!("injected {op:?}")));
        }
        Ok(())
    }
}
impl Write for FaultIo {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        let op = if self.patching {
            Operation::PatchWrite
        } else {
            match self.writes {
                0 => Operation::Tag,
                1 => Operation::Length,
                _ => Operation::Payload,
            }
        };
        self.visit(op)?;
        self.writes += 1;
        self.cursor.write(data)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.visit(Operation::Flush)
    }
}
impl Seek for FaultIo {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let op = match pos {
            SeekFrom::Current(0) => Operation::Position,
            SeekFrom::Start(8) => Operation::PatchSeek,
            _ => Operation::Restore,
        };
        self.visit(op)?;
        self.patching = op == Operation::PatchSeek;
        self.cursor.seek(pos)
    }
}
#[test]
fn header_and_payload_failures_are_not_silently_successful() {
    for fault in [Operation::Position, Operation::Tag, Operation::Length] {
        let mut stream = FaultIo::new();
        stream.fault = Some(fault);
        let mut writer = Writer::new(stream);
        assert!(writer.add_chunk(7u64).is_err(), "{fault:?}");
        assert!(writer.deferred_error.is_none());
        assert_eq!(writer.inner.log.last(), Some(&fault));
    }
    let mut writer = Writer::new(FaultIo::new());
    {
        let mut chunk = writer.add_chunk(7u64).unwrap();
        chunk.write_all(b"ok").unwrap();
        chunk.parent.inner.fault = Some(Operation::Payload);
        assert!(chunk.write_all(b"failed").is_err());
    }
    // payload 错误由调用者处理；Drop 只回填已经成功写入的字节。
    let stream = writer.finish().unwrap();
    assert_eq!(
        Reader::new(stream.cursor.get_ref())
            .iter()
            .next()
            .unwrap()
            .unwrap()
            .data,
        b"ok"
    );
}
#[test]
fn all_drop_failures_surface_at_finish_without_later_io() {
    for fault in [
        Operation::Position,
        Operation::PatchSeek,
        Operation::PatchWrite,
        Operation::Restore,
    ] {
        let mut writer = Writer::new(FaultIo::new());
        {
            let mut chunk = writer.add_chunk(7u64).unwrap();
            chunk.write_all(b"payload").unwrap();
            chunk.parent.inner.fault = Some(fault);
        }
        assert_eq!(writer.inner.log.last(), Some(&fault));
        assert!(writer.deferred_error.is_some());
        assert!(writer.finish().is_err(), "{fault:?}");
    }
}
#[test]
fn flush_delegates_failure_and_does_not_finalize_chunk() {
    let mut writer = Writer::new(FaultIo::new());
    {
        let mut chunk = writer.add_chunk(7u64).unwrap();
        chunk.write_all(b"x").unwrap();
        chunk.parent.inner.fault = Some(Operation::Flush);
        assert!(chunk.flush().is_err());
        assert!(chunk.parent.deferred_error.is_none());
        assert_eq!(&chunk.parent.inner.cursor.get_ref()[8..16], &[0; 8]);
        chunk.flush().unwrap();
        chunk.write_all(b"y").unwrap();
    }
    let stream = writer.finish().unwrap();
    assert_eq!(
        Reader::new(stream.cursor.get_ref())
            .iter()
            .next()
            .unwrap()
            .unwrap()
            .data,
        b"xy"
    );
}
