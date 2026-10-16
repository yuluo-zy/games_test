//! 升降支撑统计（walls/elevation_support_stats.rs）。
//!
//! 原 B `resources/walls/elevation_support_stats.rs`（PDB 实体
//! `ElevationSupportStats`/`ElevationSupports`；audio elevation_supports_audio
//! 消费；`system_wall_constructor` 的 construct_elevation_supports 维护）。
use bevy_ecs::resource::Resource;

use crate::Gap;

/// 升降支撑统计（O 名；布局 U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ElevationSupportStats(pub Gap);

/// 升降支撑集合（O 名；布局 U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ElevationSupports(pub Gap);
