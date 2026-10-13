//! public_walls.rs — 公共墙资源（WP6 恢复）。
//!
//! 源 SHA256（B skeleton）`resources/walls/public_walls.rs`；PDB 命名实体：
//! `PublicWalls` / `WallEntities` / `WallPlasters`。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 公共墙集合资源（O：`Res<PublicWalls>`；`nani_update_curved_halftimber` /
/// `nani_update_plaster` 读墙/石膏索引）。布局 U。
#[derive(Resource, Clone, Debug, Default)]
pub struct PublicWalls(pub Gap);

/// 墙实体集合（O：命名实体）。
#[derive(Resource, Clone, Debug, Default)]
pub struct WallEntities(pub Gap);

/// 墙石膏集合（O：命名实体）。
#[derive(Resource, Clone, Debug, Default)]
pub struct WallPlasters(pub Gap);
