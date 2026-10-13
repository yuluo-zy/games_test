//! arches.rs — 拱段跨帧状态（WP21）。
//!
//! O：PDB 实体 `ArchSegment` / `ArchSegments` / `BuildingHadArchesLastFrame` /
//! `LintelSegment` / `LintelSegments`；无独立编译函数（`impl$N::default` 全内联）。
//! 业务上下文：`system_wall_constructor::construct_arches` 产出 96B WallBrickInstance
//! 拱砖 + ConstructGateCmd（evidence/building-arches）。本模块是被该系统写回的
//! 跨帧拱段状态容器。布局 U → typed gap。

use bevy_ecs::resource::Resource;
use bevy_ecs::component::Component;

use crate::typed_gap::Gap;

/// 一段拱（O 名；布局 U）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ArchSegment(pub Gap);

/// 拱段集合（O 名；跨帧写回容器，U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ArchSegments(pub Gap);

/// 上一帧是否有拱（O 名；bool 标志，U 布局）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct BuildingHadArchesLastFrame(pub bool);

/// 一段过梁（O 名；布局 U）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LintelSegment(pub Gap);

/// 过梁集合（O 名；容器 U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct LintelSegments(pub Gap);

/// `ArchSegment`/`LintelSegment` 的 Default（O：`impl$3/5/7::default` 内联，无独立体）。
impl ArchSegments {
    pub fn new() -> Self {
        Self::default()
    }
}
impl LintelSegments {
    pub fn new() -> Self {
        Self::default()
    }
}
