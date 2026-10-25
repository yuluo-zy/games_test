//! `asset_libraries` — INTEGRATION MARKER facade (wp4/wp16-owned asset
//! libraries). Real modules live in `reconstruction/country-core/src/
//! asset_libraries/`.

#![allow(dead_code, unused_imports)]

use crate::Gap;

pub mod texture_library {
    use super::Gap;
    use bevy_ecs::resource::Resource;

    /// `AssetTextureLibrary` (O: `clear_preset_texture` called by
    /// `clear_canvas`).
    #[derive(Resource, Clone, Debug, Default)]
    pub struct AssetTextureLibrary {
        pub raw: Gap,
    }
    impl AssetTextureLibrary {
        pub fn clear_preset_texture(&mut self) {}
    }
}

pub mod ssbo_library {
    use super::Gap;
    use bevy_ecs::resource::Resource;

    /// `SsboId` (O: `AssetSsboLibrary::get<SsboId>` key).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
    pub struct SsboId(pub u64);

    /// `BufferState` (O).
    #[derive(Clone, Debug, Default)]
    pub struct BufferState {
        pub raw: Gap,
    }

    /// `AssetSsboLibrary` (O: staging-SSBO lookup in `draw_raster`).
    #[derive(Resource, Clone, Debug, Default)]
    pub struct AssetSsboLibrary {
        pub raw: Gap,
    }
    impl AssetSsboLibrary {
        pub fn get(&self, _id: SsboId) -> Option<Box<BufferState>> {
            None
        }
    }
}
