//! curved_halftimber.rs — 弯曲半木墙资源（WP6 恢复）。
//!
//! 源 SHA256（B skeleton）`resources/walls/curved_halftimber.rs`；PDB 命名实体：
//! `CurvedHalfTimber` / `CurvedHalfTimberDrawables`。
//! `CurvedHalfTimber` 在 ECS 查询中作**组件**（`Query<...&CurvedHalfTimber...>`，O）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 弯曲半木墙（O：ECS `&CurvedHalfTimber`；`nani_update_curved_halftimber` 读几何）。
/// 布局 I：含 16 段端帽掩码（u128；反编译 0x142077d20 的位提取）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct CurvedHalfTimber(pub Gap);

/// 弯曲半木 drawable 集合（O：命名实体；未消费）。
#[derive(Resource, Clone, Debug, Default)]
pub struct CurvedHalfTimberDrawables(pub Gap);
