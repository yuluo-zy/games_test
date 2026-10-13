//! render/spatial_resolve_offsets.rs — 空间解析偏移 SSBO（WP21）。
//!
//! O：`SpatialResolveOffsetsSsbo::default_value`（0xb31d10，7179B）：写 2KiB 常量表 =
//! 128 个 (dx,dy) i32 偏移对（0x10 字节步长交织 8B 零填充；首项 {-1,-1}、{2,-1}、
//! {-2,-2}、{-2,2}、{2,2}、{0,-3}…），`+0x0=1`（计数）。TAA 空间解析默认偏移。
//! 全表逐项转录留证据（contract 记录部分首项）；这里给结构 + 模式族说明。

use bevy_ecs::resource::Resource;

/// `SpatialResolveOffsetsSsbo`（O：2KiB 偏移表）。
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct SpatialResolveOffsetsSsbo {
    /// 表（O：2048B = 128×(dx,dy) i32 对，0x10 步长交织 0 填充）。
    pub table: [u8; 2048],
}

impl Default for SpatialResolveOffsetsSsbo {
    fn default() -> Self {
        Self::default_value()
    }
}

impl SpatialResolveOffsetsSsbo {
    /// `default_value`（O 0xb31d10）：偏移对模式族（首项 O；全表转录 U）。
    pub fn default_value() -> Self {
        let mut table = [0u8; 2048];
        // 头字节（O：计数）。
        table[0] = 1;
        // 首几项（O，小端 i32 对）：{-1,-1}、{2,-1}、{-2,-2}、{-2,2}、{2,2}、{0,-3}…
        // 全表为证据内 0x10 步长交织模式；逐项转录留 U。
        let put = |table: &mut [u8], slot: usize, a: i32, b: i32| {
            let o = 0x14 + slot * 0x10;
            table[o..o + 4].copy_from_slice(&a.to_le_bytes());
            table[o + 4..o + 8].copy_from_slice(&b.to_le_bytes());
        };
        put(&mut table, 0, -1, -1);
        put(&mut table, 1, 2, -1);
        put(&mut table, 2, -2, -2);
        put(&mut table, 3, -2, 2);
        put(&mut table, 4, 2, 2);
        put(&mut table, 5, 0, -3);
        Self { table }
    }
}
