//! 悬挑段查找（construct_elevation_supports/find_overlap_segments.rs）。
//!
//! `find_overhang_support_segments`（0x141248620，43KB 大函数）+
//! `OverhangSupportFinderCache::new`（0x141248300）：悬挑支撑段检测缓存。
//!
//! 2026-10-17 复核：旧实现（只取缓存首项）为**近似体**，已移除。
//! fresh（`evidence/elevsys-0x1407867d0.c` 等）已确认：
//! `OverhangSupportFinderCache` 为两 Vec 装配（字段布局 U）；
//! `find_overhang_support_segments` 的输出结构/阈值/多 closure 未定。
use glam::Vec3;

/// B `OverhangSupportFinderCache`（0x141248300）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct OverhangSupportFinderCache {
    /// 墙 id → 悬挑段缓存（布局 U；`new` 为两 Vec 装配）。
    pub segments: hashbrown::HashMap<u64, Vec<[u8; 40]>>,
}

impl OverhangSupportFinderCache {
    /// B `new`（0x141248300）：两 Vec 装配（字段语义 U）。
    pub fn new() -> Self {
        Self::default()
    }
}

/// B `find_overhang_support_segments`（0x141248620）。**未实现**：
/// 输出结构/阈值/多 closure（43KB）未逐语句。
pub fn find_overhang_support_segments(
    _points: &[Vec3],
    _cache: &mut OverhangSupportFinderCache,
    _wall: u64,
) -> Option<[u8; 40]> {
    todo!(
        "未实现: find_overhang_support_segments 0x141248620（43KB：输出结构/阈值/多 closure 未逐语句）"
    )
}
