//! garden_mask.rs — 花园掩码 SSBO（WP21）。
//!
//! O：PDB 实体 `GardenMaskSsbo`；空 bundle（世界光栅 512² 族，wp7/10）。布局 U。

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 花园掩码 SSBO（O 名；世界光栅 512² 域）。
#[derive(Resource, Clone, Debug, Default)]
pub struct GardenMaskSsbo(pub Gap);
