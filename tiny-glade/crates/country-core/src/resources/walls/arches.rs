//! 拱/过梁跨帧状态（walls/arches.rs）。
//!
//! 原 B `resources/walls/arches.rs`（PDB 实体 `ArchSegment`/`ArchSegments`/
//! `BuildingHadArchesLastFrame`/`LintelSegment`/`LintelSegments`；构造系统
//! `system_wall_constructor::construct_arches`/`construct_lintels` 消费/写回）。
use bevy_ecs::{component::Component, resource::Resource};

use crate::Gap;

/// 一段拱（O 名；布局 U → Gap）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ArchSegment(pub Gap);

/// 拱段集合（O 名；跨帧写回容器）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ArchSegments(pub Gap);

/// 上一帧是否有拱（O 名；bool 标志）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct BuildingHadArchesLastFrame(pub bool);

/// 一段过梁（O 名；布局 U）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LintelSegment(pub Gap);

/// 过梁集合（O 名；容器）。
#[derive(Resource, Clone, Debug, Default)]
pub struct LintelSegments(pub Gap);

/// 拱段实体标记（Query 过滤；O 名）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ArchSegmentMarker;
