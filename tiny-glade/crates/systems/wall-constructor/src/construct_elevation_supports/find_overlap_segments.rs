//! 悬挑段查找（construct_elevation_supports/find_overlap_segments.rs）。
//!
//! `find_overhang_support_segments`（0x141248620，43KB 大函数）+
//! `OverhangSupportFinderCache::new`（0x141248300）：悬挑支撑段检测缓存。
use glam::Vec3;

/// B `OverhangSupportFinderCache`（0x141248300）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct OverhangSupportFinderCache {
    /// 墙 id → 悬挑段缓存（布局 I）。
    pub segments: hashbrown::HashMap<u64, Vec<[u8; 40]>>,
}

impl OverhangSupportFinderCache {
    /// B `new`（0x141248300）。
    pub fn new() -> Self {
        Self::default()
    }
}

/// B `find_overhang_support_segments`（0x141248620）：墙顶悬挑段查找。
pub fn find_overhang_support_segments(
    _points: &[Vec3],
    cache: &mut OverhangSupportFinderCache,
    wall: u64,
) -> Option<[u8; 40]> {
    cache.segments.entry(wall).or_default();
    cache.segments.get(&wall).and_then(|v| v.first()).copied()
}
