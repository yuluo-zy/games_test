//! 屋顶历史延迟记录（deferred_record_roof_history.rs）。
//!
//! `roof_record_history`（0x141a37fd0）。候选参数面（skeleton 富化）：
//! `(ResMut<History>, EventReader<RecordRoofHistoryCmd>)`；重建映射
//! RecordRoofHistoryCmd ≡ `country_core::resources::roofs::commands::RecordRoofHistory`
//! （观测载荷 0x98B → 历史记录 tag 2 的 roof 编辑）。
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
