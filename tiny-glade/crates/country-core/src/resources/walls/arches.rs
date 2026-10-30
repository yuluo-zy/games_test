//! 拱/过梁跨帧状态（walls/arches.rs）。
//!
//! 原 B `resources/walls/arches.rs`（PDB 实体 `ArchSegment`/`ArchSegments`/
//! `BuildingHadArchesLastFrame`/`LintelSegment`/`LintelSegments`；构造系统
//! `system_wall_constructor::construct_arches`/`construct_lintels` 消费/写回）。
use bevy_ecs::{component::Component, resource::Resource};


/// Native104B record: Vec<f32>, Vec<Vec3>, Vec<f32>, then source/range metadata.
/// Reuse the original materialization owner rather than retaining opaque Gap.
pub type ArchSegment = crate::systems::wall::construction::structures::MaterializedWallSegment;
/// Native HashMap<WallId, Vec<ArchSegment>> populated by construct_arches.
#[derive(Resource, Clone, Debug, Default)]
pub struct ArchSegments(pub std::collections::HashMap<super::WallId, Vec<ArchSegment>>);

/// 上一帧是否有拱（O 名；bool 标志）。
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct BuildingHadArchesLastFrame(pub std::collections::HashMap<u64, bool>);

/// 原14076e3xx逐字段move的104B过梁段，与拱段共享真实三Vec/索引/LoopRange所有者。
/// 原writer替换heights后保留positions、points_u及range，不能以空Gap代替。
pub type LintelSegment = crate::systems::wall::construction::structures::MaterializedWallSegment;

/// 原有序过梁段按WallId保留，供碰撞主系统逐段消费并在下一次重建时替换释放。
#[derive(Resource, Clone, Debug, Default)]
pub struct LintelSegments(pub std::collections::HashMap<super::WallId, Vec<LintelSegment>>);

/// 拱段实体标记（Query 过滤；O 名）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ArchSegmentMarker;
