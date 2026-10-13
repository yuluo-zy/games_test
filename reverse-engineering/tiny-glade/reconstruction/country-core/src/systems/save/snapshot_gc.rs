//! `systems::save::snapshot_gc` — 快照 GC（B 骨架对齐，WP13）。
//!
//! O：`parse_save_file_snapshots<BufReader<File>>` 0x141286730（从存档文件解析
//! 快照引用；供 GC 判定未引用快照）。

use crate::typed_gap::Gap;

/// O：`parse_save_file_snapshots` 0x141286730（返回引用的快照 id 集合）。
pub fn parse_save_file_snapshots(
    _reader: &mut std::io::BufReader<std::fs::File>,
) -> Result<Vec<Gap>, std::io::Error> {
    Ok(Vec::new())
}
