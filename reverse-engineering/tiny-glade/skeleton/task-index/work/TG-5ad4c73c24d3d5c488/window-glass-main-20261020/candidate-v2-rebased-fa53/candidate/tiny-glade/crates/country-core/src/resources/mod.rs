pub mod active_raster_stroke;
pub mod raster_stroke;
pub mod audio_instance;
pub mod audio_resources;
#[cfg(feature = "native-audio")]
pub mod native_audio;
pub mod audio_settings;
pub mod brush_sizes;
pub mod color_icon;
pub mod color_id;
pub mod color_id_roof;
pub mod clutter_meshes;
pub mod compute_mouse_depth_value;
pub mod read_mouse_depth_value;
pub mod editing_highlight;
pub mod game_clock;
pub mod glade_color_customization;
pub mod in_progress_curve;
pub mod gothic_window_profile;
pub mod grading_lut;
pub mod grading_lut_cube;
pub mod gui;
pub mod gpu_lut;
pub mod history;
pub mod logo_animation;
pub mod onboarding;
pub mod photo_mode;
pub mod render_input_views;
pub mod roofs;
pub mod roof_tile_ssbo;
pub mod sparkle;
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
pub mod placement_data;
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
pub mod rt;
pub mod rocky_terrain;
pub mod terrain_height;
pub mod terrain_editing;
pub mod terrain_rock_mask;
pub mod steam;
pub mod water_exclude_clutter_mask;
pub mod water_mask;
pub mod water;
pub mod events;
pub mod path_stroke;
pub mod prefab;
pub mod world_raster;
pub mod world_raster_defs;
pub mod shutdown;
pub mod platform;
pub mod shader_refactoring_state;

pub mod weather;
pub mod drawing_input_smoother;

/// Original Olden placement exclusion owner.
pub mod preplaced_autoclutter;

/// `country_core::resources::shape_intersections`（原版模块路径；原版 `ShapeIntersections`
/// 由 `ClutterPlugin::build` 在 0x14065257c `insert_resource`，初值为空 hashbrown 表 + `RandomState`
/// 共 0x30 B）。
///
/// 机器码证据：`detect_intra_shape_corners`（0x141cd1a90）把它作为 `ResMut` 取用并对其做
/// `HashMap::retain`，桶步长 0x28 B；`HashMap<[WallId; 2], Vec<Vec2>>` 的桶恰为 16 + 24 = 40 B。
/// 键/值的精确泛型实参未从符号取得（PDB 无该泛型实例名），此处按桶大小取上述形状，**键值类型
/// 视为未闭合**。该模块本在原版 `country_core` 内，为遵守“不新增文件”以内联模块承载，路径不变。
pub mod shape_intersections {
    use crate::resources::walls::WallId;
    use bevy_ecs::resource::Resource;
    use glam::Vec2;
    use hashbrown::HashMap;
    use std::hash::RandomState;

    #[derive(Resource, Clone, Debug, Default)]
    pub struct ShapeIntersections(pub HashMap<[WallId; 2], Vec<Vec2>, RandomState>);
}

/// 窗灯共享状态与真实组件，供装饰生产者和玻璃渲染共同借用。
pub mod window_lights;
