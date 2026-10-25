//! dormer_tiles.rs / commands.rs — 屋顶资源辅助（WP9）。
//!
//! PDB 骨架：`resources/roofs/dormer_tiles.rs`（`DormerTiles` Default）、
//! `resources/roofs/commands.rs`（屋顶命令枚举 + fmt）。

#![allow(dead_code, unused_imports)]

use crate::typed_gap::Gap;

/// 天窗瓦片集合（命名实体，布局 U）。
#[derive(Clone, Debug, Default)]
pub struct DormerTiles(pub Gap);

impl DormerTiles {
    /// impl$1::default。
    pub fn default_value() -> Self {
        Self::default()
    }
}
