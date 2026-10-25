//! dormer_tiles.rs — 老虎窗瓦片资源（WP12 系统向 facade）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 老虎窗瓦片（O：`DormerTiles`；布局 U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DormerTiles {
    pub raw: Gap,
}
