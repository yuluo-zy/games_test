//! 曲面半木构资源（walls/curved_halftimber.rs）。
//!
//! 原 B `resources/walls/curved_halftimber.rs`（PDB 实体
//! `CurvedHalfTimberDrawables`；`system_wall_constructor` 的
//! curved_halftimber 系统维护；渲染侧 nani `CurvedHalfTimber` 子集）。
use bevy_ecs::resource::Resource;

use crate::Gap;

/// 曲面半木构绘制表（O 名；布局 U → Gap）。
#[derive(Resource, Clone, Debug, Default)]
pub struct CurvedHalfTimberDrawables(pub Gap);
