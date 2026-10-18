//! B `crates/tiny-glade/src/render_loop.rs`（整帧 render / render_world / 截图错误）。
//!
//! 这三个函数在 PDB 中属于 **tiny_glade** 二进制 crate：
//! - `tiny_glade::render_loop::render` `0x1404e8010`（10,114 B）
//! - `tiny_glade::render_loop::render_world` `0x1404ec110`（4,173 B）
//! - `tiny_glade::render_loop::handle_screenshot_error` `0x1404ebfb0`（303 B）
//!
//! 由于 country-core 的 pass 入口（`compute_passes::*` / `draw_passes::*`）只被
//! `render_world` 调用，逐语句重建立项时已落在
//! `country_core::systems::render_loop::{render_frame, render_world}`：
//! - `render_frame.rs`：外层帧编排（资源获取 → 绘制列表 → 渲染图编译，含 OOM
//!   降分辨率重试 → 执行 → swapchain 获取/展示/present → 帧统计与 GPU 队列
//!   修剪）+ `handle_screenshot_error`；常量/分支/日志文案/panic 文案均有
//!   地址注释与测试。
//! - `render_world.rs`：54 个 pass 的调用顺序、条件分支与资源查找 panic。
//!
//! 本文件是 tiny-glade 的**归属导出面**：保持原 crate 路径可调用，逻辑直接
//! 复用 country-core 的编排实现。宿主设备（rhapsody
//! `RenderGraphState`/`GraphExecutor`/swapchain）通过 `RenderHost` 缝注入，
//! 与 `render_host.rs` 的设备接缝一致。
//!
//! 2026-10-18 已重新读取 B 原程序的 render、三条图编译闭包与原始指令；
//! 编排 trait 或单元测试通过不代表生产图、资源 producer 与 present 已闭合。

pub use country_core::systems::render_loop::render_frame::{
    handle_screenshot_error, render, FramePacer, RenderHost, RenderMode, Ring4, SwapchainInfo,
};
pub use country_core::systems::render_loop::render_world::{
    render_world, FramePasses, RenderWorldArgs, RenderWorldOutputs, TextureDesc,
};

/// 原 `render_loop` 的帧外层/log target（O：`tiny_glade::render_loop`）。
pub const LOG_TARGET: &str = "tiny_glade::render_loop";
