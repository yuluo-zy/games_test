//! `ui_autoclutter.rs` — 右键把 auto-clutter 转 deco-clutter（B 0x14099ca60，O）。
//!
//! 证据：`chapters/systems/ui_clutter/ui_autoclutter.rs.md`、`ui-input/DIGEST.md`、
//! re-analyst-interaction：0x48 B spawn 事件统一布局（tag=2@0、subtype=3@+0x29、
//! type@+0x3c、entity@+0x40）；旋转 = `FUN_1408b83d0`（`ClutterMeshTy::from`）+ 常量区
//! UNK_1429cd1/1429258。
//!
//! 完整转换（`get_clutter_rot_in_roof_space` 行 403、`maintain_autoclutter_parent_hashmap`
//! 行 410）依赖 DecoratorStorage 实体机制（recovered_building，宿主），本实现以
//! [`EvRightClickedAutoClutter`] 消费 + 0x48 B [`SpawnAutoClutter`] 产出登记结构。
use bevy_ecs::prelude::*;

use super::{
    debug_spawn_clutter::SpawnAutoClutter,
    rmb_autoclutter::EvRightClickedAutoClutter,
};

/// 0x48 B spawn 事件布局（O，re-analyst 定案）。
pub struct SpawnEventLayout;
impl SpawnEventLayout {
    /// tag = 2 @0。
    pub const TAG: u8 = 2;
    /// subtype = 3 @+0x29。
    pub const SUBTYPE: u8 = 3;
}

/// `maintain_autoclutter_parent_hashmap`（行 410，I：维护 clutter→parent 映射）——
/// 依赖 DecoratorStorage 实体位图，待取证；本实现以事件计数登记结构。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct AutoClutterParentMap {
    /// 已维护的条目数（宿主从 DecoratorStorage 同步）。
    pub entries: u64,
}

/// `ui_convert_autoclutter_to_decoclutter` @0x14099ca60（O 门 + 结构）。
pub fn ui_convert_autoclutter_to_decoclutter(
    mut right_clicks: EventReader<EvRightClickedAutoClutter>,
    mut spawned: EventWriter<SpawnAutoClutter>,
    mut parents: ResMut<AutoClutterParentMap>,
) {
    for click in right_clicks.read() {
        // 旋转 = 三向量点积 → acos 链 + ClutterMeshTy::from（O）。输入向量（roof 空间朝向）
        // 依赖 DecoratorStorage 实体位图，re-analyst 补证前不臆造——旋转暂置 0 并登记待取证。
        let rotation = 0.0;
        spawned.write(SpawnAutoClutter {
            entity: click.entity,
            ty: click.ty,
            position: click.position,
            rotation,
        });
        parents.entries = parents.entries.wrapping_add(1);
    }
}
