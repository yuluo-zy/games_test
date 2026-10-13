//! sparkle.rs — 闪烁粒子 SSBO（WP21）。
//!
//! O：PDB 实体 `SparkleSsbo`；空 bundle。U。

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// `SparkleSsbo`（O 名；闪烁粒子 SSBO，U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct SparkleSsbo(pub Gap);
