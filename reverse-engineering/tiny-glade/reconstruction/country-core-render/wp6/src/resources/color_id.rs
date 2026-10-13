//! color_id.rs — 颜色索引资源族（WP6 恢复）。
//!
//! 源 SHA256（B skeleton）`resources/color_id.rs`；PDB 命名实体 10 个：
//! `ColorId` / `WallColorIds` / `GateColorIds` / `HalftimberColorIds` /
//! `{Wall,Gate,Halftimber}ColorIdSsbo` / `Set{Gate,Wall,Halftimber}ColorIdCmd`。
//! 本模块只恢复 nani systems 消费的 `ColorId` 与 `HalftimberColorIds`。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 颜色索引（O：ECS `&ColorId`；serde newtype 证据——`visit_newtype_struct`）。
/// 布局 I：`u32` 颜色/材质索引（`WallColorIds::default_value` 有默认值逻辑）。
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct ColorId(pub u32);

/// 半木色索引集合资源（O：`Res<HalftimberColorIds>`；
/// `nani_update_curved_halftimber` 读端帽颜色）。
/// 布局 U（typed gap；`HalftimberColorIds::new` / `iter_nondefault` 存在）。
#[derive(Resource, Clone, Debug, Default)]
pub struct HalftimberColorIds(pub Gap);

/// 墙色索引集合（O：命名实体；本组系统未消费——typed gap 备用）。
#[derive(Resource, Clone, Debug, Default)]
pub struct WallColorIds(pub Gap);

/// 门色索引集合（O：命名实体；本组系统未消费）。
#[derive(Resource, Clone, Debug, Default)]
pub struct GateColorIds(pub Gap);
