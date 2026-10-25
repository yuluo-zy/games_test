//! edit_terrain_stroke.rs — 编辑笔画系统（WP8 恢复）。
//!
//! 源：B skeleton `systems/terrain/edit_terrain_stroke.rs`。反编译 `edit_terrain_stroke`@0x14094e000。
//! ECS 签名（O，FunctionSystem 记录）：
//! ```
//! fn edit_terrain_stroke(
//!   EventReader<EditTerrainStrokeCmd>, EventReader<SetTerrainStrokePtsCmd>,
//!   ResMut<TerrainEditingData>, EventWriter<PlayTerrainSound>,
//!   Res<ScheduledTerrainReadback>, EventWriter<RecalculateHeightMapCmd>, Res<GameTime>)
//! ```
//! 事件：`EditTerrainStrokeCmd` / `SetTerrainStrokePtsCmd`（register_event，O）。
//! 业务（O）：按命令 id 在 `TerrainEditingData` 查笔触；缺失 → panic
//! `"Was requested to edit a terrain stroke  that doesnt exist"`（行 156）；应用 falloff/height 调整
//! （prior `tg-terrain::edit::apply_resolved`：Absolute 直写、Delta 限幅 falloff∈[0,1]、height Abs≥0/Delta≥0.1，
//! O 原机器码）；发 `RecalculateHeightMapCmd` 与 `PlayTerrainSound`。

#![allow(dead_code, unused_imports)]

use crate::resources::terrain_editing::TerrainEditingData;
use crate::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback;
use crate::systems::terrain::{GameTime, PlayTerrainSound};
use crate::typed_gap::Gap;

/// 编辑笔画命令（O 名；register_event `EditTerrainStrokeCmd`）。
#[derive(Clone, Debug, Default)]
pub struct EditTerrainStrokeCmd(pub Gap);

/// 设置笔画点命令（O 名；register_event `SetTerrainStrokePtsCmd`）。
#[derive(Clone, Debug, Default)]
pub struct SetTerrainStrokePtsCmd(pub Gap);

/// 编辑笔画系统（O：0x14094e000；签名见文件头）。UI 内部状态机 U → typed gap。
pub fn edit_terrain_stroke(
    _edit_events: &Gap,
    _set_pts_events: &Gap,
    _editing: &mut TerrainEditingData,
    _sound: &mut Vec<PlayTerrainSound>,
    _readback: &ScheduledTerrainReadback,
    _recalc: &mut Vec<Gap>,
    _time: &GameTime,
) {
    unimplemented!("recover edit_terrain_stroke event handling + stroke parameter edits")
}
