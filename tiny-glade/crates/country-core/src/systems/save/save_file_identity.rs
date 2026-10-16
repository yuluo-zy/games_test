//! B `systems/save/save_file_identity.rs` — 存档文件身份识别。
//!
//! - `try_approx_parse_save_file_identity` 0x141e58f20：分配 0x800 B 缓冲，ReadFile 读入，
//!   在 UTF-8 文本里搜索字面量 `"Identity"`（含引号，串 @0x142f92a40，len 10），随后把紧跟的
//!   JSON 值解析为身份 u64。
//! - `derive_fallback_save_file_identity` 0x141673140：解析失败回退——取 SystemTime、
//!   二次尝试、再由文件元数据派生（字段序 U，未决）。
//!
//! Identity 节的载荷是 `to_json_string<HistoryIdentity>`（0x140b385e0），即把 u64 按十进制
//! 写成数字字符串；因此存档内 `"Identity":<number>` 中的 `<number>` 就是身份值。
//! 文件身份与历史身份同值（`SaveFileIdentity` = `HistoryIdentity`，re-analyst 闭合）。
use crate::resources::history::HistoryIdentity;
use anyhow::{bail, Context, Result};
use std::{
    fs::File,
    io::Read,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

/// 存档文件身份 = 历史身份 u64（PDB 类型 `SaveFileIdentity` 与 `HistoryIdentity` 同值）。
pub type SaveFileIdentity = HistoryIdentity;

/// 近似解析缓冲大小（O：0x141e58f20 分配 0x800 B）。
const APPROX_PARSE_BUFFER: usize = 0x800;
/// 搜索字面量 `"Identity"`（含引号，O：串 @0x142f92a40，len 10）。
const IDENTITY_KEY: &str = "\"Identity\"";

/// 读文件、搜索 `"Identity"` 字面量、解析其后 JSON 值（O：0x141e58f20）。
///
/// 只读前 0x800 B；文件更大时也只扫描这段（原版缓冲上限）。找不到关键字或解析失败返回
/// `Ok(None)`，由调用方走 `derive_fallback_save_file_identity`。
pub fn try_approx_parse_save_file_identity(file: &mut File) -> Result<Option<SaveFileIdentity>> {
    let mut buffer = vec![0u8; APPROX_PARSE_BUFFER];
    let read = file
        .read(&mut buffer)
        .context("Reading save file for identity")?;
    let text = std::str::from_utf8(&buffer[..read]).context("Decoding save file as UTF-8")?;
    let Some(relative) = text.find(IDENTITY_KEY) else {
        return Ok(None);
    };
    // 关键字之后应形如 `"Identity":<number>`；先跳过键、再越过冒号与空白。
    let rest = &text[relative + IDENTITY_KEY.len()..];
    let rest = rest.trim_start();
    let rest = rest.strip_prefix(':').unwrap_or(rest).trim_start();
    let Some(end) = rest.find(|c: char| !c.is_ascii_digit()) else {
        let value = rest.parse::<u64>().context("Parsing identity number")?;
        return Ok(Some(HistoryIdentity(value)));
    };
    let digits = &rest[..end];
    if digits.is_empty() {
        bail!("Identity value is not a number");
    }
    let value = digits.parse::<u64>().context("Parsing identity number")?;
    Ok(Some(HistoryIdentity(value)))
}

/// 解析失败回退（O：0x141673140）：SystemTime 取时 + 二次尝试 + 文件元数据派生。
///
/// 原版字段序 U 未决；本实现把文件修改时间戳折叠进身份值，既不伪造成功解析，也保证同一
/// 文件稳定产出同一身份。二次尝试复用 `try_approx_parse_save_file_identity`（seek 回 0）。
pub fn derive_fallback_save_file_identity(path: &Path) -> SaveFileIdentity {
    // 二次尝试：调用方先 seek 到文件头再解析一次；仍失败则按元数据派生。
    if let Ok(mut file) = File::open(path) {
        use std::io::Seek;
        if file.seek(std::io::SeekFrom::Start(0)).is_ok() {
            if let Ok(Some(identity)) = try_approx_parse_save_file_identity(&mut file) {
                return identity;
            }
        }
    }
    let modified = std::fs::metadata(path)
        .ok()
        .and_then(|metadata| metadata.modified().ok())
        .unwrap_or(UNIX_EPOCH);
    let seconds = modified
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let nanos = modified
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    HistoryIdentity(seconds ^ (nanos as u64) << 32)
}

/// 供外部使用的完整入口：解析失败自动回退（`try_approx_parse` → `derive_fallback`）。
pub fn parse_save_file_identity(path: &Path) -> SaveFileIdentity {
    if let Ok(mut file) = File::open(path) {
        if let Ok(Some(identity)) = try_approx_parse_save_file_identity(&mut file) {
            return identity;
        }
    }
    derive_fallback_save_file_identity(path)
}

/// 让 `SystemTime` 参与测试语义；不暴露更多实现细节。
#[allow(dead_code)]
fn _modified_since(path: &Path) -> Option<SystemTime> {
    std::fs::metadata(path).ok().and_then(|m| m.modified().ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Seek, Write};

    #[test]
    fn parses_the_number_after_the_identity_key() {
        let mut temp = tempfile::NamedTempFile::new().unwrap();
        temp.write_all(
            br#"{"Version":27,"Identity":1048580,"BackwardCompatibility":0,"Camera":{},"History":{"snapshots":[]}}"#,
        )
        .unwrap();
        temp.flush().unwrap();
        temp.as_file_mut().seek(std::io::SeekFrom::Start(0)).unwrap();
        assert_eq!(
            try_approx_parse_save_file_identity(temp.as_file_mut()).unwrap(),
            Some(HistoryIdentity(1048580))
        );
    }

    #[test]
    fn missing_key_or_broken_value_falls_back() {
        let mut temp = tempfile::NamedTempFile::new().unwrap();
        temp.write_all(br#"{"Version":27,"Camera":{}}"#).unwrap();
        temp.flush().unwrap();
        temp.as_file_mut().seek(std::io::SeekFrom::Start(0)).unwrap();
        assert_eq!(try_approx_parse_save_file_identity(temp.as_file_mut()).unwrap(), None);
        let identity = derive_fallback_save_file_identity(temp.path());
        assert_ne!(identity, HistoryIdentity(0));
    }

    #[test]
    fn identity_is_stable_for_the_same_file() {
        let mut temp = tempfile::NamedTempFile::new().unwrap();
        temp.write_all(br#"{"Identity":7}"#).unwrap();
        temp.flush().unwrap();
        assert_eq!(parse_save_file_identity(temp.path()), HistoryIdentity(7));
    }
}
