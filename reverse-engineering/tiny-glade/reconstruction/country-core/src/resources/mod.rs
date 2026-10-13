//! `crate::resources` — game CPU-side state.
//!
//! Merged from: wp5-8 (render-domain / water / terrain typed gaps), wp11
//! (`walls` state types), wp13 (`session` / `prefab` / `history`), wp14
//! (`audio_resources` / `audio_settings`), wp16 (`ivy` / `rt` /
//! window_lights+color_id extensions), plus the wp12 wall-domain typed-gap
//! facade families (`events`, `gui`, `in_progress_curve`, `time`, `window`, …).

#![allow(dead_code, unused_imports)]

use crate::typed_gap::Gap;

/// Terrain/roof/grass CPU-side state referenced by the frame passes
/// (typed gaps only; `Water` lives in `resources::water` — WP7).
pub struct Terrain(pub Gap);
pub struct RoofTiles(pub Gap);
pub struct Grass(pub Gap);

// WP7: water content domain resources (RippleSourceSsbo / events / CPU state,
// water mask readbacks, session glade-type slice, brush sizes).
pub mod brush_sizes;
pub mod water;
pub mod water_exclude_clutter_mask;
pub mod water_mask;

// WP13: session / prefab / history (authoritative).
pub mod session;
pub mod prefab;
pub mod history;

// WP14: audio resources.
pub mod audio_resources;
pub mod audio_settings;

// WP16: ivy growth state + software-BVH ray-tracing world + window-light /
// color-id CPU state (extensions).
pub mod ivy;
pub mod rt;
pub mod color_id;
pub mod window_lights;

/// Frame-pass typed-gap SSBO carriers referenced through `consume::core`.
/// These are game-owned storage-buffer types the render-loop passes name but
/// the render packages do not own (wp5 OUT OF SCOPE).
pub mod render_ssbo {
    use crate::typed_gap::Gap;

    pub struct LanternSsbo(pub Gap);
    pub struct SpatialResolveOffsetsSsbo(pub Gap);
    pub struct MouseDepthValueSsbo(pub Gap);
    pub struct DofAutofocusSsbo(pub Gap);
    pub struct DofFocusReadbackSsbo(pub Gap);
    pub struct ShadowMaskHistogramSsbo(pub Gap);
    pub struct ShadowMaskMeanSsbo(pub Gap);
    pub struct WallColorIdSsbo(pub Gap);

    /// Indirect command buffer wrappers (wp2 contract; not re-implemented).
    pub struct DrawIndexedIndirectBuffer(pub Gap);
    pub struct DrawIndirectBuffer(pub Gap);
    pub struct DispatchIndirectBuffer(pub Gap);
    pub struct AtomicCounterBuffer(pub Gap);
}

// WP6: recovered nani-systems-consumed resource/component families.
pub mod critters;
pub mod render;
pub mod roofs;
pub mod stairs;
pub mod walls;
pub mod world_raster;
pub mod world_raster_defs;

// WP18: draw_shaders typed-gap carriers (texture presets + roof color-id SSBO).
pub mod color_id_roof;
pub mod texture_presets;

// WP19: LUT / boot resources.
pub mod gpu_lut;
pub mod grading_lut;
pub mod loading_screen;
pub mod logo_animation;

// WP8: recovered terrain content-domain resources (terrain_height /
// rocky_terrain / world_raster_defs + terrain editing & raycast families).
pub mod active_raster_stroke;
pub mod cursor_terrain_raycast;
pub mod mouse_terrain_raycast;
pub mod rocky_terrain;
pub mod terrain_editing;
pub mod terrain_height;
pub mod terrain_rock_mask;
pub mod terrain_stroke;

// WP12 wall-domain typed-gap facade families (genuine gaps; the wall systems
// reference these through the documented PDB names).
pub mod events;
pub mod gui;
pub mod in_progress_curve;
pub mod drawing_input_smoother;
pub mod glade_border;
pub mod editing_highlight;
pub mod read_mouse_depth_value;
pub mod onboarding;
pub mod stairs_walls;
pub mod time;
pub mod window;

// WP21: misc-resource residual family (walls/stairs/ivy/rt residuals live under
// their subpaths above; these are the remaining flat resources).
pub mod clutter_meshes;
pub mod color_icon;
pub mod daily;
pub mod feedback_report_resources;
pub mod garden_mask;
pub mod glade_color_customization;
pub mod glade_settings;
pub mod gothic_window_profile;
pub mod image_formation;
pub mod mouse_capture;
pub mod photo_mode;
pub mod placement_data;
pub mod preplaced_autoclutter;
pub mod raster_stroke;
pub mod shader_refactoring_state;
pub mod shape_intersections;
pub mod shutdown;
pub mod sparkle;
pub mod steam;
pub mod time_of_day;
pub mod toolbar;
pub mod ui_settings;
pub mod weather;
