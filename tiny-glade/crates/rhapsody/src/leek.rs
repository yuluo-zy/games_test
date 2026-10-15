//! `leek` 的 16 字节定长名字（原 `leek::impl$1::from` 0x140be2880 / `impl$0::fmt` 0x140be2710 / `impl$2::fmt` 0x140be2990）。
//!
//! 机器码证实的 `From<&str>`：对 UTF-8 逐码点解码（`is_ascii`/`next_code_point`），**码点 > 0x7f 的被跳过**
//! （不是报错），ASCII 字节依次写入 `[u8; 16]`，写满 16 个（下标 15 写入后 `je` 退出）或字符串结束即停，
//! 其余字节保持 0。`dispose_cb_resource` 里的同一循环被内联（见 `raw/disposal.rs`、`log_ring.rs` 的 16B 名字字段）。
//! `impl$0::fmt` 先 `String::new` 再逐字节 push、最后按 str 的 Debug 输出（带引号）——这里实现为取到首个 0 的前缀；
//! 对 `impl$2::fmt`（Display）仅知为 `Formatter::write_fmt`，输出形态未证实，按同一前缀输出。
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Name(pub [u8; 16]);

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        let mut bytes = [0u8; 16];
        for (slot, byte) in bytes.iter_mut().zip(value.bytes().filter(u8::is_ascii)) {
            *slot = byte;
        }
        Self(bytes)
    }
}
impl Name {
    /// 首个 0 之前的字节（全 ASCII，必为合法 UTF-8）。
    pub fn as_str(&self) -> &str {
        let end = self.0.iter().position(|&b| b == 0).unwrap_or(16);
        std::str::from_utf8(&self.0[..end]).unwrap_or("")
    }
}
impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}
impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_non_ascii_and_truncates_to_16_bytes() {
        assert_eq!(Name::from("abc").0[..4], *b"abc\0");
        assert_eq!(Name::from("a\u{4e2d}b").as_str(), "ab");
        let long = Name::from("0123456789abcdefXYZ");
        assert_eq!(long.0, *b"0123456789abcdef");
        assert_eq!(format!("{:?}", Name::from("hi")), "\"hi\"");
    }
}
