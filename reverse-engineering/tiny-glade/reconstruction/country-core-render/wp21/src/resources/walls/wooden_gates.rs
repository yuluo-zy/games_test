//! wooden_gates.rs — 木门（WP21）。
//!
//! O：`Gates::clear` 0xaaae80 —— `HashMap<WallId, Vec<u64>>`（entry stride 0x20，值
//! 元素 8B=实体引用），clear 释放各 Vec 并清 map。由
//! `system_wall_constructor::construct_gates` 维护（evidence/building-gates）。

use std::collections::HashMap;

use bevy_ecs::resource::Resource;

use crate::boundary::WallId;

/// `Gates`（O：每墙的木门实体引用表）。
#[derive(Resource, Clone, Debug, Default)]
pub struct Gates {
    /// 每墙木门实体引用列表（O：值元素 8B；`Gates::clear` 释放）。
    pub per_wall: HashMap<WallId, Vec<u64>>,
}

impl Gates {
    /// `clear`（O 0xaaae80）：释放全部门引用并清空表。
    pub fn clear(&mut self) {
        self.per_wall.clear();
    }
}
