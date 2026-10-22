//! The per-frame hand-over from the game world to the render world
//! (reports/render-thread-2026-10-22/ARCHITECTURE.md, categories A/B).
use crate::render_thread::ExtractSet;
use country_core::{
    game_config::GameConfig,
    render::{frame_constants::FrameConstants, render_config::FrameRenderConfig},
    resources::{
        game_clock::GameClock,
        glade_settings::GladeSettings,
        gui::{DebugGui, EscMenuState},
        image_formation::ImageFormationConfigs,
        session::ActiveSession,
        window::WindowSize,
    },
    systems::{
        camera_rig::main_camera::MainCamera,
        mode_manager::AppMode,
        render_loop::{painterly::PainterlyState, vermeer::VermeerDebugControl},
    },
};

/// Category A (small per-frame state): copied when it changed.
///
/// Still to migrate before the render world is complete: per-frame state that
/// is not `Clone` yet (LutGalleryState, VermeerState, WallBrickUploadStatus,
/// ActiveIccProfile, ShaderRefactoringState) and every category-B library
/// (AssetTexture/Ssbo/VAO/PlacementData libraries, GpuWorld, the path/garden/
/// water rasters, LUTs, text renderer), which move to render-side ownership
/// behind command queues registered with `ExtractSet::apply_queue`.
pub fn production_extract() -> ExtractSet {
    let mut extract = ExtractSet::default();
    extract
        .copy_changed::<GameConfig>()
        .copy_changed::<FrameRenderConfig>()
        .copy_changed::<EscMenuState>()
        .copy::<FrameConstants>()
        .copy_changed::<WindowSize>()
        .copy_changed::<ImageFormationConfigs>()
        .copy_changed::<GladeSettings>()
        .copy_changed::<VermeerDebugControl>()
        .copy_changed::<PainterlyState>()
        .copy::<MainCamera>()
        .copy_changed::<AppMode>()
        .copy_changed::<ActiveSession>()
        .copy::<GameClock>()
        .copy_changed::<DebugGui>()
        .copy::<bevy_time::Time>();
    extract
}
