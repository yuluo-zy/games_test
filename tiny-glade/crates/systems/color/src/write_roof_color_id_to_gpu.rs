//! write_roof_color_id_to_gpu.rs — 屋顶/墙颜色逐项脏写 GPU SSBO（B 0x142298520，
//! O；re-analyst-A 契约 §2）。
//!
//! 语义（O，4 参）：`RoofColorIdSsbo`（0x143136738 键 → 0x1431368cc 55B 串）双
//! 哈希遍历：屋顶（0x40B 条目）逐项 `0x140beefa0(…, 4 字节, offset=idx<<2)`；
//! 墙（0x18B 条目）同——**逐元素脏写**（区别于 set_* 的整表 flush）。

use bevy_ecs::prelude::*;

use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::color_id::{RoofColorIds, WallColorIds};
use country_core::resources::roofs::commands::CreateRoofCmd;

use crate::SetRoofColorIdCmd;

/// 逐元素脏写集合（O：`0x140beefa0(…, 4 字节, offset=idx<<2)` 的 CPU 侧输入；
/// StagingSsbo 上传缓冲区接线属 rhapsody/渲染域）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ColorIdDirtyWrites {
    /// (索引, 颜色) 脏写对（屋顶 + 墙；I：0x40B/0x18B 条目遍历顺序）。
    pub writes: Vec<(usize, u32)>,
}

/// `write_roof_color_id_to_gpu` @0x142298520（O 面：逐元素脏写）。
pub fn write_roof_color_id_to_gpu(
    _library: Res<AssetSsboLibrary>,
    mut roof_cmds: EventReader<SetRoofColorIdCmd>,
    mut creates: EventReader<CreateRoofCmd>,
    roof_colors: Res<RoofColorIds>,
    wall_colors: Res<WallColorIds>,
    mut dirty: ResMut<ColorIdDirtyWrites>,
) {
    let _ = roof_cmds.read().next();
    let _ = creates.read().next();
    dirty.writes.clear();
    // O：逐元素脏写（非默认色；屋顶默认 1、墙默认 0）。
    dirty.writes.extend(roof_colors.iter_nondefault());
    dirty.writes.extend(wall_colors.iter_nondefault());
}
