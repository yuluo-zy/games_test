//! 石膏资源族（walls/plaster.rs）。
//!
//! 原 B `resources/walls/plaster.rs`（PDB 实体 `Plaster`/`AnimatedPlasterPeel`/
//! `CreatePlasterCmd`/`DeletePlasterCmd`/`RecalculateHalfTimberMeshCmd`/
//! `RecalculatePlasterCmd`/`RecalculateWoodenTurretCmd`；`system_wall_constructor`
//! 的 plaster_systems 消费）。
use bevy_ecs::{component::Component, event::Event, resource::Resource};

use crate::{Gap, resources::walls::WallId};

/// 石膏墙（O：ECS `&Plaster`；布局 U）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Plaster(pub Gap);

/// 石膏剥离动画资源（O：`Res<AnimatedPlasterPeel>`）。
#[derive(Resource, Clone, Debug, Default)]
pub struct AnimatedPlasterPeel(pub Gap);

/// 创建石膏命令（O 名）。
#[derive(Event, Clone, Copy, Debug)]
pub struct CreatePlasterCmd(pub WallId);

/// 删除石膏命令（O 名）。
#[derive(Event, Clone, Copy, Debug)]
pub struct DeletePlasterCmd(pub WallId);

/// 石膏重算命令（O 名）。
#[derive(Event, Clone, Copy, Debug)]
pub struct RecalculatePlasterCmd(pub WallId);
