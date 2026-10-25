//! history/roof_edit.rs — RoofEdit / RoofReplayCmd 历史（WP9）。
//!
//! PDB 骨架 `resources/history/roof_edit.rs`。命名实体 `RoofEdit` / `RoofReplayCmd`。
//! OBS：`RoofEdit native replay variants`、`Roof history VisualEdit serialized fields`
//! （11 个 u64 场 offset 4,8,12,16,20,24,28,32,40,48,56 = 变体 tag + 字段）。
//! serde_json 序列化/反序列化（CompactFormatter + Value 枚举分发，tag 编号 U）。
//! 与 `reconstruction/roof/edit_rules.rs`（EditFeedback 事件 tag 2/3/10/11）对应。

#![allow(dead_code, unused_imports)]

use crate::typed_gap::Gap;

/// 屋顶历史编辑记录（serde 序列化；变体/字段 U）。
#[derive(Clone, Debug)]
pub enum RoofEdit {
    /// 变体编号/字段布局 U（OBS 11×u64）。
    Unknown(Gap),
}

/// 屋顶回放命令（replay_roof 消费）。
#[derive(Clone, Debug)]
pub struct RoofReplayCmd(pub Gap);

impl RoofEdit {
    /// 序列化（impl$0::serialize，serde_json CompactFormatter）。字段语义 U。
    pub fn to_json(&self) -> Vec<u8> {
        let _ = self;
        Vec::new()
    }
}
