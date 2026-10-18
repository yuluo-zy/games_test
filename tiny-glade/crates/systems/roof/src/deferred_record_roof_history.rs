//! 屋顶历史延迟记录（deferred_record_roof_history.rs）。
//!
//! `roof_record_history`（0x141a37fd0，401B/103）。参数面（mangled 名 O）：
//! `void(ResMut<History>, EventReader<RecordRoofHistoryCmd>)`。
//!
//! 判等（fresh decomp-0x141a37fd0）：逐 `RecordRoofHistoryCmd` 事件（stride 0x98，
//! RoofEdit 载荷 @+0x10 起）构造 RoofEdit 历史条目（Arc{强=1,弱=1, tag=2
//! VisualEdit@+0x10, 字段 @+0x14/+0x18/+0x1c/+0x20/+0x24/+0x28..+0x30}），
//! `History::add()`（失败不 panic）。本实现委托 `EditType::from(RoofHistoryEdit)`
//! → `History::add`；`RoofHistoryEdit([u8;64])` 即观测 RoofEdit 载荷
//! （tag@[0]、字段@[4..]），转换与机器码写一致（发射方构造载荷为准）。
use country_core::resources::{history::History, roofs::commands::RecordRoofHistory};
use bevy_ecs::prelude::*;

/// B 0x141a37fd0：把待提交的屋顶变更写入历史事务。
pub fn roof_record_history(
    mut history: ResMut<History>,
    mut events: EventReader<RecordRoofHistory>,
) {
    for event in events.read() {
        let edit = country_core::resources::history::EditType::from(event.0.clone());
        let _ = history.add(edit);
    }
}