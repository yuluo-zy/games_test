//! `material_id.rs` — 延迟光照的材质标签（`core::fmt::impl$74::fmt<MaterialId>` 0x14085da60，41 B）。
//!
//! O：`#[derive(Debug)]` 展开：以枚举判别（u32）索引 0x142a95c70（长度表）与 0x142a95ca0（相对偏移表，6 项）
//! → `Formatter::write_str`。字符串读自 EXE：Solid / Grass / Plant / Canopy / Forward / COUNT。
//! 其中前 4 个与 `deferred_raster` 的 URI 一一对应：`/deferred/{solid,grass,plant,canopy}_light_sun_sky.raster.hlsl`。

/// 材质 id（`#[repr(u32)]`，O：判别值直接作表下标）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u32)]
pub enum MaterialId {
    Solid = 0,
    Grass = 1,
    Plant = 2,
    Canopy = 3,
    Forward = 4,
    /// 计数哨兵（Debug 仍会输出 "COUNT"）。
    COUNT = 5,
}

impl MaterialId {
    /// 带光照的 4 个延迟材质（`deferred_raster` 的 4 个着色器）。
    pub const DEFERRED: [MaterialId; 4] = [
        MaterialId::Solid,
        MaterialId::Grass,
        MaterialId::Plant,
        MaterialId::Canopy,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_names_match_the_exe_string_table() {
        let names = ["Solid", "Grass", "Plant", "Canopy", "Forward", "COUNT"];
        let all = [
            MaterialId::Solid,
            MaterialId::Grass,
            MaterialId::Plant,
            MaterialId::Canopy,
            MaterialId::Forward,
            MaterialId::COUNT,
        ];
        for (i, m) in all.iter().enumerate() {
            assert_eq!(format!("{m:?}"), names[i]);
            assert_eq!(*m as u32, i as u32);
        }
    }
}
