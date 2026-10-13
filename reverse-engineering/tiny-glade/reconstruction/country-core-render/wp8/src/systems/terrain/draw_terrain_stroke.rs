//! draw_terrain_stroke.rs — 绘制笔画系统（WP8 恢复）。
//!
//! 源：B skeleton `systems/terrain/draw_terrain_stroke.rs`。反编译 `terrain_stroke_changed`@0x140a03ba0（行 17）。
//! ECS 签名（O）：
//! ```
//! fn terrain_stroke_changed(
//!   EventReader<EvTerrainStrokeChanged>, Res<ScheduledTerrainReadback>,
//!   EventWriter<RecalculateHeightMapCmd>, EventWriter<PlayTerrainSound>)
//! ```
//! 业务（O）：有 `EvTerrainStrokeChanged` 事件（EventReader 差分非零）时发 `RecalculateHeightMapCmd`
//! + `PlayTerrainSound`；否则不动。

#![allow(dead_code, unused_imports)]

use crate::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback;
use crate::systems::terrain::PlayTerrainSound;
use crate::typed_gap::Gap;

/// 笔画已变事件（O 名；register_event `EvTerrainStrokeChanged`）。
#[derive(Clone, Debug, Default)]
pub struct EvTerrainStrokeChanged(pub Gap);

/// 笔画已变系统（O：`terrain_stroke_changed`@0x140a03ba0，行 17）。
pub fn terrain_stroke_changed(
    _changed: &Gap,
    _readback: &ScheduledTerrainReadback,
    _recalc: &mut Vec<Gap>,
    _sound: &mut Vec<PlayTerrainSound>,
) {
    unimplemented!("recover terrain_stroke_changed event forwarding")
}
