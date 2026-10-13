//! inner_walls.rs — 内部墙集合资源（WP11 状态家族 + WP12 系统向方法面）。
//!
//! 源骨架 `resources/walls/inner_walls.rs`（B）；PDB 命名实体 `InnerWalls`。
//! WP12 系统以 `ResMut<InnerWalls>` 消费（adjust_wall_height_impl 等骨架签名）。
//! 方法体为 typed gap（编辑几何 U）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;
use std::collections::HashMap;

use crate::resources::walls::{WallId, inner_wall_state::WallState};
use crate::typed_gap::Gap;

/// 内部墙集合资源（O：`HashMap<WallId, WallState>` 权威编辑状态）。
/// 内部字段（dirty 集合、roof 关联、terrain 收集）为 typed gap。
#[derive(Resource, Clone, Debug, Default)]
pub struct InnerWalls {
    /// 内部墙记录（O：HashMap<WallId, WallState>；hashbrown/std map 属库实现）。
    pub walls: HashMap<WallId, WallState>,
    /// 新建墙 ID 分配游标（O：`new_wall` 分配；布局 U）。
    pub next_id: u64,
    /// 其它未恢复字段（dirty 等）。
    pub gap: Gap,
}

impl InnerWalls {
    /// O：按 id 取编辑态。
    pub fn get(&self, id: WallId) -> Option<&WallState> {
        self.walls.get(&id)
    }
    /// O：按 id 取（缺失 panic）。
    pub fn get_unwrapped(&self, id: WallId) -> &WallState {
        self.walls.get(&id).expect("inner wall present")
    }
    /// O：包含判定。
    pub fn contains(&self, id: WallId) -> bool {
        self.walls.contains_key(&id)
    }
    /// O：全部墙 id。
    pub fn wall_ids(&self) -> Vec<WallId> {
        self.walls.keys().copied().collect()
    }
    /// O：新建墙（分配 id 并插入）。
    pub fn new_wall(&mut self, state: WallState) -> WallId {
        let id = WallId(self.next_id);
        self.next_id = self.next_id.wrapping_add(1);
        self.walls.insert(id, state);
        id
    }
    /// O：`mark_as_non_enclosed`（finish_wall）。
    pub fn mark_as_non_enclosed(&mut self, _id: WallId) {}
    /// O：按闭包更新编辑态。
    pub fn update_wall_state(&mut self, id: WallId, f: impl FnOnce(&mut WallState)) {
        if let Some(s) = self.walls.get_mut(&id) {
            f(s);
        }
    }
    /// O：更新编辑高度。
    pub fn update_wall_state_height(&mut self, _id: WallId, _height: f32) {}
    /// O：更新墙曲线。
    pub fn update_wall_state_curve(&mut self, _id: WallId, _c: crate::utils::curve::Curve) {}
    /// O：更新封闭形状记录。
    pub fn update_wall_state_enclosure(&mut self, _id: WallId, _s: crate::resources::walls::WallShapeSnapshot) {}
    /// O：更新徒手高度类型。
    pub fn update_freehand_ty(&mut self, _id: WallId, _ty: u8) {}
    /// O：更新拱类型。
    pub fn update_arch_type(&mut self, _id: WallId, _ty: u8) {}
    /// O：更新石膏剥离量。
    pub fn update_plaster_peel(&mut self, _id: WallId, _amount: f32) {}
    /// O：更新样式。
    pub fn update_style(&mut self, _id: WallId, _style: crate::resources::walls::inner_wall_state::WallStyle) {}
    /// O：更新标高。
    pub fn update_elevation(&mut self, _id: WallId, _e: f32) {}
}
