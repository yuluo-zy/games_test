pub mod active_raster_stroke;
pub mod raster_stroke;
pub mod audio_instance;
pub mod audio_resources;
pub mod brush_sizes;
pub mod color_icon;
pub mod color_id;
pub mod editing_highlight;
pub mod game_clock;
pub mod glade_color_customization;
pub mod in_progress_curve;
pub mod gothic_window_profile;
pub mod grading_lut;
pub mod grading_lut_cube;
pub mod gui;
pub mod history;
pub mod logo_animation;
pub mod onboarding;
pub mod photo_mode;
pub mod render_input_views;
pub mod roofs;
pub mod stairs;
pub mod stairs_walls;
pub mod save_file;
pub mod session;
pub mod time_of_day;
pub mod time;
pub mod tree_storage;
pub mod toolbar;
pub mod ui_settings;
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
pub mod critters;
pub mod ivy;
pub mod glade_border;
pub mod glade_settings;
pub mod mouse_capture;
pub mod mouse_terrain_raycast;
pub mod image_formation;
pub mod garden_mask;
pub mod render;
pub mod render_ssbo;
pub mod rocky_terrain;
pub mod terrain_height;
pub mod terrain_editing;
pub mod terrain_rock_mask;
pub mod water_exclude_clutter_mask;
pub mod water_mask;
pub mod water;
pub mod events;
pub mod path_stroke;
pub mod prefab;
pub mod world_raster;
pub mod world_raster_defs;
