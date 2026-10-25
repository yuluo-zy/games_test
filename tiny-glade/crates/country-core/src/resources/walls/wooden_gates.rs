//! 木门表（walls/wooden_gates.rs）。
//!
//! O：`Gates` = `HashMap<WallId, Vec<u64>>`（entry stride 0x20，值元素 8B =
//! 实体引用；`Gates::clear` 0xaaae80 释放）。由
//! `system_wall_constructor::construct_gates` 维护（evidence/building-gates）。
use bevy_ecs::resource::Resource;
use std::collections::HashMap;

use crate::resources::walls::WallId;

/// `Gates`（O：每墙木门实体引用表）。
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
    /// 取/建该墙的门引用桶（construct_gates 写回）。
    pub fn for_wall_mut(&mut self, id: WallId) -> &mut Vec<u64> {
        self.per_wall.entry(id).or_default()
    }
}
