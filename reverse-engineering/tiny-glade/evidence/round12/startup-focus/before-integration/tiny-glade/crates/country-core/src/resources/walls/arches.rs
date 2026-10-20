//! 拱/过梁跨帧状态（walls/arches.rs）。
//!
//! 原 B `resources/walls/arches.rs`（PDB 实体 `ArchSegment`/`ArchSegments`/
//! `BuildingHadArchesLastFrame`/`LintelSegment`/`LintelSegments`；构造系统
//! `system_wall_constructor::construct_arches`/`construct_lintels` 消费/写回）。
use bevy_ecs::{component::Component, resource::Resource};

use crate::Gap;

/// Native104B record: Vec<f32>, Vec<Vec3>, Vec<f32>, then source/range metadata.
/// Reuse the original materialization owner rather than retaining opaque Gap.
pub type ArchSegment = crate::systems::wall::construction::structures::MaterializedWallSegment;
/// Native HashMap<WallId, Vec<ArchSegment>> populated by construct_arches.
#[derive(Resource, Clone, Debug, Default)]
pub struct ArchSegments(pub std::collections::HashMap<super::WallId, Vec<ArchSegment>>);

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
