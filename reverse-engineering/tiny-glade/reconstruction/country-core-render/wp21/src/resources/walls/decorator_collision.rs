//! decorator_collision.rs — 装饰物理碰撞体（WP21）。
//!
//! O：PDB 实体 `DecoratorPhysicsColliders`；无独立编译函数。rapier/parry 碰撞世界
//! 边界（parry3d 0.17.0 为复用依赖）。

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 装饰物理碰撞体集合（O 名；布局 U；复用边界 parry3d/rapier）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DecoratorPhysicsColliders(pub Gap);
