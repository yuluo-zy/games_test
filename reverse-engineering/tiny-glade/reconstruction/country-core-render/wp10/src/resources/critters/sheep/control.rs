//! `resources::critters::sheep::control` — 绵羊控制快照（B 骨架 `resources/critters/sheep/control.rs`）。
//! serde_json 序列化（保存/回放路径）；`capture_sheep_snapshot` 行 O。

#![allow(dead_code, unused_imports)]

use serde::{Deserialize, Serialize};

use crate::typed_gap::Gap;

/// 单只绵羊快照（O 名；serde；字段 U）。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SheepSnapshot {
    /// 未恢复字段。
    pub _gap: Gap,
}

/// 绵羊控制快照（整体）（O 名；serde；字段 U）。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SheepControlSnapshot {
    /// 绵羊集合（Vec 大小 U）。
    pub sheep: Vec<SheepSnapshot>,
}

/// `capture_sheep_snapshot`（O 名；闭包 `closure$0` 逐羊采样；字段映射 U）。
pub fn capture_sheep_snapshot(_control: &SheepControlSnapshot) -> SheepControlSnapshot {
    _control.clone()
}
