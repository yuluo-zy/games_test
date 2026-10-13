//! plaster.rs — 石膏资源族（WP6 恢复）。
//!
//! 源 SHA256（B skeleton）`resources/walls/plaster.rs`；PDB 命名实体 7 个：
//! `Plaster` / `AnimatedPlasterPeel` / `CreatePlasterCmd` / `DeletePlasterCmd` /
//! `RecalculateHalfTimberMeshCmd` / `RecalculatePlasterCmd` / `RecalculateWoodenTurretCmd`。
//!
//! 注：模块路径为 `resources::`，但 `Plaster` 在 ECS 查询中作**组件**使用
//! （`Query<(&NaniActiveInstance, &Plaster), ...>`，O），`AnimatedPlasterPeel` 作资源
//! （`Res<AnimatedPlasterPeel>`，O）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 石膏墙（O：ECS `&Plaster`；`nani_update_plaster` 读）。布局 U（0x14207b290
/// 反编译读多个字节段；剥离动画参数）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Plaster(pub Gap);

/// 石膏剥离动画资源（O：`Res<AnimatedPlasterPeel>`；plaster 系统读动画相位）。
#[derive(Resource, Clone, Debug, Default)]
pub struct AnimatedPlasterPeel(pub Gap);
