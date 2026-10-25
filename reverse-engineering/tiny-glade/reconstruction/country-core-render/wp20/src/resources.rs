//! `resources` — INTEGRATION MARKER facade for country-core resources
//! (wp5-wp10/wp13/wp16-owned). Real modules live in
//! `reconstruction/country-core/src/resources/`. This gate declares the exact
//! typed-gap surfaces the WP20 systems reference; the integration owner maps
//! them to the real types.

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// Game time (`resources::time`, wp12 facade / WP20 time_update target).
pub mod time {
    use super::Gap;
    use bevy_ecs::resource::Resource;

    /// `GameTime` (O: 56 B clock `{Duration secs+nanos, total_f32@+0x10,
    /// total_60ths_u64@+0x18, frac_seconds@+0x20, flag@+0x30, frame_count
    /// @+0x2c}`).
    #[derive(Resource, Clone, Debug, Default)]
    pub struct GameTime {
        pub raw: Gap,
    }
    impl GameTime {
        pub fn delta_seconds(&self) -> f32 {
            0.0
        }
    }

    /// `Time` (bevy_time facade; offline-absent).
    #[derive(Resource, Clone, Copy, Debug, Default)]
    pub struct Time {
        pub raw: Gap,
    }
}

/// Time-of-day state (`resources::time_of_day`; WP20 time_of_day target).
pub mod time_of_day {
    use super::Gap;
    use bevy_ecs::resource::Resource;

    /// O constant read by `time_of_day`.
    pub const TIME_OF_DAY_SPEED: f32 = 1.0;

    /// `TimeOfDay` state resource (O: `progress@[1]`, `state@[0]`,
    /// `target_hour@[2]`, `year@[10]`).
    #[derive(Resource, Clone, Debug, Default)]
    pub struct TimeOfDay {
        pub raw: Gap,
    }
}

/// Toolbar (`resources::toolbar`; WP16/misc).
pub mod toolbar {
    use super::Gap;
    use bevy_ecs::resource::Resource;

    /// `Toolbar` (O: `Toolbar::new` @0x140AF8E00 — 11 tool categories with
    /// per-category `(mode, variant)` Vecs; demo-mode reduced sets).
    #[derive(Resource, Clone, Debug, Default)]
    pub struct Toolbar {
        pub raw: Gap,
    }
    impl Toolbar {
        pub fn new() -> Self {
            Toolbar { raw: Gap }
        }
    }
}

/// World raster (`resources::world_raster`; wp7-owned `WorldRaster<512>`).
pub mod world_raster {
    use super::Gap;
    use bevy_ecs::resource::Resource;

    /// `WorldRaster<N>` (wp7; N×N u8 grid).
    #[derive(Resource, Clone, Debug, Default)]
    pub struct WorldRaster<const N: usize> {
        pub cells: Vec<u8>,
    }
    impl<const N: usize> WorldRaster<N> {
        pub fn clear_all(&mut self) {}
        pub fn copy_to_staging<const S: usize>(&mut self) {}
    }
}

/// World-raster defs (`resources::world_raster_defs`; wp7-owned SSBOs).
pub mod world_raster_defs {
    use super::Gap;
    use bevy_ecs::resource::Resource;

    /// Raster staging SSBO carriers (wp7: Path/Cobblestone1..3/Garden/Water).
    macro_rules! raster_staging {
        ($name:ident) => {
            #[derive(Resource, Clone, Debug, Default)]
            pub struct $name {
                pub raw: Gap,
            }
        };
    }
    raster_staging!(PathRasterStagingSsbo);
    raster_staging!(Cobblestone1RasterStagingSsbo);
    raster_staging!(Cobblestone2RasterStagingSsbo);
    raster_staging!(Cobblestone3RasterStagingSsbo);
    raster_staging!(GardenRasterStagingSsbo);
    raster_staging!(WaterRasterStagingSsbo);

    /// CPU raster variants (Path/Cobblestone/Garden/Water masks).
    #[derive(Clone, Copy, Debug, Default)]
    pub struct PathRaster(pub Gap);
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Cobblestone1Raster(pub Gap);
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Cobblestone2Raster(pub Gap);
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Cobblestone3Raster(pub Gap);

    /// `AllWorldRasterWrite::upload_to_staging` (O call from
    /// `upload_world_raster_buffers_to_staging`).
    #[derive(Resource, Clone, Debug, Default)]
    pub struct AllWorldRasterWrite {
        pub raw: Gap,
    }
    impl AllWorldRasterWrite {
        pub fn upload_to_staging(&mut self) {}
    }
}

/// Prefabs (`resources::prefab`; WP20 prefab target).
pub mod prefab {
    use super::Gap;
    use bevy_ecs::component::Component;
    use bevy_ecs::resource::Resource;

    /// `PrefabLibrary` (O: `get<PrefabName>`).
    #[derive(Resource, Clone, Debug, Default)]
    pub struct PrefabLibrary {
        pub raw: Gap,
    }

    /// `PrefabInstance` component (O).
    #[derive(Component, Clone, Debug, Default)]
    pub struct PrefabInstance {
        pub raw: Gap,
    }

    /// `PrefabInstanceData` component (O; GPU instance data).
    #[derive(Component, Clone, Debug, Default)]
    pub struct PrefabInstanceData {
        pub raw: Gap,
    }
}

/// History (`resources::history`; wp13-owned `History::add`).
pub mod history {
    use super::Gap;
    use bevy_ecs::resource::Resource;

    /// `History` (O: `History::add` called by record_raster_history /
    /// impl_raise_shape).
    #[derive(Resource, Clone, Debug, Default)]
    pub struct History {
        pub raw: Gap,
    }
    impl History {
        pub fn add(&mut self, _record: impl Sized) {}
    }
}

/// Placement data (`resources::placement_data`; `PlacementDataSsbo`).
pub mod placement_data {
    use super::Gap;
    use bevy_ecs::resource::Resource;

    /// `PlacementDataSsbo` (O: `update_prefabs` `update_instance_data`).
    #[derive(Resource, Clone, Debug, Default)]
    pub struct PlacementDataSsbo {
        pub raw: Gap,
    }
}

/// Audio core (`resources::audio_resources`; `AudioCoreInner`).
pub mod audio_resources {
    use super::Gap;
    use bevy_ecs::resource::Resource;

    /// `AudioCoreInner` (O: `set_parameter` read by time_of_day).
    #[derive(Resource, Clone, Debug, Default)]
    pub struct AudioCoreInner {
        pub raw: Gap,
    }
    impl AudioCoreInner {
        pub fn set_parameter(&mut self, _name: &str, _value: f32) {}
    }
}
