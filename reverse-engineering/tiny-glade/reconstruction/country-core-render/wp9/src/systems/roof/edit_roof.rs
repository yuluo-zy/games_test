//! edit_roof.rs — edit_roof 系统（WP9）。
//!
//! PDB 骨架 `systems/roof/src/edit_roof.rs`（closure$0）。机器码 0x1421ab7c0。
//! 数值核心 = `reconstruction/roof/edit_rules.rs`（64B RoofEditEvent / apply_edit，
//! 4514 原指令机器码验证）——本文件把该核心接入 ECS 事件流。
//!
//! 事件结构（O，64B RoofEditEvent，edit_rules.rs `from_observed`）：
//!   profile{mode,value}@0、height{...}@8、ridge{...}@16、eave{...}@24、
//!   wall_id u64@32、tip{mode,value2}@40、source_flag u8@52。
//! 反馈事件 tag（O）：2=Scalar、3=Tip、10=GableEnter、11=GableLeave（edit_rules.rs
//! `EditFeedback`）。

#![allow(dead_code, unused_imports)]

use crate::resources::roofs::roof_shape::Roof;
use crate::systems::roof::edit_rules::{apply_edit, RoofEditEvent};
use crate::typed_gap::Gap;

/// edit_roof（O：0x1421ab7c0）。
///
/// 每屋顶编辑命令 → `apply_edit`（profile/height/ridge/eave/tip 的 Absolute=1/Delta=2
/// 语义、clamp、change_tick 写、gable 进出反馈）；按 `Roof::is_gable`/`pivot_3d` 发历史/
/// 装饰事件。深 ECS 事件写入 = typed gap（U）。
pub fn edit_roof(_events: Gap) {
    let _ = Gap;
}

/// 单屋顶编辑入口（O：0x1421ab7c0 核心；承接 RoofEditEvent 字节）。
pub fn apply_edit_event(roof: &mut Roof, event: &[u8; 64], tick: &mut u32, current: u32) {
    let parsed = RoofEditEvent::from_observed(event);
    apply_edit(&mut roof.0, &parsed, tick, current);
}
