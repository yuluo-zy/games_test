//! `country_core::systems` — 合并模块树（wp1-16）。
//!
//!   - `render_loop`    wp5 帧通道组合
//!   - `terrain`        wp8 地形编辑 / 射线 / 回读系统
//!   - `paths`          wp6 + wp16 路径绘制 / 历史 / 笔刷预览
//!   - `ui`             wp15 真实 UI 域（widget 状态 / signifier / ui_update）
//!   - `wall`           wp12 墙系统（57 单元）
//!   - `audio`          wp14 音频域（31 模块）
//!   - `history`/`save` wp13 历史/存档系统
//!   - `backend`/`collision`/`debug`/`steam`/`ivy`  wp16 misc 系统
//!   - `camera_rig`     wp12 facade（`MainCamera`）→ wp20 恢复的 CameraBody 族
//!   - `main_camera`/`look_at_camera`/`time_of_day`/`time_update`/`weather_system`/
//!     `world_raster`/`timelapse_replay`/`mode_manager`/`prefab`  wp20 相机/时间/世界
//!   - `ui_2d_follow_mouse`/`ui_camera_mode_gizmos`/`ui_clutter`/`ui_edit_*`/
//!     `ui_move_*`/`ui_focus_*`/`ui_raise_shape`  wp20 剩余 UI 系统
//!   - `arrow_gizmo_animation`/`brush_preview`/`clear_canvas`/`despawn`/
//!     `mesh_follow_terrain`/`mod_key`/`utils`/`utils_intersect`  wp20 misc 系统

pub mod audio;
pub mod backend;
pub mod camera_rig;
pub mod collision;
pub mod debug;
pub mod garden;
pub mod history;
pub mod ivy;
pub mod paths;
pub mod render_loop;
pub mod save;
pub mod steam;
pub mod terrain;
pub mod ui;
pub mod wall;

// WP20: camera / remaining-UI / time-weather / backend-input systems.
pub mod arrow_gizmo_animation;
pub mod brush_preview;
pub mod clear_canvas;
pub mod despawn;
pub mod look_at_camera;
pub mod main_camera;
pub mod mesh_follow_terrain;
pub mod mod_key;
pub mod mode_manager;
pub mod prefab;
pub mod time_of_day;
pub mod time_update;
pub mod timelapse_replay;
pub mod ui_2d_follow_mouse;
pub mod ui_camera_mode_gizmos;
pub mod ui_clutter;
pub mod ui_edit_focus;
pub mod ui_edit_freehand_walls;
pub mod ui_focus_mode_shape_height;
pub mod ui_move_freehand_walls;
pub mod ui_move_shapes;
pub mod ui_raise_shape;
pub mod utils;
pub mod utils_intersect;
pub mod weather_system;
pub mod world_raster;

/// `GameSystemSet`（O：enum-iterator 驱动 `configure_sets(Update)`；WP20 系统
/// 集标签；变体集 U）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GameSystemSet {
    Camera,
    Time,
    WorldRaster,
    BackendInput,
    Ui,
    Timelapse,
    Prefab,
    ModeManager,
    Misc,
}
