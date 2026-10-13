//! systems/garden/record_garden_history.rs — 花园编辑历史记录（WP21）。
//!
//! O：`record_garden_history`（0x1695e10）：鼠标按下时把当前花园编辑记录（
//! `lVar8*0xc` 字节 = 12B 编辑单元 Arc）封装成历史项，`resources::history::History::add`
//!（共享 crate wp13）。`RecordGardenEditToHistoryCmd`（PDB 实体）。

use bevy_ecs::event::Event;

use crate::typed_gap::Gap;

/// `RecordGardenEditToHistoryCmd`（O：命令）。
#[derive(Event, Clone, Copy, Debug, Default, PartialEq)]
pub struct RecordGardenEditToHistoryCmd(pub Gap);

/// 花园编辑单元（O：12B 记录；字段 U）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GardenEditCell(pub [u8; 12]);

/// `record_garden_history`（O 0x1695e10；系统面）。
///
/// `edits` = 本帧花园编辑单元（12B/项，O）；`history` = 历史资源（wp13 域，U）。
pub fn record_garden_history(
    _edits: &[GardenEditCell],
    _history: &mut Gap,
) {
    // O 语义：封装 Arc 编辑单元并 History::add；骨架表面。
}
