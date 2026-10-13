//! `systems::save::save_file_identity` — 存档文件身份识别（B 骨架对齐，WP13）。
//!
//! O：`derive_fallback_save_file_identity` 0x141673140 /
//! `try_approx_parse_save_file_identity` 0x141e58f20（+闭包；近似解析存档身份）。

use crate::typed_gap::Gap;

/// 存档文件身份（布局 U）。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SaveFileIdentity(pub Gap);

/// O：`derive_fallback_save_file_identity` 0x141673140。
pub fn derive_fallback_save_file_identity(_path: &std::path::Path) -> SaveFileIdentity {
    SaveFileIdentity(Gap)
}

/// O：`try_approx_parse_save_file_identity<File>` 0x141e58f20（近似解析）。
pub fn try_approx_parse_save_file_identity(
    _file: &mut std::fs::File,
) -> Result<SaveFileIdentity, anyhow::Error> {
    Ok(SaveFileIdentity(Gap))
}
