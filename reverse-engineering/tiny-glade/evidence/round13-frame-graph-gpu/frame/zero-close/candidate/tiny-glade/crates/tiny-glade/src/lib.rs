//! Development integration facade for recovered game modules.
pub use country_core;
pub use rhapsody;
pub use system_roof as roof;
pub use tg_history as history;
pub use tg_terrain as terrain;
pub use utils;
pub mod raster_host;
pub mod render_host;
pub mod compute_frame;
pub mod render_bridge;
pub mod frame_world;
pub mod frame_setup;
pub mod egui_gpu;
pub mod effects_host;
pub mod production_renderer;
pub mod mei_host;
pub mod menu_background_host;
pub mod loading_graph;
pub mod scene_compute;
pub mod scene_raster;

// B `crates/tiny-glade/src` 启动/主循环/配置/日志部分（restorer-tinyglade-A）。
// `main` 在 B 中是模块（`tiny_glade::main::*`），Cargo.toml 已设 `autobins = false`。
pub mod app_config;
pub mod app_systems;
#[cfg(target_os = "windows")]
pub mod audio_host;
pub mod decorator_plugin;
pub mod event_loop;
pub mod fps_limiter;
pub mod game_loop;
pub mod gui;
pub mod icc;
pub mod load_language_library;
pub mod logging;
#[allow(special_module_name)] // B `src/main.rs` 是模块而非 bin target（autobins=false）
pub mod main;
pub mod render_loop;
pub mod run_game;
pub(crate) mod shutdown_runtime;
pub mod setup;

// B `crates/tiny-glade/src` 窗口/渲染 glue、崩溃上报、GUI 面板、截图与杂项
// （restorer-tinyglade-B）。
pub mod aftermath;
pub mod crash_pickup;
pub mod dlss_plugin;
pub mod egui_backend;
pub mod fs_util;
pub mod hang_watchdog;
pub mod integrity_report_main;
pub mod panic_reporter;
pub mod rollercoaster;
pub mod screenshot;
pub mod theme_fetcher_plugin;
pub mod toolbar_plugin;
pub mod vram;
pub mod vulkan_config_main;
pub mod winit_bevy_glue;

pub mod world_lighting;
pub mod world_postprocess;
pub mod world_terrain;
pub mod world_geometry;
pub mod world_walls;
pub mod world_surface;
pub mod world_graph;
pub mod world_vegetation;

pub mod gameplay_startup;
