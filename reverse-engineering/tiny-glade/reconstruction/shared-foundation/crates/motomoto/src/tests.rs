use super::*;
#[derive(Debug)]
enum Expected {
    Chunk(u64, Vec<u8>),
    Error(io::ErrorKind),
}
struct Case {
    name: &'static str,
    input: Vec<u8>,
    expected: Vec<Expected>,
    position: u64,
}
#[test]
fn original_reader_instruction_cases() {
    let cases: Vec<Case> = include!("tests/reader_cases.rs");
    assert_eq!(cases.len(), 173);
    for case in cases {
        let reader = Reader::new(&case.input);
        let mut iter = reader.iter();
        for expected in case.expected {
            match expected {
                Expected::Chunk(tag, data) => {
                    let got = iter.next().unwrap().unwrap();
                    assert_eq!((got.tag, got.data), (tag, data.as_slice()), "{}", case.name);
                }
                Expected::Error(kind) => assert_eq!(
                    iter.next().unwrap().unwrap_err().kind(),
                    kind,
                    "{}",
                    case.name
                ),
            }
        }
        assert_eq!(iter.cursor.position(), case.position, "{}", case.name);
        if !case.name.starts_with("length_") {
            assert!(iter.next().is_none(), "{}", case.name);
        }
    }
}
#[test]
fn original_negative_length_slice_panics() {
    let mut data = Vec::new();
    data.write_u64::<LittleEndian>(7).unwrap();
    data.write_u64::<LittleEndian>(u64::MAX).unwrap();
    data.push(1);
    assert!(std::panic::catch_unwind(|| Reader::new(&data).iter().next()).is_err());
}
#[test]
fn tags_pack_utf8_bytes_and_truncate() {
    assert_eq!(7u64.as_tag(), 7);
    assert_eq!(tag_from_str(""), 0);
    assert_eq!(tag_from_str("abcdefgh"), 0x6867666564636261);
    assert_eq!(tag_from_str_trunc("abcdefghijk"), 0x6867666564636261);
    assert_eq!(tag_from_str("é"), 0xa9c3);
    assert_eq!(tag_from_str_trunc("1234567é"), 0xc337363534333231);
    assert!(std::panic::catch_unwind(|| tag_from_str("1234567é")).is_err());
}
#[test]
fn streamed_payload_backfills_and_restores_position() {
    let mut bytes = b"pre".to_vec();
    let mut cursor = Cursor::new(&mut bytes);
    cursor.set_position(3);
    let mut writer = Writer::new(cursor);
    {
        writer.add_chunk("first").unwrap().write_all(b"ab").unwrap();
    }
    {
        writer.add_chunk(7u64).unwrap().write_all(b"c").unwrap();
    }
    let cursor = writer.finish().unwrap();
    assert_eq!(cursor.position(), 3 + 16 + 2 + 16 + 1);
    let mut expected = b"pre".to_vec();
    expected.extend_from_slice(b"first\0\0\0");
    expected.extend_from_slice(&2u64.to_le_bytes());
    expected.extend_from_slice(b"ab");
    expected.extend_from_slice(&7u64.to_le_bytes());
    expected.extend_from_slice(&1u64.to_le_bytes());
    expected.push(b'c');
    assert_eq!(bytes, expected);
    let chunks = Reader::new(&bytes[3..])
        .iter()
        .collect::<io::Result<Vec<_>>>()
        .unwrap();
    assert_eq!(
        chunks,
        vec![
            Chunk {
                tag: tag_from_str("first"),
                data: b"ab"
            },
            Chunk { tag: 7, data: b"c" }
        ]
    );
}
struct FaultStream {
    cursor: Cursor<Vec<u8>>,
    fail_patch: bool,
    max_write: usize,
}
impl Write for FaultStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.fail_patch && self.cursor.position() == 8 {
            self.fail_patch = false;
            return Err(io::ErrorKind::Other.into());
        }
        self.cursor.write(&buf[..buf.len().min(self.max_write)])
    }
    fn flush(&mut self) -> io::Result<()> {
        self.cursor.flush()
    }
}
impl Seek for FaultStream {
    fn seek(&mut self, p: SeekFrom) -> io::Result<u64> {
        self.cursor.seek(p)
    }
}
#[test]
fn byte_exact_header_layout_and_trailing_partial_tail_ignored() {
    // 反编译核对（batch-02）：add_chunk 在 start 写 tag u64 LE、start+8 写 0，
    // Drop 回填 end-start-16；Reader 在 position+16 >= len 时返回 None，
    // 不足 16 字节的尾部被忽略（不含 EOF 错误）。
    let mut bytes = Vec::new();
    let mut writer = Writer::new(Cursor::new(&mut bytes));
    {
        writer.add_chunk("Snap").unwrap().write_all(b"payload").unwrap();
    }
    let cursor = writer.finish().unwrap();
    assert_eq!(cursor.position(), 16 + 7);
    let mut expected = Vec::new();
    expected.extend_from_slice(&tag_from_str("Snap").to_le_bytes());
    expected.extend_from_slice(&7u64.to_le_bytes());
    expected.extend_from_slice(b"payload");
    assert_eq!(bytes, expected);

    // 尾部追加 5 字节残缺数据：Reader 忽略，不报错，位置停在残缺段起点。
    bytes.extend_from_slice(b"tai\0\0");
    let mut iter = Reader::new(&bytes).iter();
    let chunk = iter.next().unwrap().unwrap();
    assert_eq!((chunk.tag, chunk.data), (tag_from_str("Snap"), &b"payload"[..]));
    assert!(iter.next().is_none());
    assert_eq!(iter.cursor.position(), 23);
}
#[test]
fn partial_writes_use_std_write_all() {
    let stream = FaultStream {
        cursor: Cursor::new(vec![]),
        fail_patch: false,
        max_write: 2,
    };
    let mut writer = Writer::new(stream);
    {
        writer
            .add_chunk(9u64)
            .unwrap()
            .write_all(b"payload")
            .unwrap();
    }
    let stream = writer.finish().unwrap();
    assert_eq!(stream.cursor.position(), 23);
    assert_eq!(
        Reader::new(stream.cursor.get_ref())
            .iter()
            .next()
            .unwrap()
            .unwrap()
            .data,
        b"payload"
    );
}
#[test]
fn deferred_drop_failure_is_taken_before_tag_validation() {
    let mut writer = Writer::new(FaultStream {
        cursor: Cursor::new(vec![]),
        fail_patch: false,
        max_write: usize::MAX,
    });
    {
        let mut chunk = writer.add_chunk(9u64).unwrap();
        chunk.write_all(b"x").unwrap();
        chunk.parent.inner.fail_patch = true;
    }
    // A pending patch error precedes an invalid-tag panic and is taken once.
    assert!(writer.add_chunk("longerthan8").is_err());
    assert!(writer.deferred_error.is_none());
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = writer.add_chunk("longerthan8");
        }))
        .is_err()
    );
    writer.add_chunk(3u64).unwrap().write_all(b"y").unwrap();
    assert!(writer.finish().is_ok());
}

#[path = "tests/io_contract.rs"]
mod io_contract;
