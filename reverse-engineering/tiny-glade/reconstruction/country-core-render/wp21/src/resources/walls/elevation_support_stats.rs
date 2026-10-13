//! elevation_support_stats.rs — 升降支撑统计（WP21）。
//!
//! O：PDB 实体 `ElevationSupportStats` / `ElevationSupports`；无独立编译函数
//! （`impl$1/2::default` 内联）。语义：墙高调整的升降支撑统计（audio
//! elevation_supports_audio 消费）。布局 U。

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 升降支撑统计（O 名；布局 U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ElevationSupportStats(pub Gap);

/// 升降支撑集合（O 名；布局 U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ElevationSupports(pub Gap);
