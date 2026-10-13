//! stairs_collider.rs — 楼梯碰撞体注册（WP21）。
//!
//! O：PDB 实体 `StairColliderRegistry`；无独立编译函数。rapier 碰撞世界边界。U。

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 楼梯碰撞体注册（O 名；布局 U；复用边界 rapier）。
#[derive(Resource, Clone, Debug, Default)]
pub struct StairColliderRegistry(pub Gap);
