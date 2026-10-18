use super::*;
use std::io::{self, Cursor, Seek, SeekFrom, Write};

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
fn streamed_chunks_match_wire_format_and_borrow_payloads() {
    let mut bytes = b"pre".to_vec();
    let mut cursor = Cursor::new(&mut bytes);
    cursor.set_position(3);
    let mut writer = Writer::new(cursor);
    let mut chunk = writer.add_chunk("first").unwrap();
    chunk.write_all(b"ab").unwrap();
    chunk.finish().unwrap();
    // Exercise the Drop fallback and consecutive empty chunks, including EOF.
    drop(writer.add_chunk(7u64).unwrap());
    drop(writer.add_chunk(8u64).unwrap());
    assert_eq!(writer.finish().unwrap().position(), 3 + 18 + 16 + 16);

    let mut expected = b"prefirst\0\0\0".to_vec();
    expected.extend_from_slice(&2u64.to_le_bytes());
    expected.extend_from_slice(b"ab");
    for tag in [7u64, 8] {
        expected.extend_from_slice(&tag.to_le_bytes());
        expected.extend_from_slice(&0u64.to_le_bytes());
    }
    assert_eq!(bytes, expected);
    let chunks = Reader::new(&bytes[3..])
        .iter()
        .collect::<io::Result<Vec<_>>>()
        .unwrap();
    assert_eq!(
        chunks,
        [
            Chunk {
                tag: tag_from_str("first"),
                data: b"ab"
            },
            Chunk { tag: 7, data: b"" },
            Chunk { tag: 8, data: b"" }
        ]
    );
    assert_eq!(chunks[0].data.as_ptr(), bytes[19..].as_ptr());
}

#[test]
fn every_truncation_and_extreme_length_returns_one_terminal_error() {
    let mut encoded = Vec::new();
    encoded.extend_from_slice(&7u64.to_le_bytes());
    encoded.extend_from_slice(&3u64.to_le_bytes());
    encoded.extend_from_slice(b"abc");
    assert!(Reader::new(&[]).iter().try_next().unwrap().is_none());
    for length in 1..encoded.len() {
        let mut iter = Reader::new(&encoded[..length]).iter();
        assert_eq!(
            iter.try_next().unwrap_err().kind(),
            io::ErrorKind::UnexpectedEof,
            "{length}"
        );
        assert!(iter.next().is_none());
        assert!(iter.try_next().unwrap().is_none());
    }
    for length in [4, i64::MAX as u64, 1u64 << 63, u64::MAX - 16, u64::MAX] {
        encoded[8..16].copy_from_slice(&length.to_le_bytes());
        let mut iter = Reader::new(&encoded).iter();
        assert_eq!(
            iter.next().unwrap().unwrap_err().kind(),
            io::ErrorKind::UnexpectedEof
        );
        assert!(iter.next().is_none());
    }
    encoded[8..16].copy_from_slice(&3u64.to_le_bytes());
    for trailing in 1..16 {
        let mut data = encoded.clone();
        data.resize(data.len() + trailing, 0);
        let mut iter = Reader::new(&data).iter();
        assert_eq!(iter.next().unwrap().unwrap().data, b"abc");
        assert_eq!(
            iter.next().unwrap().unwrap_err().kind(),
            io::ErrorKind::UnexpectedEof
        );
        assert!(iter.next().is_none());
    }
}

struct ShortWriter(Cursor<Vec<u8>>);
impl Write for ShortWriter {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.0.write(&data[..data.len().min(2)])
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
impl Seek for ShortWriter {
    fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
        self.0.seek(position)
    }
}

#[test]
fn partial_writes_preserve_complete_header_and_payload() {
    let mut writer = Writer::new(ShortWriter(Cursor::new(Vec::new())));
    let mut chunk = writer.add_chunk(9u64).unwrap();
    chunk.write_all(b"payload").unwrap();
    chunk.finish().unwrap();
    let stream = writer.finish().unwrap();
    assert_eq!(stream.0.position(), 23);
    assert_eq!(
        Reader::new(stream.0.get_ref())
            .iter()
            .next()
            .unwrap()
            .unwrap(),
        Chunk {
            tag: 9,
            data: b"payload"
        }
    );
}
