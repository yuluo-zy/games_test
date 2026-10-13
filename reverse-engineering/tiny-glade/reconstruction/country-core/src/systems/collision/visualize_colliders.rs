//! visualize_colliders.rs — collider 可视化（O）。
//!
//! 源 `systems/collision/visualize_colliders.rs`。
//! `visualize_colliders`（0x141326370）：debug 模式下遍历 collider，按形状画
//! 线框；`visualize_shape`（0x141326930）把 parry 形状转成线框图元。

#![allow(dead_code)]

/// `visualize_colliders`（O 0x141326370）。
pub fn visualize_colliders(_world: &mut bevy_ecs::world::World) {
    // U：线框图元队列（debug_viz AppendOnlyVec）。
}

/// `visualize_shape`（O 0x141326930）。
pub fn visualize_shape(_shape: &crate::reused::rapier::SharedShape) {
    // U：按形状族画 DebugLine/DebugSphere（typed gap）。
}
