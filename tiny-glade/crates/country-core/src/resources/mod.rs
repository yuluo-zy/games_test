pub mod color_id;
pub mod gothic_window_profile;
pub mod grading_lut;
pub mod grading_lut_cube;
pub mod render_input_views;
pub mod roofs;
pub mod session;
pub mod walls;
pub mod window;

pub mod texture_presets;
pub mod placement_data {
    use crate::render::ssbo::StorageElement;
    use serde::Deserialize;
    #[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
    pub struct PlacementDataSsbo {
        pub x: f32,
        pub y: f32,
    }
    impl StorageElement for PlacementDataSsbo {
        const NAME: &'static str = "PlacementDataSsbo";
    }
}
pub mod cursor_terrain_raycast;
pub mod image_formation;
pub mod render;
pub mod render_ssbo;
pub mod water_mask;
pub mod world_raster;
